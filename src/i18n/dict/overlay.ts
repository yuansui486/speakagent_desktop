

const en = {
  recognizing: "Recognizing…",
  recording: "Recording…",
};

type T = typeof en;
const zh: T = {
  recognizing: "正在识别…", recording: "正在录音…",
};

export const overlay = { en, zh };
