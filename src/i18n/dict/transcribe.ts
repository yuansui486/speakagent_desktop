// Экран «Расшифровка» (хаб: управление + очередь + таблица истории) и отдельный
// экран одной записи. Общие атомы (Отмена/Удалить/Назад/Остановить/Авто/поиск)
// берём из common — здесь только уникальные для страницы строки.
const ru = {
  // Статус-пилюли в таблице.
  statusRunning: "в работе",
  statusError: "ошибка",
  statusDone: "готово",

  // Модалка удаления.
  deleteTitle: "Удалить запись?",
  deleteBody:
    "Расшифровка и составленные по ней итоги будут удалены безвозвратно. Сам файл на диске не трогаем.",

  // Модалка повторной расшифровки.
  retranscribeTitle: "Расшифровать заново?",
  retranscribeBody:
    "Текущий результат для этой записи будет перезаписан, а обработка может занять время (зависит от длины записи и модели).",
  retranscribeConfirm: "Да, заново",

  // Отдельный экран записи.
  revealInSystem: (path: string) => `Показать в системе: ${path}`,

  // Заголовок + интро хаба.
  title: "Расшифровка",
  intro:
    "Выберите или перетащите одну или несколько записей — они встанут в очередь. Ниже — все ваши расшифровки, они хранятся только на этом компьютере.",

  // Управление.
  pickFiles: "Выбрать записи",
  diarizeLabel: "Определять, кто говорит",
  speakersHint: "Если знаете точное число говорящих — так надёжнее, чем авто",
  speakersLabel: "Говорящих:",

  // Очередь.
  inProgress: "В обработке",
  plusInQueue: (n: number) => `+${n} в очереди`,
  running: (elapsed: string) => `идёт ${elapsed}`,
  analyzingVoices: " · анализирую голоса…",
  waiting: "ожидает",
  removeFromQueue: "Убрать из очереди",

  // Слабое железо.
  weakCpu: "слабый процессор — обработка займёт больше времени",
  lowRam: "немного оперативной памяти — крупные модели могут тормозить",

  // История.
  historyTitle: "Мои расшифровки",
  searchPlaceholder: "Поиск по названию и тексту…",
  emptyState: "Здесь появятся ваши расшифровки — выберите запись выше.",

  // Заголовки таблицы.
  colName: "Название",
  colDuration: "Длит.",
  colStatus: "Статус",
  colDate: "Дата",

  // Тултипы.
  copyText: "Копировать текст",
};

type T = typeof ru;
const en: T = {
  statusRunning: "running",
  statusError: "error",
  statusDone: "done",

  deleteTitle: "Delete recording?",
  deleteBody:
    "The transcript and any summaries built from it will be deleted for good. The file on disk stays untouched.",

  retranscribeTitle: "Transcribe again?",
  retranscribeBody:
    "The current result for this recording will be overwritten, and processing may take a while (depending on the length and model).",
  retranscribeConfirm: "Yes, redo it",

  revealInSystem: (path) => `Reveal in file manager: ${path}`,

  title: "Transcription",
  intro:
    "Pick or drag one or more recordings — they'll join the queue. Below are all your transcripts; they're kept only on this computer.",

  pickFiles: "Choose recordings",
  diarizeLabel: "Detect who's speaking",
  speakersHint: "If you know the exact number of speakers, that's more reliable than auto",
  speakersLabel: "Speakers:",

  inProgress: "Processing",
  plusInQueue: (n) => `+${n} in queue`,
  running: (elapsed) => `running ${elapsed}`,
  analyzingVoices: " · analyzing voices…",
  waiting: "waiting",
  removeFromQueue: "Remove from queue",

  weakCpu: "weak CPU — processing will take longer",
  lowRam: "limited memory — large models may run slowly",

  historyTitle: "My transcripts",
  searchPlaceholder: "Search by name and text…",
  emptyState: "Your transcripts will appear here — choose a recording above.",

  colName: "Name",
  colDuration: "Dur.",
  colStatus: "Status",
  colDate: "Date",

  copyText: "Copy text",
};

const zh: T = {
  statusRunning: "处理中", statusError: "出错", statusDone: "完成", deleteTitle: "删除录音记录？",
  deleteBody: "转写文本及其生成的总结将被永久删除，磁盘上的原文件将保留。",
  retranscribeTitle: "重新转写？", retranscribeBody: "此录音的当前结果将被覆盖。处理可能需要一些时间，具体取决于录音长度和所选模型。",
  retranscribeConfirm: "确认重新转写", revealInSystem: (path) => `在文件管理器中显示：${path}`,
  title: "语音转写", intro: "选择或拖入一个或多个录音文件即可加入队列。所有转写记录显示在下方，仅保存在本机。",
  pickFiles: "选择录音文件", diarizeLabel: "区分说话人", speakersHint: "如果知道确切的说话人数，手动指定比自动检测更可靠", speakersLabel: "说话人数：",
  inProgress: "处理中", plusInQueue: (n) => `另有 ${n} 个排队中`, running: (elapsed) => `已运行 ${elapsed}`,
  analyzingVoices: " · 正在分析声音…", waiting: "等待中", removeFromQueue: "移出队列",
  weakCpu: "处理器性能较低，处理时间会更长", lowRam: "内存较少，运行大型模型可能较慢",
  historyTitle: "我的转写", searchPlaceholder: "搜索名称和文本…", emptyState: "转写记录将显示在这里，请先在上方选择录音。",
  colName: "名称", colDuration: "时长", colStatus: "状态", colDate: "日期", copyText: "复制文本",
};

export const transcribe = { ru, en, zh };
