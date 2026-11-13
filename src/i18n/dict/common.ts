// Общие атомы, переиспользуемые многими экранами. Каждый фрагмент словаря сам
// English defines the shared shape; Chinese implements the same keys.

const en = {
  locale: "en-US",

  cancel: "Cancel",
  delete: "Delete",
  copy: "Copy",
  copied: "Copied",
  copiedBang: "Copied!",
  save: "Save",
  saved: "Saved",
  stop: "Stop",
  back: "Back",
  clear: "Clear",

  notDownloaded: "not downloaded",
  clickToRename: "Click to rename",
  auto: "Auto",

  noResultsFor: (q: string) => `No results for “${q}”.`,
  speaker: (n: number) => `Speaker ${n}`,
  cores: (n: number) => `${n} ${n === 1 ? "core" : "cores"}`,
  ramGb: (gb: string | number) => `${gb} GB RAM`,
  mb: (n: number) => `${n} MB`,

  pickerAudioVideo: "Audio & video",
  pickerText: "Text",
};

export type CommonDict = typeof en;

const zh: CommonDict = {
  locale: "zh-CN",
  cancel: "取消", delete: "删除", copy: "复制", copied: "已复制", copiedBang: "已复制！",
  save: "保存", saved: "已保存", stop: "停止", back: "返回", clear: "清空",
  notDownloaded: "未下载", clickToRename: "点击重命名", auto: "自动",
  noResultsFor: (q) => `未找到与“${q}”相关的结果。`,
  speaker: (n) => `说话人 ${n}`, cores: (n) => `${n} 核`,
  ramGb: (gb) => `${gb} GB 内存`, mb: (n) => `${n} MB`,
  pickerAudioVideo: "音频和视频", pickerText: "文本",
};

export const common = { en, zh };
