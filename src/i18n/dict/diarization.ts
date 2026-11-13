const zh = {
  title: "说话人分离模型", hint: "分段模型使用 pyannote segmentation 3.0，可分别选择声纹模型。缺少组件时，会在转写前自动下载并显示进度。",
  quality: "ERes2NetV2 — 中文质量优先", qualityHint: "约 72 MB。中文声纹验证指标更好，CPU 处理较慢；中文界面默认选择。",
  fast: "CAM++ — 快速多语言", fastHint: "约 28 MB。适合中英文和日常快速处理。",
  note: "下一次转写生效。声纹验证指标不等同于完整会议分离准确率；如已知人数，请在转写页面指定说话人数。",
  installed: "已下载", automatic: "首次使用自动下载", saveFailed: "保存模型选择失败",
};
type T = typeof zh;
const en: T = {
  title: "Speaker diarization models", hint: "Uses pyannote segmentation 3.0 with a selectable speaker embedding model. Missing components download with progress before transcription.",
  quality: "ERes2NetV2 — Chinese quality", qualityHint: "About 72 MB. Better Chinese speaker verification metrics; slower on CPU. Default for Chinese UI.",
  fast: "CAM++ — fast multilingual", fastHint: "About 28 MB. For Chinese, English and fast everyday processing.",
  note: "Applies to the next transcription. Verification metrics do not measure full meeting diarization accuracy. Specify the speaker count when known.",
  installed: "Downloaded", automatic: "Downloads on first use", saveFailed: "Could not save model selection",
};

export const diarization = { zh, en };
