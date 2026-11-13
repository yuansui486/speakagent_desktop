

const en = {
  download: "Download",
  txt: "Text (.txt)",
};

type T = typeof en;
const zh: T = {
  download: "下载", txt: "文本（.txt）",
};

export const exportMenu = { en, zh };
