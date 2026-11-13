

const en = {
  welcome: "Welcome to SpeakAgent",
  sub: "Speech transcription, right on your computer",

  feat1: "Works offline",
  feat2: "Tells voices apart",
  feat3: "Nothing leaves your device",

  pickTitle: "Choose the recognition language",
  pickHint:
    "The model downloads once (size shown below) — plus an audio converter (~98 MB). Everything else is already built in. You can change it later in Settings.",

  downloadingModel: (pct: number) => `Downloading model… ${pct}%`,

  later: "Later",
  start: "Start",
  recommended: "Recommended",
};

type T = typeof en;

const zh: T = {
  welcome: "欢迎使用 SpeakAgent", sub: "在您的电脑上完成语音转写", feat1: "无需联网", feat2: "区分说话人", feat3: "数据保留在本机",
  pickTitle: "选择语音识别模型", pickHint: "日常中文推荐 SenseVoiceSmall；复杂中文和方言可选 Qwen3-ASR。首次下载模型后即可离线使用；另需音频转换器（约 98 MB）。可在设置中随时切换。",
  downloadingModel: (pct) => `正在下载模型… ${pct}%`, later: "稍后设置", start: "开始", recommended: "推荐",
};

export const onboarding = { en, zh };
