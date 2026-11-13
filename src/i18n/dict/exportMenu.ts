const ru = {
  download: "Скачать",
  txt: "Текст (.txt)",
};
type T = typeof ru;
const en: T = {
  download: "Download",
  txt: "Text (.txt)",
};
const zh: T = {
  download: "下载", txt: "文本（.txt）",
};

export const exportMenu = { ru, en, zh };
