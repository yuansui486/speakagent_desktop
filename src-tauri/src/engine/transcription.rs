//! File transcription runs in a disposable Rust process so native inference can
//! be stopped at any point, including decoding, model loading and diarization.
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::atomic::AtomicBool;
use serde::{Deserialize, Serialize};
use super::{asr, asr_runtime, diarize, punct, voiceprints};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub path: String,
    pub files: asr::AsrFiles,
    pub embedding_id: String,
    pub diarization: Option<(String, String)>,
    pub vad: Option<String>,
    pub punctuation: Option<(String, String)>,
    pub num_threads: i32,
    pub num_speakers: i32,
    pub identify_speakers: bool,
    pub provider: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub stage: String,
    pub done: usize,
    pub total: usize,
    pub partial: String,
    pub audio_sec: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_names: Option<HashMap<i32, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turns: Option<Vec<diarize::TranscriptTurn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_warning: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionResult {
    pub text: String,
    pub speaker_names: HashMap<i32, String>,
    pub turns: Vec<diarize::TranscriptTurn>,
    pub identity_warning: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub result: TranscriptionResult,
    pub matches: Vec<voiceprints::VoiceMatch>,
    pub audio_sec: f64,
    pub overhead_sec: f64,
    pub processing_sec: f64,
    pub provider: Option<String>,
    pub fallback_reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum Event {
    Progress(Progress),
    Result(Output),
    Error(String),
}

fn process(request: Request, mut emit: impl FnMut(Progress)) -> Result<Output, String> {
    let cancel = AtomicBool::new(false);
    let started = std::time::Instant::now();
    let mut audio_sec = 0.0;
    let mut send = |stage: &str, done, total, partial: &str, duration| {
        emit(Progress { stage: stage.into(), done, total, partial: partial.into(),
            audio_sec: duration, speaker_names: None, turns: None, identity_warning: None });
    };
    send("decoding", 0, 0, "", audio_sec);
    let samples = super::decode::decode_to_16k_mono(&request.path)?;
    audio_sec = samples.len() as f32 / 16000.0;
    send("decoding", 0, 0, "", audio_sec);
    let asr = asr::Asr::load_with_provider(&request.files, request.num_threads, &request.provider)?;
    let overhead_sec = started.elapsed().as_secs_f64();
    let processing_started = std::time::Instant::now();
    let segs = if let Some((seg, emb)) = &request.diarization {
        send("diarizing", 0, 0, "", audio_sec);
        Some(diarize::diarize(seg, emb, &samples, request.num_threads, 0.8, request.num_speakers)?)
    } else { None };
    let mut words = asr.transcribe_words(&samples, request.vad.as_deref(), &cancel,
        |done, total, partial| send("transcribing", done, total, partial, audio_sec));
    if let Some((model, tokens)) = &request.punctuation {
        if let Some(p) = punct::Punctuator::load(model, tokens) {
            send("punctuating", 0, 0, &asr::join_text(&words), audio_sec);
            let texts: Vec<String> = words.iter().map(|w| w.text.clone()).collect();
            for (word, restored) in words.iter_mut().zip(p.restore(&texts)) { word.text = restored; }
        }
    }
    let mut result = TranscriptionResult::default();
    let mut matches = Vec::new();
    if let Some(segs) = &segs {
        let transcript = diarize::words_to_transcript(&words, segs);
        if request.identify_speakers {
            send("matching", 0, 0, &transcript.text, audio_sec);
            match voiceprints::match_speakers(&samples, segs, &transcript.raw_to_display,
                &request.embedding_id, &cancel) {
                Ok(found) => matches = found,
                Err(error) => result.identity_warning = Some(error),
            }
            for found in &matches {
                if let Some(name) = &found.name { result.speaker_names.insert(found.speaker, name.clone()); }
            }
        }
        result.text = transcript.text;
        result.turns = transcript.turns;
    } else { result.text = asr::join_text(&words); }
    let (provider, fallback_reason) = asr_runtime::last_snapshot();
    Ok(Output { result, matches, audio_sec: audio_sec as f64, overhead_sec,
        processing_sec: processing_started.elapsed().as_secs_f64(), provider, fallback_reason })
}

/// Invoked before Tauri initialization. Only JSON protocol messages go to stdout.
pub fn worker_main() {
    fn emit(event: Event) {
        let mut out = std::io::stdout().lock();
        if serde_json::to_writer(&mut out, &event).is_ok() { let _ = writeln!(out); let _ = out.flush(); }
    }
    let outcome = (|| {
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).map_err(|e| e.to_string())?;
        let request: Request = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        process(request, |progress| emit(Event::Progress(progress)))
    })();
    match outcome { Ok(output) => emit(Event::Result(output)), Err(error) => emit(Event::Error(error)) }
}
