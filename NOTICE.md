# Third-party notices / Сторонние компоненты / 第三方组件

中文版本默认使用 SenseVoiceSmall；Qwen3-ASR 0.6B 为可选模型。模型权重与应用代码遵循各自许可，不能用应用的 Apache-2.0 许可代替模型许可。SenseVoice 权重适用 [FunASR 模型许可](https://huggingface.co/FunAudioLLM/SenseVoiceSmall/blob/main/LICENSE)，Qwen3-ASR 权重适用 Apache-2.0。原有俄语模型 GigaAM 的非商业限制仍适用于该模型。中文识别选型与 CUDA 来源见 [CHINESE_ASR.md](CHINESE_ASR.md)。

SpeakAgent Desktop
Copyright 2026 SpeakAgent Desktop

This product includes software developed by third parties, listed below.

SpeakAgent Desktop's own source code is licensed under the **Apache License 2.0**
(see [`LICENSE.md`](LICENSE.md)). The application builds on, bundles, or downloads at
runtime the third-party components listed below, each under **its own license**. This
file is provided for attribution and to make those obligations visible.

> **Собственный код** приложения — под Apache-2.0. Перечисленные ниже сторонние
> компоненты остаются под своими лицензиями. GigaAM для русской речи имеет
> некоммерческие ограничения; китайская конфигурация использует SenseVoice (см. ниже).

---

## ⚠️ Важно про модели / Model licensing

Модели **скачиваются при первом запуске** и **не входят** в исходный код или установщик
этого репозитория. Каждая модель подчиняется собственной лицензии.

**Модель русского языка по умолчанию — GigaAM — распространяется под некоммерческой
лицензией.** Код приложения открыт под Apache-2.0 и коммерческих ограничений не
накладывает, но **при выборе GigaAM** приложение пригодно
только для **некоммерческого** использования — из-за лицензии самой модели. Для
коммерческих сценариев выберите в настройках модель ASR с подходящей лицензией
(например, Parakeet — CC-BY-4.0, или Whisper — MIT).

---

## Библиотеки сборки / Build-time libraries

| Компонент | Роль | Лицензия |
|---|---|---|
| [Tauri 2](https://github.com/tauri-apps/tauri) | Оболочка приложения (Rust + WebView) | Apache-2.0 / MIT |
| [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) | ASR, диаризация, VAD | Apache-2.0 |
| [ONNX Runtime](https://github.com/microsoft/onnxruntime) | Инференс ONNX (статически) | MIT |
| [tract](https://github.com/sonos/tract) | Инференс ONNX для RUPunct (чистый Rust) | Apache-2.0 / MIT |
| [tokenizers](https://github.com/huggingface/tokenizers) | WordPiece-токенизатор | Apache-2.0 |
| [Symphonia](https://github.com/pdeljanov/Symphonia) | Декод аудио (чистый Rust) | MPL-2.0 |
| [rusqlite](https://github.com/rusqlite/rusqlite) / SQLite | Локальная БД истории | MIT / Public Domain |
| [genpdf](https://github.com/Techassi/genpdf-rs) | Генерация PDF | Apache-2.0 / MIT |
| [ureq](https://github.com/algesten/ureq) | HTTP-клиент к llama-server | Apache-2.0 / MIT |
| [sysinfo](https://github.com/GuillaumeGomez/sysinfo) | Детект железа | MIT |
| [cpal](https://github.com/RustAudio/cpal) · [rodio](https://github.com/RustAudio/rodio) · [enigo](https://github.com/enigo-rs/enigo) | Диктовка: микрофон · звук · авто-вставка | Apache-2.0 / MIT |
| [axum](https://github.com/tokio-rs/axum) · [tokio](https://github.com/tokio-rs/tokio) | Локальный MCP-сервер (HTTP) | MIT |
| [tar](https://github.com/alexcrichton/tar-rs) · [bzip2](https://github.com/trifectatechfoundation/bzip2-rs) · [flate2](https://github.com/rust-lang/flate2-rs) · [zip](https://github.com/zip-rs/zip2) | Распаковка загружаемых моделей | Apache-2.0 / MIT |
| [windows](https://github.com/microsoft/windows-rs) (Win) · [core-graphics/core-foundation](https://github.com/servo/core-foundation-rs) (macOS) | Системные API: хук клавиш, детект GPU | Apache-2.0 / MIT |
| React · Vite · Tailwind CSS · Zustand · TanStack Query · react-markdown · lucide-react | Фронтенд | MIT |

## Загружаемое в рантайме / Runtime-downloaded binaries

| Компонент | Роль | Лицензия |
|---|---|---|
| [llama.cpp / llama-server](https://github.com/ggml-org/llama.cpp) | Движок локальной LLM (sidecar) | MIT |
| [NVIDIA CUDA 11.8 / cuBLAS / cuFFT / cuDNN 8](https://developer.nvidia.com/cuda-toolkit) | Optional Windows Rust ASR worker runtime; extracted locally by PowerShell | NVIDIA redistribution / component licenses, copied into the runtime's `licenses` directory |
| [FFmpeg](https://ffmpeg.org/) (сборки [gyan.dev](https://www.gyan.dev/ffmpeg/builds/) для Windows, [martin-riedl.de](https://ffmpeg.martin-riedl.de/) для macOS) | Fallback-декод медиа — качается в рантайме, вызывается как отдельный процесс (не бандлится, не линкуется) | GPL |

## Модели / Models

| Модель | Роль | Лицензия |
|---|---|---|
| [SenseVoiceSmall](https://huggingface.co/FunAudioLLM/SenseVoiceSmall) | Chinese ASR default; INT8 sherpa-onnx conversion | [FunASR model license](https://huggingface.co/FunAudioLLM/SenseVoiceSmall/blob/main/LICENSE) |
| [Qwen3-ASR 0.6B](https://github.com/QwenLM/Qwen3-ASR) | Chinese / multilingual ASR; INT8 sherpa-onnx conversion | Apache-2.0 |
| [GigaAM v3 CTC-punct](https://github.com/salute-developers/GigaAM) | ASR (русский, по умолчанию) | **Некоммерческая** (см. карточку модели) |
| [Parakeet-TDT-0.6b-v3](https://huggingface.co/nvidia/parakeet-tdt-0.6b-v3) (NVIDIA NeMo) | ASR (25 языков) | CC-BY-4.0 |
| [Whisper small](https://github.com/openai/whisper) (OpenAI) | ASR (98 языков) | MIT |
| [Qwen3-4B-Instruct-2507](https://huggingface.co/Qwen/Qwen3-4B-Instruct-2507) / [Qwen3-1.7B](https://huggingface.co/Qwen/Qwen3-1.7B) | LLM «Итоги встречи» | Apache-2.0 |
| [pyannote segmentation 3.0](https://huggingface.co/pyannote/segmentation-3.0) | Диаризация (сегментация) | MIT |
| [CAM++ (3D-Speaker)](https://github.com/modelscope/3D-Speaker) | Диаризация (эмбеддинг голосов) | Apache-2.0 |
| [ERes2NetV2 Chinese (3D-Speaker)](https://modelscope.cn/models/iic/speech_eres2netv2_sv_zh-cn_16k-common/summary) | Chinese speaker embeddings, quality option | Apache-2.0 |
| [Silero VAD](https://github.com/snakers4/silero-vad) | Детектор речи | MIT |
| [RUPunct small](https://huggingface.co/ekhodzitsky/rupunct-small-onnx) | Пунктуация (дремлющий fallback) | MIT |

## Шрифты / Fonts

| Компонент | Роль | Лицензия |
|---|---|---|
| [DejaVu Sans](https://dejavu-fonts.github.io/) | Кириллический шрифт для PDF-экспорта (вшивается в исходники и в каждый экспортируемый PDF) | Bitstream Vera Fonts License (изменения DejaVu — public domain); полный текст: [`src-tauri/fonts/LICENSE`](src-tauri/fonts/LICENSE) |
| [Noto Sans SC](https://github.com/notofonts/noto-cjk) | Simplified Chinese PDF text and bold | SIL Open Font License 1.1; [`src-tauri/fonts/NotoSansSC-LICENSE.txt`](src-tauri/fonts/NotoSansSC-LICENSE.txt) |

---

Названия и лицензии приведены на основе карточек и репозиториев компонентов на момент
написания; при использовании сверяйтесь с актуальными условиями по ссылкам выше.
Licenses are stated per each component's model card / repository at the time of writing —
always verify against the upstream terms linked above.
