const ru = {
  recognizing: "Распознаю…",
  recording: "Запись…",
};
type T = typeof ru;
const en: T = {
  recognizing: "Recognizing…",
  recording: "Recording…",
};
const zh: T = {
  recognizing: "正在识别…", recording: "正在录音…",
};

export const overlay = { ru, en, zh };
