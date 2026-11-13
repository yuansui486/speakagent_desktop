

const en = {
  original: "Original",
  beautified: "Cleaned up",
  process: "Clean up text",
  reprocess: "Clean up again",
  processing: (done: number, total: number) =>
    total > 0 ? `Processing… ${done} of ${total}` : "Processing…",
  stop: "Stop",
  emptyTitle: "The cleaned-up text isn't ready yet",
  emptyHint:
    "The LLM will carefully fix spelling, punctuation, and obvious recognition errors while trying not to change the meaning or the wording. The original stays available via the toggle.",
  costNote:
    "Locally this may take a few minutes (feel free to switch tabs); via a cloud AI it is billed by your provider's tokens.",
  retry: "Try again",
  needEngine:
    "Connect an AI first: a local summary model or a cloud provider in Settings.",
  settingsTitle: "Text cleanup (beautifier)",
  settingsHint:
    "Optional LLM cleanup of the transcript: fixes spelling, punctuation, and obvious mishears while trying not to change the meaning. Off by default — it is slow on CPU.",
  enableLabel: "Enable text cleanup",
  enableHint: "Adds an “Original | Cleaned up” toggle on the recording screen.",
  autoLabel: "Clean up automatically after transcription",
  autoHint: "Careful: noticeably lengthens each file's processing (runs when the queue is empty).",
};

type T = typeof en;

const zh: T = {
  original: "原文", beautified: "整理后", process: "整理文本", reprocess: "重新整理",
  processing: (done, total) => total > 0 ? `正在处理… ${done}/${total}` : "正在处理…",
  stop: "停止", emptyTitle: "整理后的文本尚未生成",
  emptyHint: "大语言模型会谨慎修正拼写、标点和明显的识别错误，尽量保留原意和措辞。您可以随时切换查看原文。",
  costNote: "本地处理可能需要几分钟（期间可以切换到其他页面）；使用云端 AI 时，将按服务商的用量规则计费。",
  retry: "重试", needEngine: "请先在设置中配置 AI：下载本地总结模型或连接云端服务。",
  settingsTitle: "文本整理", settingsHint: "可选的大语言模型文本整理功能：修正拼写、标点和明显的识别错误，尽量保留原意。默认关闭，因为在处理器上运行较慢。",
  enableLabel: "启用文本整理", enableHint: "在录音页面显示“原文 | 整理后”切换选项。",
  autoLabel: "转写完成后自动整理", autoHint: "注意：这会明显延长每个文件的处理时间（在队列为空时执行）。",
};

export const beautify = { en, zh };
