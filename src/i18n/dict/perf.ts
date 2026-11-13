const ru = {
  about1min: "~1 мин",
  aboutMin: (m: number) => `~${m} мин`,
  aboutHM: (h: number, mm: number) => `~${h} ч ${mm} мин`,
  aboutH: (h: number) => `~${h} ч`,
};
type T = typeof ru;
const en: T = {
  about1min: "~1 min",
  aboutMin: (m) => `~${m} min`,
  aboutHM: (h, mm) => `~${h} h ${mm} min`,
  aboutH: (h) => `~${h} h`,
};
const zh: T = {
  about1min: "约 1 分钟", aboutMin: (m) => `约 ${m} 分钟`, aboutHM: (h, mm) => `约 ${h} 小时 ${mm} 分钟`, aboutH: (h) => `约 ${h} 小时`,
};

export const perf = { ru, en, zh };
