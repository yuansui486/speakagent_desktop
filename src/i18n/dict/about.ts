const ru = {
  title: "О приложении",
  tagline: "Расшифровка и диаризация речи — офлайн, на вашем компьютере",
  version: (v: string) => `Версия ${v}`,
  easterEgg:
    "🥚 Все ваши записи так и не покинули этот компьютер. Ни байта. Спасибо, что выбираете приватность!",

  // Обновления.
  updates: "Обновления",
  checking: "Проверяю наличие обновлений…",
  updateAvailable: (v: string) => `Доступна новая версия ${v}`,
  upToDate: "У вас последняя версия",
  autoCheck: "Автоматическая проверка обновлений",
  downloading: (pct: number) => `Загрузка ${pct}%`,
  updateBtn: "Обновить",
  checkBtn: "Проверить",

  // Полезное.
  sourceCode: "Исходный код",
  license: "Лицензия",
  hardware: "Железо",
  accel: "Ускорение",
  gpuAccel: (gpu: string | null) => (gpu ? `видеокарта — ${gpu}` : "видеокарта"),
  cpu: "процессор",

  // Поддержать проект.
  supportTitle: "Поддержать проект",
  supportText:
    "Приложение бесплатное и офлайн. Если оно вам помогает — можно поддержать разработку. Это не покупает поддержку или приоритет, просто помогает проекту жить.",
  copyAddrTitle: (label: string, addr: string) => `Скопировать адрес ${label}: ${addr}`,

  // Состояние компонентов.
  componentsTitle: "Состояние компонентов",
  asrLabel: "Распознавание речи",
  asrOk: "готово к работе",
  asrBad: "нужно скачать модель",
  aiLabel: "ИИ-функции (итоги встреч)",
  aiOk: "готово",
  aiBad: "не настроено",
  accelLabel: "Ускорение вычислений",
  gpuFallback: "видеокарта",
  cpuOnly: "только процессор",

  // Диагностика.
  diagnosticsTitle: "Диагностика и поддержка",
  diagnosticsIntro:
    "Уникальный ID этого компьютера и служебная информация — пригодятся, если нужно задать вопрос или сообщить о проблеме.",
  madeWith: "Сделано с ❤️ для тех, кому важна приватность записей.",
};

type T = typeof ru;

const en: T = {
  title: "About",
  tagline: "Speech transcription and diarization — offline, on your computer",
  version: (v) => `Version ${v}`,
  easterEgg:
    "🥚 None of your recordings ever left this computer. Not a single byte. Thank you for choosing privacy!",

  updates: "Updates",
  checking: "Checking for updates…",
  updateAvailable: (v) => `Version ${v} is available`,
  upToDate: "You're on the latest version",
  autoCheck: "Automatic update checks",
  downloading: (pct) => `Downloading ${pct}%`,
  updateBtn: "Update",
  checkBtn: "Check",

  sourceCode: "Source code",
  license: "License",
  hardware: "Hardware",
  accel: "Acceleration",
  gpuAccel: (gpu) => (gpu ? `GPU — ${gpu}` : "GPU"),
  cpu: "CPU",

  supportTitle: "Support the project",
  supportText:
    "The app is free and offline. If it helps you, consider supporting its development. This doesn't buy you support or priority — it just helps the project stay alive.",
  copyAddrTitle: (label, addr) => `Copy ${label} address: ${addr}`,

  componentsTitle: "Component status",
  asrLabel: "Speech recognition",
  asrOk: "ready to use",
  asrBad: "model needs downloading",
  aiLabel: "AI features (meeting summaries)",
  aiOk: "ready",
  aiBad: "not configured",
  accelLabel: "Compute acceleration",
  gpuFallback: "GPU",
  cpuOnly: "CPU only",

  diagnosticsTitle: "Diagnostics and support",
  diagnosticsIntro:
    "This computer's unique ID and technical details — handy if you need to ask a question or report a problem.",
  madeWith: "Made with ❤️ for people who care about the privacy of their recordings.",
};

const zh: T = {
  title: "关于", tagline: "在您的电脑上离线完成语音转写与说话人分离",
  version: (v) => `版本 ${v}`,
  easterEgg: "🥚 您的所有录音始终保存在这台电脑上，一个字节也未曾传出。感谢您重视隐私！",
  updates: "更新", checking: "正在检查更新…", updateAvailable: (v) => `发现新版本 ${v}`,
  upToDate: "当前已是最新版本", autoCheck: "自动检查更新", downloading: (pct) => `正在下载 ${pct}%`,
  updateBtn: "更新", checkBtn: "检查更新", sourceCode: "源代码", license: "许可证",
  hardware: "硬件", accel: "加速", gpuAccel: (gpu) => gpu ? `显卡 — ${gpu}` : "显卡", cpu: "处理器",
  supportTitle: "支持项目", supportText: "本应用免费且支持离线使用。如果它对您有帮助，欢迎支持开发。捐助仅用于维持项目，不提供额外技术支持或优先待遇。",
  copyAddrTitle: (label, addr) => `复制 ${label} 地址：${addr}`,
  componentsTitle: "组件状态", asrLabel: "语音识别", asrOk: "可用", asrBad: "需要下载模型",
  aiLabel: "AI 功能（会议总结）", aiOk: "就绪", aiBad: "未配置", accelLabel: "计算加速",
  gpuFallback: "显卡", cpuOnly: "仅使用处理器", diagnosticsTitle: "诊断与支持",
  diagnosticsIntro: "本机的唯一标识和诊断信息，可在咨询问题或报告故障时提供。",
  madeWith: "为重视录音隐私的您用心打造 ❤️。",
};

export const about = { ru, en, zh };
