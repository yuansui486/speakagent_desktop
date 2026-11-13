//! ASR через sherpa-onnx. Поддержка GigaAM (CTC), Parakeet (transducer), Whisper.
//! Нарезка длинного аудио — по VAD (Silero), чтобы границы падали в тишину, а не
//! посреди слова (иначе «мусор» на стыках). Если модель VAD недоступна — слепые окна.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use sherpa_onnx::{OfflineRecognizer, SileroVadModelConfig, VadModelConfig, VoiceActivityDetector};

const SR: i32 = 16000;
const CHUNK_SEC: usize = 20; // слепой фолбэк, если нет VAD
const VAD_WINDOW: usize = 512; // окно Silero VAD (samples)
const VAD_MAX_SPEECH_SEC: f32 = 20.0; // не даём offline-CTC слишком длинный кусок

pub use super::asr_config::{AsrFiles, Engine};

/// Слово с временем начала и конца (сек от начала файла) — для привязки к спикерам.
pub struct Word {
    pub text: String,
    pub start: f32,
    pub end: f32,
}

/// Распознаватель с загруженной моделью. Создаётся один раз, переиспользуется.
pub struct Asr {
    backend: Mutex<Backend>,
    files: AsrFiles,
    threads: i32,
}

enum Backend {
    Cpu(OfflineRecognizer),
    Cuda(super::asr_runtime::CudaWorker),
    Failed,
}

impl Asr {
    pub fn load(f: &AsrFiles, num_threads: i32) -> Result<Self, String> {
        match super::asr_runtime::try_worker(f, num_threads) {
            Ok(Some(worker)) => return Ok(Self { backend: Mutex::new(Backend::Cuda(worker)), files: f.clone(), threads: num_threads }),
            Ok(None) => super::asr_runtime::record("cpu", None),
            Err(reason) => super::asr_runtime::record("cpu", Some(reason)),
        }
        let config = f.config(num_threads, "cpu");
        let rec = OfflineRecognizer::create(&config)
            .ok_or(crate::engine::localization::error("failed to create the recognizer (check model files)"))?;
        Ok(Self {
            backend: Mutex::new(Backend::Cpu(rec)),
            files: f.clone(),
            threads: num_threads,
        })
    }

    /// Сплошной текст. Внутри — тот же проход со словами, что и `transcribe_words`.
    pub fn transcribe(
        &self,
        samples: &[f32],
        vad_model: Option<&str>,
        cancel: &AtomicBool,
        on_chunk: impl FnMut(usize, usize, &str),
    ) -> String {
        let words = self.transcribe_words(samples, vad_model, cancel, on_chunk);
        join_text(&words)
    }

    /// Файл целиком → слова с таймкодами. VAD-нарезка (если есть модель) или слепые окна.
    /// `on_chunk(done, total, partial)` — прогресс (в семплах).
    pub fn transcribe_words(
        &self,
        samples: &[f32],
        vad_model: Option<&str>,
        cancel: &AtomicBool,
        mut on_chunk: impl FnMut(usize, usize, &str),
    ) -> Vec<Word> {
        let mut words: Vec<Word> = Vec::new();

        // ── путь 1: нарезка по VAD (границы в тишине) ──
        if let Some(vm) = vad_model {
            if let Some(vad) = self.make_vad(vm) {
                let total = samples.len();
                let mut i = 0usize;
                while i < total {
                    if cancel.load(Ordering::Relaxed) {
                        finalize_word_ends(&mut words);
                        return words;
                    }
                    let end = (i + VAD_WINDOW).min(total);
                    vad.accept_waveform(&samples[i..end]);
                    i = end;
                    while let Some(seg) = vad.front() {
                        let off = seg.start() as f32 / SR as f32;
                        self.decode_segment(seg.samples(), off, &mut words);
                        vad.pop();
                        on_chunk(i, total, &join_text(&words));
                    }
                }
                vad.flush();
                while let Some(seg) = vad.front() {
                    let off = seg.start() as f32 / SR as f32;
                    self.decode_segment(seg.samples(), off, &mut words);
                    vad.pop();
                }
                finalize_word_ends(&mut words);
                on_chunk(total, total, &join_text(&words));
                return words;
            }
        }

        // ── путь 2 (фолбэк): слепые окна по 20с ──
        let win = CHUNK_SEC * SR as usize;
        let total = samples.len().div_ceil(win).max(1);
        let mut done = 0usize;
        let mut i = 0usize;
        while i < samples.len() {
            if cancel.load(Ordering::Relaxed) {
                break;
            }
            let end = (i + win).min(samples.len());
            let off = i as f32 / SR as f32;
            self.decode_segment(&samples[i..end], off, &mut words);
            i = end;
            done += 1;
            if done % 2 == 0 || done == total {
                on_chunk(done, total, &join_text(&words));
            }
        }
        finalize_word_ends(&mut words);
        words
    }

