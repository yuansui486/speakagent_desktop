// «Итоги встречи»: вкладки над результатом (Текст · Саммари · Протокол · Задачи),
// панель одного артефакта (генерация локальным LLM), докачка помощника и экспорт.
// Общие атомы (Копировать/Скопировано!/Остановить) берём из common.

const en = {
  tabText: "Text",
  tabSummary: "Summary",
  tabProtocol: "Minutes",
  tabTodo: "Tasks",

  kindSummary: "Summary",
  kindBusiness: "Minutes",
  kindInterview: "Interview minutes",
  kindTodo: "Tasks",

  styleBusiness: "Business meeting",
  styleInterview: "Interview",

  reading: (done: number, total: number) => `Reading the recording… ${done} of ${total}`,
  writing: "Writing…",
  starting: "Starting the assistant…",
  readingHint: "It's a long recording — I'll read it in parts first.",
  writingHint: "This takes from a minute to a few — feel free to switch tabs.",

  regenerate: "Regenerate",

  tryAgain: "Try again",
  compose: "Compose",
  etaLocal: (eta: string) => `On this computer — about ${eta}. Everything runs locally, no internet.`,
  minRange: (lo: number, hi: number) => `${lo}–${hi} min`,

  copyPrompt: "Copy the request for an AI",
  copyPromptHint:
    "A quick alternative: paste it into ChatGPT, Claude, or another AI chat — it'll build the summary from your transcript.",

  helperNeeded:
    "Summaries need an assistant — it downloads once (~2.4 GB) and then works fully offline.",
  downloadHelper: "Download the assistant",

  download: "Download",
  exportTxt: "Text (.txt)",
};

type T = typeof en;

const zh: T = {
  tabText: "文本", tabSummary: "摘要", tabProtocol: "纪要", tabTodo: "任务",
  kindSummary: "摘要", kindBusiness: "会议纪要", kindInterview: "面试纪要", kindTodo: "任务",
  styleBusiness: "商务会议", styleInterview: "面试",
  reading: (done, total) => `正在读取录音… ${done}/${total}`, writing: "正在生成…", starting: "正在启动助手…",
  readingHint: "录音较长，将先分段读取。", writingHint: "生成需要一到几分钟，期间可以切换到其他页面。",
  regenerate: "重新生成", tryAgain: "重试", compose: "生成",
  etaLocal: (eta) => `在本机预计需要 ${eta}，全部离线处理，无需联网。`, minRange: (lo, hi) => `${lo}–${hi} 分钟`,
  copyPrompt: "复制 AI 提示词", copyPromptHint: "也可以将复制的内容粘贴到 ChatGPT、Claude 或其他 AI 对话中，依据转写文本生成总结。",
  helperNeeded: "生成总结需要下载助手（约 2.4 GB），只需下载一次，之后即可完全离线使用。",
  downloadHelper: "下载助手", download: "下载", exportTxt: "文本（.txt）",
};

export const meeting = { en, zh };
