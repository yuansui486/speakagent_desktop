//! Local, model-specific speaker enrollment and conservative open-set matching.
//! Similarity is a cosine score, never a probability or proof of identity.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Read;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sherpa_onnx::{SpeakerEmbeddingExtractor, SpeakerEmbeddingExtractorConfig,
    SileroVadModelConfig, VadModelConfig, VoiceActivityDetector};

use super::{diarize::Segment, models};

const SR: usize = 16_000;
const MAX_INPUT: usize = 60 * SR;
const MAX_SPEAKER: usize = 30 * SR;
const MIN_SPEECH: usize = 3 * SR;
const WINDOW: usize = 4 * SR;
// Engineering starting points, independently configurable/calibratable per model.
const MATCH_THRESHOLD: f32 = 0.60;
const MATCH_MARGIN: f32 = 0.10;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceProfile {
    pub id: String,
    pub name: String,
    pub samples: Vec<VoiceSampleInfo>,
    pub created_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceSampleInfo {
    pub id: String,
    pub model_id: String,
    pub speech_seconds: f32,
    pub created_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceMatch {
    pub speaker: i32,
    pub profile_id: Option<String>,
    pub name: Option<String>,
    pub score: Option<f32>,
    pub reason: String,
}

fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64
}

fn new_id(prefix: &str) -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("{prefix}-{nanos:x}-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed))
}

fn err(zh: &'static str, en: &'static str) -> String {
    super::localization::text(en, zh).into()
}

fn connection() -> Result<Connection, String> {
    let connection = Connection::open(super::store::data_dir().join("speakagent.db"))
        .map_err(|e| e.to_string())?;
    init(&connection)?;
    Ok(connection)
}

fn init(c: &Connection) -> Result<(), String> {
    c.busy_timeout(Duration::from_secs(5)).map_err(|e| e.to_string())?;
    c.execute_batch("PRAGMA foreign_keys=ON;
        CREATE TABLE IF NOT EXISTS voice_profiles (
            id TEXT PRIMARY KEY, name TEXT NOT NULL, created_at INTEGER NOT NULL);
        CREATE TABLE IF NOT EXISTS voice_samples (
            id TEXT PRIMARY KEY, profile_id TEXT NOT NULL REFERENCES voice_profiles(id) ON DELETE CASCADE,
            model_id TEXT NOT NULL, model_revision TEXT NOT NULL, dimension INTEGER NOT NULL,
            embedding TEXT NOT NULL, speech_seconds REAL NOT NULL, created_at INTEGER NOT NULL);
        CREATE INDEX IF NOT EXISTS voice_samples_profile ON voice_samples(profile_id);
        CREATE TABLE IF NOT EXISTS voice_job_matches (
            job_id TEXT NOT NULL, speaker INTEGER NOT NULL, profile_id TEXT, name TEXT,
            score REAL, reason TEXT NOT NULL, PRIMARY KEY(job_id,speaker));")
        .map_err(|e| e.to_string())
}

fn list_with(c: &Connection) -> Result<Vec<VoiceProfile>, String> {
    let mut query = c.prepare("SELECT id,name,created_at FROM voice_profiles ORDER BY created_at,id")
        .map_err(|e| e.to_string())?;
    let profiles = query.query_map([], |r| Ok(VoiceProfile {
        id: r.get(0)?, name: r.get(1)?, created_at: r.get(2)?, samples: Vec::new(),
    })).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for profile in profiles {
        let mut profile = profile.map_err(|e| e.to_string())?;
        let mut samples = c.prepare("SELECT id,model_id,speech_seconds,created_at FROM voice_samples
            WHERE profile_id=?1 ORDER BY created_at,id").map_err(|e| e.to_string())?;
        profile.samples = samples.query_map([&profile.id], |r| Ok(VoiceSampleInfo {
            id: r.get(0)?, model_id: r.get(1)?, speech_seconds: r.get(2)?, created_at: r.get(3)?,
        })).map_err(|e| e.to_string())?.collect::<Result<_, _>>().map_err(|e| e.to_string())?;
        out.push(profile);
    }
    Ok(out)
}

pub fn list_profiles() -> Result<Vec<VoiceProfile>, String> { list_with(&connection()?) }

fn valid_name(name: &str) -> Result<&str, String> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 100 || name.chars().any(char::is_control) {
        return Err(err("请输入 1–100 个字符的姓名，不能包含控制字符。", "Enter a name of 1–100 characters without control characters."));
    }
    Ok(name)
}

pub fn rename_profile(id: &str, name: &str) -> Result<(), String> {
    let changed = connection()?.execute("UPDATE voice_profiles SET name=?1 WHERE id=?2",
        params![valid_name(name)?, id]).map_err(|e| e.to_string())?;
    if changed == 0 { return Err(err("找不到此声纹档案。", "Voice profile not found.")); }
    Ok(())
}

pub fn delete_profile(id: &str) -> Result<(), String> {
    connection()?.execute("DELETE FROM voice_profiles WHERE id=?1", [id]).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete_sample(id: &str) -> Result<(), String> {
    connection()?.execute("DELETE FROM voice_samples WHERE id=?1", [id]).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate_model(id: &str) -> Result<(), String> {
    if matches!(id, "diar-emb" | "diar-emb-eres2netv2") { Ok(()) }
    else { Err(err("不支持的声纹模型。", "Unsupported voiceprint model.")) }
}

fn cancelled(cancel: &AtomicBool) -> Result<(), String> {
    if cancel.load(Ordering::Relaxed) { Err("cancelled".into()) } else { Ok(()) }
}

fn normalized(mut vector: Vec<f32>) -> Option<Vec<f32>> {
    if vector.is_empty() || vector.iter().any(|v| !v.is_finite()) { return None; }
    let norm = vector.iter().map(|v| (*v as f64).powi(2)).sum::<f64>().sqrt();
    if norm <= 1e-12 { return None; }
    for value in &mut vector { *value = (*value as f64 / norm) as f32; }
    Some(vector)
}

fn cosine(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.len() != b.len() || a.is_empty() { return None; }
    let value = a.iter().zip(b).map(|(a,b)| a * b).sum::<f32>();
    value.is_finite().then_some(value.clamp(-1.0, 1.0))
}

fn centroid(vectors: &[Vec<f32>]) -> Option<Vec<f32>> {
    let dim = vectors.first()?.len();
    if vectors.iter().any(|vector| vector.len() != dim) { return None; }
    let mut sum = vec![0.0; dim];
    for vector in vectors { for (total, value) in sum.iter_mut().zip(vector) { *total += value; } }
    normalized(sum)
}

/// Content revision avoids comparing embeddings after an ONNX file replacement.
/// FNV is a local compatibility fingerprint, not a download authenticity hash.
fn revision(path: &str) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hash = 0xcbf29ce484222325u64;
    let mut buf = [0; 64 * 1024];
    loop {
        let count = file.read(&mut buf).map_err(|e| e.to_string())?;
        if count == 0 { break; }
        for byte in &buf[..count] { hash ^= *byte as u64; hash = hash.wrapping_mul(0x100000001b3); }
    }
    Ok(format!("fnv1a64-{hash:016x}"))
}

fn extractor(model_id: &str, threads: i32) -> Result<(SpeakerEmbeddingExtractor, String), String> {
    validate_model(model_id)?;
    models::ensure(model_id, |_, _| {})?;
    extractor_prepared(model_id, threads)
}

// Matching runs in an inference-only child. Preparation belongs to its parent;
// never start a second, independently locked download from this process.
fn extractor_prepared(model_id: &str, threads: i32) -> Result<(SpeakerEmbeddingExtractor, String), String> {
    validate_model(model_id)?;
    let path = models::embedding_model_file(model_id).ok_or_else(|| err("声纹模型文件不可用。", "Voiceprint model is unavailable."))?;
    let revision = revision(&path)?;
    let extractor = SpeakerEmbeddingExtractor::create(&SpeakerEmbeddingExtractorConfig {
        model: Some(path), num_threads: threads.clamp(1, 16), provider: Some("cpu".into()), debug: false,
    }).ok_or_else(|| err("无法加载声纹模型。", "Could not load voiceprint model."))?;
    Ok((extractor, revision))
}

fn speech_only(samples: &[f32]) -> Result<Vec<f32>, String> {
    models::ensure("vad", |_, _| {})?;
    let path = models::vad().ok_or_else(|| err("语音检测模型不可用。", "Speech detection model is unavailable."))?;
    let config = VadModelConfig {
        silero_vad: SileroVadModelConfig { model: Some(path), threshold: 0.5,
            min_silence_duration: 0.4, min_speech_duration: 0.25, max_speech_duration: 10.0,
            window_size: 512 },
        sample_rate: SR as i32, num_threads: 1, provider: Some("cpu".into()), debug: false,
        ..Default::default()
    };
    let vad = VoiceActivityDetector::create(&config, 65.0).ok_or_else(|| err("无法初始化语音检测。", "Could not initialize speech detection."))?;
    let mut out = Vec::new();
    let drain = |out: &mut Vec<f32>| {
        while let Some(segment) = vad.front() { out.extend_from_slice(segment.samples()); vad.pop(); }
    };
    for chunk in samples.chunks(512) { vad.accept_waveform(chunk); drain(&mut out); }
    vad.flush();
    drain(&mut out);
    Ok(out)
}

fn embeddings(extractor: &SpeakerEmbeddingExtractor, samples: &[f32], cancel: &AtomicBool) -> Result<Vec<Vec<f32>>, String> {
    let mut out = Vec::new();
    for window in samples.chunks(WINDOW) {
        cancelled(cancel)?;
        // A very short tail is noisy and should not outweigh full speech windows.
        if window.len() < SR { continue; }
        let stream = extractor.create_stream().ok_or_else(|| err("无法创建声纹提取流。", "Voice embedding stream unavailable."))?;
        stream.accept_waveform(SR as i32, window);
        stream.input_finished();
        if !extractor.is_ready(&stream) { continue; }
        if let Some(vector) = extractor.compute(&stream).and_then(normalized) { out.push(vector); }
    }
    if out.is_empty() { return Err(err("有效语音不足，无法提取声纹。", "Insufficient usable speech for a voiceprint.")); }
    Ok(out)
}

pub fn enroll_file(name: &str, profile_id: Option<&str>, path: &str, model_id: &str, num_threads: i32) -> Result<VoiceProfile, String> {
    valid_name(name)?;
    validate_model(model_id)?;
    // Decode one extra sample to distinguish an overlong file from exactly 60 s.
    let samples = super::decode::decode_to_16k_mono_max(path, Some(60.01))?;
    enroll_samples(name, profile_id, &samples, model_id, num_threads)
}

pub fn enroll_samples(name: &str, profile_id: Option<&str>, samples: &[f32], model_id: &str, num_threads: i32) -> Result<VoiceProfile, String> {
    let name = valid_name(name)?;
    validate_model(model_id)?;
    if samples.len() > MAX_INPUT || samples.iter().any(|value| !value.is_finite()) {
        return Err(err("录入音频最多 60 秒，且必须为有效的 16 kHz 单声道音频。", "Enrollment requires valid 16 kHz mono audio of at most 60 seconds."));
    }
    let c = connection()?;
    if let Some(id) = profile_id {
        if !c.query_row("SELECT EXISTS(SELECT 1 FROM voice_profiles WHERE id=?1)", [id], |r| r.get::<_, bool>(0)).map_err(|e| e.to_string())? {
            return Err(err("找不到此声纹档案。", "Voice profile not found."));
        }
    }
    let speech = speech_only(samples)?;
    if speech.len() < MIN_SPEECH {
        return Err(err("至少需要 3 秒有效单人语音，建议录入 15–30 秒。", "At least 3 seconds of single-speaker speech is required; 15–30 seconds is recommended."));
    }
    let (extractor, revision) = extractor(model_id, num_threads)?;
    let vectors = embeddings(&extractor, &speech[..speech.len().min(MAX_SPEAKER)], &AtomicBool::new(false))?;
    let vector = centroid(&vectors).ok_or_else(|| err("声纹向量无效。", "Invalid voice embedding."))?;
    // Reject clearly inconsistent windows. This is a quality guard, not a
    // guarantee that a recording contains exactly one person.
    if vectors.iter().any(|v| cosine(v, &vector).unwrap_or(-1.0) < 0.5) {
        return Err(err("语音片段声纹不一致，请使用同一人的清晰单人录音。", "Speech windows are inconsistent; use a clear recording of one person."));
    }
    let profile = profile_id.map(str::to_owned).unwrap_or_else(|| new_id("voice"));
    insert_sample(&c, &profile, profile_id.is_none().then_some(name), model_id, &revision,
        &vector, speech.len().min(MAX_SPEAKER) as f32 / SR as f32)?;
    list_with(&c)?.into_iter().find(|p| p.id == profile).ok_or_else(|| err("录入后找不到声纹档案。", "Voice profile not found after enrollment."))
}

fn insert_sample(c: &Connection, profile: &str, new_name: Option<&str>, model_id: &str,
    revision: &str, vector: &[f32], speech_seconds: f32) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(|e| e.to_string())?;
    if let Some(name) = new_name {
        tx.execute("INSERT INTO voice_profiles(id,name,created_at) VALUES(?1,?2,?3)", params![profile,name,now()]).map_err(|e| e.to_string())?;
    }
    tx.execute("INSERT INTO voice_samples(id,profile_id,model_id,model_revision,dimension,embedding,speech_seconds,created_at)
        VALUES(?1,?2,?3,?4,?5,?6,?7,?8)", params![new_id("sample"),profile,model_id,revision,vector.len() as i64,
            serde_json::to_string(vector).map_err(|e| e.to_string())?,speech_seconds,now()]).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())
}

