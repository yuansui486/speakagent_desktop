const ru = {
  matching: "Сопоставляю голоса с именами…",
  preparing: "Подготавливаю и загружаю модели…",
  decoding: "Готовлю аудио…",
  diarizing: "Определяю говорящих…",
  transcribing: "Распознаю речь…",
  punctuating: "Расставляю знаки препинания…",
  done: "Готово",
};
type T = typeof ru;
const en: T = {
  matching: "Matching voices to names…",
  preparing: "Preparing and downloading models…",
  decoding: "Preparing audio…",
  diarizing: "Identifying speakers…",
  transcribing: "Transcribing…",
  punctuating: "Adding punctuation…",
  done: "Done",
};
const zh: T = {
  matching: "正在比对声纹并匹配姓名…",
  preparing: "正在准备并下载辅助模型…",
  decoding: "正在准备音频…", diarizing: "正在识别说话人…", transcribing: "正在转写语音…", punctuating: "正在添加标点…", done: "完成",
};

export const progress = { ru, en, zh };