    /// Декод одного речевого куска (уже вырезанного) + добавление слов со сдвигом `offset`.
    fn decode_segment(&self, seg: &[f32], offset: f32, out: &mut Vec<Word>) {
        if seg.len() < (SR as usize) / 5 {
            return; // < 0.2с — пропускаем
        }
        if let Some(res) = self.decode(seg) {
            match res.timestamps {
                // Пословные таймкоды (GigaAM/Parakeet CTC/transducer) → точная привязка к спикерам.
                Some(times) if !times.is_empty() && times.len() == res.tokens.len() => {
                    push_words(&res.tokens, &times, offset, out);
                },
                // Whisper даёт текст, но timestamps=Some(пусто) → берём текст сегмента целиком
                // (диаризация огрубляется до границы VAD-сегмента — у whisper пословных нет).
                _ => {
                    let t = res.text.trim();
                    if !t.is_empty() {
                        out.push(Word {
                            text: t.to_string(),
                            start: offset,
                            end: offset,
                        });
                    }
                }
            }
        }
    }

    fn make_vad(&self, model: &str) -> Option<VoiceActivityDetector> {
        let cfg = VadModelConfig {
            silero_vad: SileroVadModelConfig {
                model: Some(model.to_string()),
                threshold: 0.5,
                min_silence_duration: 0.5,
                min_speech_duration: 0.25,
                window_size: VAD_WINDOW as i32,
                max_speech_duration: VAD_MAX_SPEECH_SEC,
            },
            sample_rate: SR,
            num_threads: self.threads,
            provider: Some("cpu".to_string()),
            debug: false,
            ..Default::default()
        };
        VoiceActivityDetector::create(&cfg, 30.0)
    }

    fn decode(&self, seg: &[f32]) -> Option<sherpa_onnx::OfflineRecognizerResult> {
        let mut backend = self.backend.lock().unwrap();
        if let Backend::Cuda(worker) = &mut *backend {
            match worker.decode(seg) {
                Ok(result) => { super::asr_runtime::record("cuda", None); return Some(result); }
                Err(error) => {
                    super::asr_runtime::record("cpu", Some(error));
                    // Release the GPU model and kill the worker before allocating
                    // a second copy of the model in the main process.
                    *backend = Backend::Failed;
                    // Decode the SAME segment on CPU; no spoken text is discarded.
                    let config = self.files.config(self.threads, "cpu");
                    let cpu = OfflineRecognizer::create(&config)?;
                    *backend = Backend::Cpu(cpu);
                }
            }
        }
        let Backend::Cpu(rec) = &*backend else { return None; };
        let stream = rec.create_stream();
        stream.accept_waveform(SR, seg);
        rec.decode(&stream);
        stream.get_result()
    }
}

pub fn join_text(words: &[Word]) -> String {
    super::text::join(words.iter().map(|w| w.text.as_str()))
}

/// Гарантируем ненулевую ширину слова (для расчёта перекрытия при диаризации).
fn finalize_word_ends(words: &mut [Word]) {
    for w in words.iter_mut() {
        if w.end < w.start + 0.05 {
            w.end = w.start + 0.05;
        }
    }
}

/// Сборка слов из токенов. Поддержка char-моделей (пробел-токен) и BPE (префикс ▁).
/// `end` слова ≈ время последнего его токена (от `offset`).
fn push_words(tokens: &[String], times: &[f32], offset: f32, out: &mut Vec<Word>) {
    let mut cur = String::new();
    let mut start = offset;
    let mut cur_end = offset;
    for (tok, &t) in tokens.iter().zip(times.iter()) {
        let tt = offset + t;
        if tok.starts_with("<|") && tok.ends_with("|>") {
            continue; // SenseVoice language/emotion/event metadata is not spoken text.
        }
        let token = tok.trim_start_matches('\u{2581}');
        if token.chars().any(super::text::is_cjk) {
            if !cur.is_empty() {
                out.push(Word { text: std::mem::take(&mut cur), start, end: cur_end.max(start) });
            }
            out.push(Word { text: token.to_owned(), start: tt, end: tt });
        } else if tok == " " || tok.chars().all(char::is_whitespace) {
            // char-модель: пробел = граница слова
            if !cur.is_empty() {
                out.push(Word {
                    text: std::mem::take(&mut cur),
                    start,
                    end: cur_end.max(start),
                });
            }
        } else if let Some(rest) = tok.strip_prefix('\u{2581}') {
            // BPE: ▁ = начало нового слова
            if !cur.is_empty() {
                out.push(Word {
                    text: std::mem::take(&mut cur),
                    start,
                    end: cur_end.max(start),
                });
            }
            start = tt;
            cur_end = tt;
            cur.push_str(rest);
        } else {
            if cur.is_empty() {
                start = tt;
            }
            cur_end = tt;
            cur.push_str(tok);
        }
    }
    if !cur.is_empty() {
        out.push(Word {
            text: cur,
            start,
            end: cur_end.max(start),
        });
    }
}


#[cfg(test)]
mod chinese_tests {
    use super::*;

    #[test]
    fn chinese_tokens_keep_individual_timestamps_without_metadata() {
        let tokens: Vec<String> = ["<|zh|>", "<|NEUTRAL|>", "你", "好", "world", "▁again"].iter().map(|s| s.to_string()).collect();
        let mut words = Vec::new();
        push_words(&tokens, &[0.0, 0.0, 0.1, 0.2, 0.3, 0.4], 2.0, &mut words);
        finalize_word_ends(&mut words);
        assert_eq!(join_text(&words), "你好world again");
        assert_eq!(words.len(), 4);
        assert!((words[0].start - 2.1).abs() < 0.001);
        assert!((words[1].start - 2.2).abs() < 0.001);
        assert!(words.iter().all(|word| word.end > word.start));
    }
}