#[derive(Clone)]
struct Candidate { id: String, name: String, vector: Vec<f32> }

fn candidates(c: &Connection, model_id: &str, revision: &str, dim: usize) -> Result<Vec<Candidate>, String> {
    let mut query = c.prepare("SELECT p.id,p.name,s.embedding FROM voice_profiles p JOIN voice_samples s ON s.profile_id=p.id
        WHERE s.model_id=?1 AND s.model_revision=?2 AND s.dimension=?3 ORDER BY p.id,s.id").map_err(|e| e.to_string())?;
    let rows = query.query_map(params![model_id,revision,dim as i64], |r| Ok((r.get::<_,String>(0)?,r.get::<_,String>(1)?,r.get::<_,String>(2)?)))
        .map_err(|e| e.to_string())?;
    let mut grouped: BTreeMap<String,(String,Vec<Vec<f32>>)> = BTreeMap::new();
    for row in rows {
        let (id,name,raw) = row.map_err(|e| e.to_string())?;
        if let Some(vector) = serde_json::from_str::<Vec<f32>>(&raw).ok().and_then(normalized).filter(|v| v.len() == dim) {
            grouped.entry(id).or_insert_with(|| (name,Vec::new())).1.push(vector);
        }
    }
    Ok(grouped.into_iter().filter_map(|(id,(name,vectors))| centroid(&vectors).map(|vector| Candidate{id,name,vector})).collect())
}

fn unknown(speaker: i32, reason: &str, score: Option<f32>) -> VoiceMatch {
    VoiceMatch { speaker, profile_id: None, name: None, score, reason: reason.into() }
}

fn classify(speaker: i32, query: &[f32], candidates: &[Candidate]) -> VoiceMatch {
    let mut scores: Vec<_> = candidates.iter().filter_map(|candidate| cosine(query,&candidate.vector).map(|score| (score,candidate))).collect();
    scores.sort_by(|a,b| b.0.total_cmp(&a.0).then_with(|| a.1.id.cmp(&b.1.id)));
    let Some((score,best)) = scores.first() else { return unknown(speaker,"model_mismatch",None); };
    if *score < MATCH_THRESHOLD { return unknown(speaker,"unknown",Some(*score)); }
    if scores.get(1).is_some_and(|runner_up| score - runner_up.0 < MATCH_MARGIN) {
        return unknown(speaker,"ambiguous",Some(*score));
    }
    VoiceMatch { speaker, profile_id: Some(best.id.clone()), name: Some(best.name.clone()), score: Some(*score), reason: "matched".into() }
}

/// Union same-speaker ranges, subtract every other speaker's ranges, and trim
/// boundaries. Never infer identity from mixed/overlapping dialogue audio.
fn clean_ranges(segs: &[Segment], raw_ids: &HashSet<i32>, sample_count: usize) -> Vec<(usize,usize)> {
    let seconds = sample_count as f32 / SR as f32;
    let valid = |s: &&Segment| s.start.is_finite() && s.end.is_finite() && s.end > s.start;
    let mut target: Vec<(f32,f32)> = segs.iter().filter(valid).filter(|s| raw_ids.contains(&s.speaker))
        .map(|s| (s.start.max(0.0),s.end.min(seconds))).filter(|(start,end)|end>start).collect();
    target.sort_by(|a,b| a.0.total_cmp(&b.0));
    let mut union: Vec<(f32,f32)> = Vec::new();
    for (start,end) in target {
        if let Some(last) = union.last_mut().filter(|last| start <= last.1) { last.1 = last.1.max(end); }
        else { union.push((start,end)); }
    }
    for other in segs.iter().filter(valid).filter(|s| !raw_ids.contains(&s.speaker)) {
        let (start,end) = ((other.start - 0.1).max(0.0),(other.end + 0.1).min(seconds));
        union = union.into_iter().flat_map(|(a,b)| {
            let mut remaining = Vec::new();
            if end <= a || start >= b { remaining.push((a,b)); }
            else { if a < start { remaining.push((a,start.min(b))); } if end < b { remaining.push((end.max(a),b)); } }
            remaining
        }).collect();
    }
    union.into_iter().filter_map(|(a,b)| {
        let start = ((a+0.1)*SR as f32).ceil() as usize;
        let end = ((b-0.1).max(0.0)*SR as f32).floor() as usize;
        (end > start && end-start >= SR/2).then_some((start.min(sample_count),end.min(sample_count)))
    }).collect()
}

pub fn match_speakers(samples: &[f32], segs: &[Segment], raw_to_display: &HashMap<i32,i32>, model_id: &str,
    cancel: &AtomicBool) -> Result<Vec<VoiceMatch>, String> {
    validate_model(model_id)?;
    cancelled(cancel)?;
    let mut display: BTreeMap<i32,HashSet<i32>> = BTreeMap::new();
    for (&raw,&speaker) in raw_to_display { if speaker > 0 { display.entry(speaker).or_default().insert(raw); } }
    let c = connection()?;
    let count: i64 = c.query_row("SELECT count(*) FROM voice_samples", [], |r| r.get(0)).map_err(|e| e.to_string())?;
    if count == 0 { return Ok(display.keys().map(|&speaker| unknown(speaker,"empty_library",None)).collect()); }
    if display.is_empty() { return Ok(Vec::new()); }
    if samples.iter().any(|sample| !sample.is_finite()) { return Err("Non-finite voice matching audio".into()); }
    let (extractor,revision) = extractor_prepared(model_id, 4)?;
    cancelled(cancel)?;
    let candidates = candidates(&c,model_id,&revision,extractor.dim() as usize)?;
    if candidates.is_empty() { return Ok(display.keys().map(|&speaker| unknown(speaker,"model_mismatch",None)).collect()); }
    let mut matches = Vec::new();
    for (speaker,raw_ids) in display {
        cancelled(cancel)?;
        let mut speech = Vec::new();
        for (start,end) in clean_ranges(segs,&raw_ids,samples.len()) {
            let end = end.min(start + MAX_SPEAKER.saturating_sub(speech.len()));
            speech.extend_from_slice(&samples[start..end]);
            if speech.len() >= MAX_SPEAKER { break; }
        }
        if speech.len() < MIN_SPEECH { matches.push(unknown(speaker,"short_speech",None)); continue; }
        let vectors = match embeddings(&extractor,&speech,cancel) {
            Ok(vectors) => vectors,
            Err(_) => {
                cancelled(cancel)?;
                matches.push(unknown(speaker,"extraction_failed",None));
                continue;
            }
        };
        let Some(query) = centroid(&vectors) else {
            matches.push(unknown(speaker,"extraction_failed",None));
            continue;
        };
        matches.push(classify(speaker,&query,&candidates));
    }
    Ok(matches)
}

fn save_matches_with(c: &Connection, job_id: &str, matches: &[VoiceMatch]) -> Result<(), String> {
    let tx = c.unchecked_transaction().map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM voice_job_matches WHERE job_id=?1",[job_id]).map_err(|e|e.to_string())?;
    for matched in matches {
        if matched.speaker <= 0 || matched.score.is_some_and(|score| !score.is_finite()) { return Err("Invalid voice match".into()); }
        tx.execute("INSERT INTO voice_job_matches(job_id,speaker,profile_id,name,score,reason) VALUES(?1,?2,?3,?4,?5,?6)",
            params![job_id,matched.speaker,matched.profile_id,matched.name,matched.score,matched.reason]).map_err(|e|e.to_string())?;
    }
    tx.commit().map_err(|e|e.to_string())
}

pub fn save_job_matches(job_id: &str, matches: &[VoiceMatch]) -> Result<(), String> {
    save_matches_with(&connection()?, job_id, matches)
}

fn job_matches_with(c: &Connection, job_id: &str) -> Result<Vec<VoiceMatch>, String> {
    let mut query = c.prepare("SELECT speaker,profile_id,name,score,reason FROM voice_job_matches WHERE job_id=?1 ORDER BY speaker").map_err(|e|e.to_string())?;
    let matches = query.query_map([job_id],|r|Ok(VoiceMatch {speaker:r.get(0)?,profile_id:r.get(1)?,name:r.get(2)?,score:r.get(3)?,reason:r.get(4)?}))
        .map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
    Ok(matches)
}

pub fn job_matches(job_id: &str) -> Result<Vec<VoiceMatch>, String> { job_matches_with(&connection()?,job_id) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_and_open_set_matching_are_conservative() {
        assert!(normalized(vec![0.0,0.0]).is_none());
        assert!(normalized(vec![f32::NAN]).is_none());
        let query=normalized(vec![1.0,0.0]).unwrap();
        let a=Candidate{id:"a".into(),name:"Alice".into(),vector:query.clone()};
        let b=Candidate{id:"b".into(),name:"Bob".into(),vector:normalized(vec![0.99,0.01]).unwrap()};
        assert_eq!(classify(1,&query,&[a.clone()]).reason,"matched");
        assert_eq!(classify(1,&query,&[a.clone(),b]).reason,"ambiguous");
        assert_eq!(classify(1,&[0.0,1.0],&[a]).reason,"unknown");
        assert!(cosine(&[1.0],&[1.0,0.0]).is_none());
    }

    #[test]
    fn ranges_remove_overlap_union_duplicates_and_clip_to_audio() {
        let segments=vec![Segment{start:0.0,end:10.0,speaker:8},Segment{start:0.0,end:10.0,speaker:8},
            Segment{start:4.0,end:6.0,speaker:2},Segment{start:f32::NAN,end:1.0,speaker:8}];
        let ranges=clean_ranges(&segments,&HashSet::from([8]),8*SR);
        assert_eq!(ranges.len(),2);
        assert!(ranges[0].1 <= (3.9*SR as f32) as usize);
        assert!(ranges[1].0 >= (6.1*SR as f32) as usize);
        assert!(ranges.last().unwrap().1 <= 8*SR);
    }

    #[test]
    fn sqlite_profiles_model_isolation_and_match_snapshots() {
        let c=Connection::open_in_memory().unwrap();init(&c).unwrap();
        insert_sample(&c,"p",Some("Alice"),"diar-emb","revision-a",&[1.0,0.0],5.0).unwrap();
        assert_eq!(list_with(&c).unwrap()[0].samples.len(),1);
        assert_eq!(candidates(&c,"diar-emb","revision-a",2).unwrap().len(),1);
        assert!(candidates(&c,"diar-emb-eres2netv2","revision-a",2).unwrap().is_empty());
        assert!(candidates(&c,"diar-emb","revision-b",2).unwrap().is_empty());
        assert!(candidates(&c,"diar-emb","revision-a",3).unwrap().is_empty());
        let matched=VoiceMatch{speaker:1,profile_id:Some("p".into()),name:Some("Alice".into()),score:Some(0.9),reason:"matched".into()};
        save_matches_with(&c,"job",&[matched]).unwrap();
        c.execute("UPDATE voice_profiles SET name='New name' WHERE id='p'",[]).unwrap();
        assert_eq!(job_matches_with(&c,"job").unwrap()[0].name.as_deref(),Some("Alice"));
        c.execute("DELETE FROM voice_profiles WHERE id='p'",[]).unwrap();
        assert!(list_with(&c).unwrap().is_empty());
        assert_eq!(c.query_row("SELECT count(*) FROM voice_samples",[],|r|r.get::<_,i64>(0)).unwrap(),0);
        assert_eq!(job_matches_with(&c,"job").unwrap().len(),1);
        save_matches_with(&c,"job",&[]).unwrap();assert!(job_matches_with(&c,"job").unwrap().is_empty());
    }
}
