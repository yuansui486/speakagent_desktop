

const en = {
  searchPlaceholder: "Search the transcript…",
  matches: (n: number) => `${n} matches`,
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

type T = typeof en;
const zh: T = {
  searchPlaceholder: "搜索转写文本…", matches: (n) => `${n} 处匹配`, notFound: "未找到", noSpeech: "（未识别到语音）",
  exportFallbackName: "总结", protoBusiness: "商务会议", protoInterview: "面试", clickReplica: "点击右侧发言，播放器将跳转到对应位置。",
  retranscribeTitle: "重新转写此录音（例如更换模型）", transcribing: "正在转写…", retranscribe: "重新转写",
};

export const resultView = { en, zh };
