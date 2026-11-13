const ru = {
  title: "Расшифровка речи — прямо на вашем компьютере",
  subtitle1:
    "Превратите любую запись — интервью, встречу, лекцию, голосовое — в текст.",
  subtitle2:
    "Быстро, аккуратно и полностью офлайн: файлы никуда не загружаются.",
  cta: "Расшифровать запись",

  feat1Title: "Работает без интернета",
  feat1Text:
    "Всё считается на вашем устройстве. Никаких загрузок и подписок за минуты.",
  feat2Title: "Различает говорящих",
  feat2Text:
    "Понимает, кто и когда говорил, — удобно для встреч и интервью.",
  feat3Title: "Полная приватность",
  feat3Text:
    "Записи не покидают компьютер. Ваши разговоры остаются вашими.",
};

type T = typeof ru;

const en: T = {
  title: "Speech transcription — right on your computer",
  subtitle1:
    "Turn any recording — an interview, a meeting, a lecture, a voice note — into text.",
  subtitle2:
    "Fast, accurate, and fully offline: your files are never uploaded anywhere.",
  cta: "Transcribe a recording",

  feat1Title: "Works without internet",
  feat1Text:
    "Everything runs on your device. No uploads, no per-minute subscriptions.",
  feat2Title: "Tells speakers apart",
  feat2Text: "Knows who spoke and when — handy for meetings and interviews.",
  feat3Title: "Full privacy",
  feat3Text:
    "Recordings never leave your computer. Your conversations stay yours.",
};

const zh: T = {
  title: "语音转文字，就在您的电脑上", subtitle1: "将采访、会议、讲座、语音留言等各种录音转换为文字。",
  subtitle2: "快速、准确，完全离线运行：文件无需上传。", cta: "开始转写",
  feat1Title: "无需联网", feat1Text: "所有处理均在本机完成，无需上传，也无需按分钟订阅。",
  feat2Title: "区分说话人", feat2Text: "识别谁在何时发言，方便整理会议和采访。",
  feat3Title: "保护隐私", feat3Text: "录音保留在本机，让您的对话只属于您。",
};

export const home = { ru, en, zh };
