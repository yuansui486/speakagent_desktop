//! Shared model configuration used by the CPU app and isolated Rust CUDA worker.
use sherpa_onnx::{
    OfflineNemoEncDecCtcModelConfig, OfflineQwen3ASRModelConfig, OfflineRecognizerConfig,
    OfflineSenseVoiceModelConfig, OfflineTransducerModelConfig, OfflineWhisperModelConfig,
};
/// Тип движка модели.
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Engine {
    NemoCtc,    // GigaAM
    Transducer, // Parakeet (nemo_transducer)
    Whisper,    // Whisper
    SenseVoice,
    Qwen3Asr,
}

/// Пути к файлам активной модели + её тип.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AsrFiles {
    pub engine: Engine,
    pub model: String,   // ctc: model; transducer/whisper: encoder
    pub decoder: String, // transducer/whisper: decoder; ctc: ""
    pub joiner: String,  // transducer: joiner; иначе ""
    pub tokens: String,
    pub language: String,  // Optional language hint; empty means automatic.
    pub frontend: String,  // Qwen3-ASR convolution frontend; otherwise empty.
    pub tokenizer: String, // Qwen3-ASR tokenizer directory; otherwise empty.
}

impl AsrFiles {
    pub fn config(&self, num_threads: i32, provider: &str) -> OfflineRecognizerConfig {
        let f = self;
        let mut config = OfflineRecognizerConfig::default();
        match f.engine {
            Engine::NemoCtc => {
                config.model_config.nemo_ctc = OfflineNemoEncDecCtcModelConfig {
                    model: Some(f.model.clone()),
                };
            }
            Engine::Transducer => {
                config.model_config.transducer = OfflineTransducerModelConfig {
                    encoder: Some(f.model.clone()),
                    decoder: Some(f.decoder.clone()),
                    joiner: Some(f.joiner.clone()),
                };
                config.model_config.model_type = Some("nemo_transducer".into());
            }
            Engine::SenseVoice => {
                config.model_config.sense_voice = OfflineSenseVoiceModelConfig {
                    model: Some(f.model.clone()),
                    language: Some(if f.language.is_empty() {
                        "auto".into()
                    } else {
                        f.language.clone()
                    }),
                    use_itn: true,
                };
            }
            Engine::Qwen3Asr => {
                config.feat_config.feature_dim = 128;
                config.model_config.qwen3_asr = OfflineQwen3ASRModelConfig {
                    conv_frontend: Some(f.frontend.clone()),
                    encoder: Some(f.model.clone()),
                    decoder: Some(f.decoder.clone()),
                    tokenizer: Some(f.tokenizer.clone()),
                    // 20 seconds of fast Chinese can exceed the SDK default of 128 tokens.
                    max_total_len: 1024,
                    max_new_tokens: 512,
                    ..Default::default()
                };
            }
            Engine::Whisper => {
                config.model_config.whisper = OfflineWhisperModelConfig {
                    encoder: Some(f.model.clone()),
                    decoder: Some(f.decoder.clone()),
                    language: if f.language.is_empty() {
                        None
                    } else {
                        Some(f.language.clone())
                    },
                    ..Default::default()
                };
            }
        }
        config.model_config.tokens = Some(f.tokens.clone());
        config.model_config.num_threads = num_threads;
        config.model_config.provider = Some(provider.into());

        config
    }
}
