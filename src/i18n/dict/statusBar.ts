

const en = {
  summarizing: "summarizing…",
  summarizingReading: (done: number, total: number) => `summarizing: reading ${done}/${total}…`,
  remaining: (eta: string) => `remaining ${eta}`,
  switchLang: "Switch interface language",
};

type T = typeof en;
const zh: T = {
  summarizing: "正在生成总结…", summarizingReading: (done, total) => `正在生成总结：读取 ${done}/${total}…`,
  remaining: (eta) => `剩余 ${eta}`, switchLang: "切换界面语言",
};

export const statusBar = { en, zh };
