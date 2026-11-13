const ru = {
  title: "Настройки",

  defaultModel: "Модель по умолчанию",
  defaultModelHint:
    "Выберите язык распознавания. Не скачанную модель можно загрузить прямо здесь — нажмите на неё.",

  setupWizard: "Мастер настройки",
  dataFolder: "Папка с данными",

  aiFeatures: "ИИ-функции",
  aiFeaturesHint:
    "Саммари, протокол и задачи по записи. Можно считать локально (полностью офлайн, помощник докачивается один раз ~32 МБ) или через ваш облачный ИИ по токену.",

  interfaceLanguage: "Язык интерфейса",
  interfaceLanguageHint: "Меняет язык приложения и язык «Итогов встречи».",

  privacy: "Приватность",
  privacyText:
    "Все записи обрабатываются только на этом компьютере и никуда не отправляются. Интернет нужен лишь для первой загрузки моделей (и для облачного ИИ, если вы его выбрали).",
};

type T = typeof ru;

const en: T = {
  title: "Settings",

  defaultModel: "Default model",
  defaultModelHint:
    "Choose the recognition language. A model that isn't downloaded yet can be fetched right here — just click it.",

  setupWizard: "Setup wizard",
  dataFolder: "Data folder",

  aiFeatures: "AI features",
  aiFeaturesHint:
    "Summaries, minutes, and tasks for a recording. Run them locally (fully offline, the helper downloads once, ~32 MB) or through your cloud AI with a token.",

  interfaceLanguage: "Interface language",
  interfaceLanguageHint: "Changes the app language and the language of meeting summaries.",

  privacy: "Privacy",
  privacyText:
    "All recordings are processed only on this computer and never sent anywhere. The internet is needed only for the first model download (and for cloud AI, if you chose it).",
};

const zh: T = {
  title: "设置", defaultModel: "默认模型", defaultModelHint: "选择识别语言。点击尚未下载的模型即可下载。",
  setupWizard: "设置向导", dataFolder: "数据文件夹", aiFeatures: "AI 功能",
  aiFeaturesHint: "根据录音生成摘要、纪要和任务。可在本机完全离线运行（需一次性下载助手），也可使用访问密钥连接云端 AI。",
  interfaceLanguage: "界面语言", interfaceLanguageHint: "更改应用界面及会议总结的语言。", privacy: "隐私",
  privacyText: "录音转写在本机完成，无需上传。首次下载模型需要联网；如果选择云端 AI，相关文本会发送至您配置的服务商。",
};

export const settings = { ru, en, zh };
