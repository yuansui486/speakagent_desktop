// Диагностика и поддержка: ID устройства + служебная информация + issue на GitHub.

const en = {
  deviceId: "Device ID",
  serviceInfo: "Diagnostic info",
  forQuestionOrBug: "— for a question or bug",
  copyAll: "Copy all",
  reportOnGithub: "Report an issue on GitHub",
  issueBody: (diag: string) => `Describe your problem or question here.\n\n---\n${diag}`,
};

type T = typeof en;
const zh: T = {
  deviceId: "设备 ID", serviceInfo: "诊断信息", forQuestionOrBug: "用于咨询或报告故障", copyAll: "复制全部",
  reportOnGithub: "在 GitHub 上报告问题", issueBody: (diag) => `请在此描述您的问题。\n\n---\n${diag}`,
};

export const diagnostics = { en, zh };
