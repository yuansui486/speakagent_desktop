const ru = {
  searchPlaceholder: "Поиск по тексту расшифровки…",
  matches: (n: number) => `совпадений: ${n}`,
  notFound: "не найдено",
  noSpeech: "(речь не распознана)",
  exportFallbackName: "Итоги",
  protoBusiness: "Деловая встреча",
  protoInterview: "Собеседование",
  clickReplica: "Кликните по реплике справа — плеер перемотается к ней.",
  retranscribeTitle: "Распознать эту запись заново (например, другой моделью)",
  transcribing: "Идёт расшифровка…",
  retranscribe: "Расшифровать заново",
};
type T = typeof ru;
const en: T = {
  searchPlaceholder: "Search the transcript…",
  matches: (n) => `${n} matches`,
  notFound: "not found",
  noSpeech: "(no speech detected)",
  exportFallbackName: "Summary",
  protoBusiness: "Business meeting",
  protoInterview: "Interview",
  clickReplica: "Click a line on the right — the player will jump to it.",
  retranscribeTitle: "Transcribe this recording again (e.g. with a different model)",
  transcribing: "Transcribing…",
  retranscribe: "Transcribe again",
};
const zh: T = {
  searchPlaceholder: "搜索转写文本…", matches: (n) => `${n} 处匹配`, notFound: "未找到", noSpeech: "（未识别到语音）",
  exportFallbackName: "总结", protoBusiness: "商务会议", protoInterview: "面试", clickReplica: "点击右侧发言，播放器将跳转到对应位置。",
  retranscribeTitle: "重新转写此录音（例如更换模型）", transcribing: "正在转写…", retranscribe: "重新转写",
};

export const resultView = { ru, en, zh };
