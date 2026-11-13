// Витрина выбора ASR-модели: метки пригодности под железо + заметки-подсказки.

const en = {
  fitGood: "Good fit",
  fitOk: "Should work",
  fitHeavy: "Heavy for this PC",

  noteLowRam: "not enough RAM",
  noteMostAccurateSlower: "more robust to noise, but slower",
  noteVerySlowCpu: "very slow on this CPU",
  noteWillBeSlow: "will be slow",
  noteSlightlySlower: "slightly slower",

  hwEstimateTitle: (hw: string) => `Hardware estimate: ${hw}`,
};

type T = typeof en;

const zh: T = {
  fitGood: "非常适合", fitOk: "可以运行", fitHeavy: "本机运行较吃力",
  noteLowRam: "内存不足", noteMostAccurateSlower: "抗噪较好，但速度较慢", noteVerySlowCpu: "在此处理器上运行很慢",
  noteWillBeSlow: "运行速度较慢", noteSlightlySlower: "速度稍慢", hwEstimateTitle: (hw) => `根据本机硬件估算：${hw}`,
};

export const modelSelector = { en, zh };
