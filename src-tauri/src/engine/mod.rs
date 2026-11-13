//! Ядро SpeakAgent (OS-агностичная логика).
//!
//! Фаза 1: `audio`, `decode` (аудио→16k mono), `asr` (GigaAM), `diarize`,
//! `store` (SQLite-история), `models` (каталог + загрузка + резолвинг).
//! Далее по SPEC: `vad`, `jobs`, `export`.

pub mod asr;
pub mod asr_config;
pub mod asr_runtime;
pub mod audio;
pub mod decode;
pub mod diarize;
pub mod dictation;
pub mod gpu;
pub mod llm;
pub mod models;
pub mod pdf;
pub mod prompts;
pub mod punct;
pub mod sound;
pub mod store;

pub mod localization;

pub mod text;
pub mod voiceprints;
pub mod voice_recording;
