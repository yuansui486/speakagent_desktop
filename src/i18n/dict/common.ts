// Общие атомы, переиспользуемые многими экранами. Каждый фрагмент словаря сам
// гарантирует паритет ru/en (en: <Dict> = typeof ru).
const ru = {
  // Локаль для дат/чисел (Intl).
  locale: "ru-RU",

  // Кнопки/действия.
  cancel: "Отмена",
  delete: "Удалить",
  copy: "Копировать",
  copied: "Скопировано",
  copiedBang: "Скопировано!",
  save: "Сохранить",
  saved: "Сохранено",
  stop: "Остановить",
  back: "Назад",
  clear: "Очистить",

  // Метки состояний.
  notDownloaded: "не скачана",
  clickToRename: "Нажмите, чтобы переименовать",
  auto: "Авто",

  // Параметрические.
  noResultsFor: (q: string) => `Ничего не найдено по запросу «${q}».`,
  speaker: (n: number) => `Спикер ${n}`,
  cores: (n: number) => `${n} ядер`,
  ramGb: (gb: number | string) => `${gb} ГБ ОЗУ`,
  mb: (n: number) => `${n} МБ`,

  // Метки фильтров системных диалогов выбора/сохранения файлов.
  pickerAudioVideo: "Аудио и видео",
  pickerText: "Текст",
};

export type CommonDict = typeof ru;

const en: CommonDict = {
  locale: "en-US",

  cancel: "Cancel",
  delete: "Delete",
  copy: "Copy",
  copied: "Copied",
  copiedBang: "Copied!",
  save: "Save",
  saved: "Saved",
  stop: "Stop",
  back: "Back",
  clear: "Clear",

  notDownloaded: "not downloaded",
  clickToRename: "Click to rename",
  auto: "Auto",

  noResultsFor: (q) => `No results for “${q}”.`,
  speaker: (n) => `Speaker ${n}`,
  cores: (n) => `${n} ${n === 1 ? "core" : "cores"}`,
  ramGb: (gb) => `${gb} GB RAM`,
  mb: (n) => `${n} MB`,

  pickerAudioVideo: "Audio & video",
  pickerText: "Text",
};

const zh: CommonDict = {
  locale: "zh-CN",
  cancel: "取消", delete: "删除", copy: "复制", copied: "已复制", copiedBang: "已复制！",
  save: "保存", saved: "已保存", stop: "停止", back: "返回", clear: "清空",
  notDownloaded: "未下载", clickToRename: "点击重命名", auto: "自动",
  noResultsFor: (q) => `未找到与“${q}”相关的结果。`,
  speaker: (n) => `说话人 ${n}`, cores: (n) => `${n} 核`,
  ramGb: (gb) => `${gb} GB 内存`, mb: (n) => `${n} MB`,
  pickerAudioVideo: "音频和视频", pickerText: "文本",
};

export const common = { ru, en, zh };
