

const en = {
  about1min: "~1 min",
  aboutMin: (m: number) => `~${m} min`,
  aboutHM: (h: number, mm: number) => `~${h} h ${mm} min`,
  aboutH: (h: number) => `~${h} h`,
};

type T = typeof en;
const zh: T = {
  about1min: "约 1 分钟", aboutMin: (m) => `约 ${m} 分钟`, aboutHM: (h, mm) => `约 ${h} 小时 ${mm} 分钟`, aboutH: (h) => `约 ${h} 小时`,
};

export const perf = { en, zh };
