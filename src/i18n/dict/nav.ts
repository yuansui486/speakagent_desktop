

const en = {
  voiceLibrary: "Voice library",
  home: "Home",
  transcribe: "Transcription",
  dictation: "Dictation",
  mcp: "MCP Server",
  settings: "Settings",
  about: "About",

  brandSubtitle: "speech transcription",
  worksOffline: "Works offline",
};

type T = typeof en;
const zh: T = {
  voiceLibrary: "声纹库",
  home: "首页", transcribe: "语音转写", dictation: "语音输入", mcp: "MCP 服务器", settings: "设置", about: "关于",
  brandSubtitle: "语音转文字", worksOffline: "离线运行",
};

export const nav = { en, zh };
