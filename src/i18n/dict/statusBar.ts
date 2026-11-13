const ru = {
  summarizing: "составляю итоги…",
  summarizingReading: (done: number, total: number) =>
    `составляю итоги: читаю ${done}/${total}…`,
  remaining: (eta: string) => `осталось ${eta}`,
  switchLang: "Сменить язык интерфейса",
};
type T = typeof ru;
const en: T = {
  summarizing: "summarizing…",
  summarizingReading: (done, total) => `summarizing: reading ${done}/${total}…`,
  remaining: (eta) => `remaining ${eta}`,
  switchLang: "Switch interface language",
};
const zh: T = {
  summarizing: "正在生成总结…", summarizingReading: (done, total) => `正在生成总结：读取 ${done}/${total}…`,
  remaining: (eta) => `剩余 ${eta}`, switchLang: "切换界面语言",
};

export const statusBar = { ru, en, zh };
