// Диагностика и поддержка: ID устройства + служебная информация + issue на GitHub.
const ru = {
  deviceId: "ID устройства",
  serviceInfo: "Служебная информация",
  forQuestionOrBug: "— для вопроса или бага",
  copyAll: "Скопировать всё",
  reportOnGithub: "Сообщить о проблеме на GitHub",
  // Префилл тела issue на GitHub (diag — служебная информация, подставляется как есть).
  issueBody: (diag: string) => `Опишите проблему или вопрос здесь.\n\n---\n${diag}`,
};
type T = typeof ru;
const en: T = {
  deviceId: "Device ID",
  serviceInfo: "Diagnostic info",
  forQuestionOrBug: "— for a question or bug",
  copyAll: "Copy all",
  reportOnGithub: "Report an issue on GitHub",
  issueBody: (diag) => `Describe your problem or question here.\n\n---\n${diag}`,
};
const zh: T = {
  deviceId: "设备 ID", serviceInfo: "诊断信息", forQuestionOrBug: "用于咨询或报告故障", copyAll: "复制全部",
  reportOnGithub: "在 GitHub 上报告问题", issueBody: (diag) => `请在此描述您的问题。\n\n---\n${diag}`,
};

export const diagnostics = { ru, en, zh };
