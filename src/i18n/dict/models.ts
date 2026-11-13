// Отображаемые имена/языки моделей по стабильному `id` из каталога Rust
// (engine/models.rs). UI берёт перевод отсюда с фолбэком на строку из Rust:
//   t.models[m.id]?.name ?? m.name   /   t.models[m.id]?.lang ?? m.lang
// Так каталог остаётся источником самих моделей, а витрина — двуязычная.
type ModelText = { name: string; lang: string; description?: string };

const ru: Record<string, ModelText> = {
  sensevoice: { name: "SenseVoiceSmall — быстрый китайский", lang: "Китайский, кантонский, английский, японский, корейский", description: "Для китайской речи и голосового ввода на CPU. Автоопределение языка и нормализация чисел. Временные метки китайских токенов; пунктуация может отсутствовать." },
  "qwen3-asr-06b": { name: "Qwen3-ASR 0.6B — качество", lang: "30 языков и 22 китайских диалекта", description: "Для сложной китайской речи и диалектов. Около 987 МБ загрузки; выше нагрузка на CPU. Временные метки — по сегментам." },
  gigaam: { name: "Русский (GigaAM)", lang: "русский" },
  parakeet: { name: "Мультиязычный (Parakeet)", lang: "25 европейских языков (без китайского)" },
  "whisper-small": { name: "Whisper small", lang: "98 языков" },
  "whisper-turbo": { name: "Whisper large-v3 turbo", lang: "99 языков" },
  "llm-qwen3-4b": { name: "Стандартная — лучшее качество", lang: "русский+" },
  "llm-qwen3-17b": { name: "Быстрая — для слабых компьютеров", lang: "русский+" },
};

const en: Record<string, ModelText> = {
  sensevoice: { name: "SenseVoiceSmall — fast Chinese", lang: "Chinese, Cantonese, English, Japanese, Korean", description: "Recommended for Chinese speech and dictation on CPU. Automatic language detection and number normalization. Chinese token timestamps; punctuation may be absent." },
  "qwen3-asr-06b": { name: "Qwen3-ASR 0.6B — quality", lang: "30 languages and 22 Chinese dialects", description: "For difficult Chinese audio and dialects. About 987 MB to download; more CPU intensive. Segment timestamps." },
  gigaam: { name: "Russian (GigaAM)", lang: "Russian" },
  parakeet: { name: "Multilingual (Parakeet)", lang: "25 European languages (no Chinese)" },
  "whisper-small": { name: "Whisper small", lang: "98 languages" },
  "whisper-turbo": { name: "Whisper large-v3 turbo", lang: "99 languages" },
  "llm-qwen3-4b": { name: "Standard — best quality", lang: "Russian+" },
  "llm-qwen3-17b": { name: "Fast — for low-end computers", lang: "Russian+" },
};

const zh: Record<string, ModelText> = {
  sensevoice: { name: "SenseVoiceSmall — 中文快速识别", lang: "普通话、粤语、英语、日语、韩语", description: "推荐用于日常中文和语音输入；本地运行，可选 CUDA，自动识别语言，支持数字规范化和中文字级时间定位。标点可能不完整。" },
  "qwen3-asr-06b": { name: "Qwen3-ASR 0.6B — 中文高质量", lang: "30 种语言、22 种中文方言", description: "适合复杂中文、中英混说和方言，可与 SenseVoice 对比选择。约需下载 987 MB，CPU 识别更慢；说话人时间定位精确到语音片段。" },
  gigaam: { name: "俄语（GigaAM）", lang: "俄语" },
  parakeet: { name: "多语言（Parakeet）", lang: "25 种欧洲语言（不含中文）" },
  "whisper-small": { name: "Whisper small", lang: "98 种语言" },
  "whisper-turbo": { name: "Whisper large-v3 turbo", lang: "99 种语言" },
  "llm-qwen3-4b": { name: "标准 — 更高质量", lang: "俄语等多种语言" },
  "llm-qwen3-17b": { name: "快速 — 适合低配置电脑", lang: "俄语等多种语言" },
};

export const models = { ru, en, zh };
