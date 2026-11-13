

const en = {
  localLabel: "This computer",
  cloudLabel: "Cloud AI",

  gpuAccel: (gpu: string | null) => `GPU acceleration — ${gpu ?? "GPU"}`,
  noGpuAccel: "No acceleration — running on the CPU",
  gpuUnfit: (gpu: string) => ` (${gpu} not supported)`,

  cloudIntro:
    "Summaries, protocols, and tasks will be produced through your cloud AI (OpenAI-compatible: OpenRouter, OpenAI, and others). The token is stored only on this computer.",
  apiUrl: "API URL",
  model: "Model",
  token: "Token (access key)",
  test: "Check connection",
  testing: "Checking…",
  testOkFallback: "ok",
  testSuccess: (reply: string) => `Connected — the cloud replied: “${reply}”`,
};

type T = typeof en;

const zh: T = {
  localLabel: "本机运行", cloudLabel: "云端 AI",
  gpuAccel: (gpu) => `显卡加速 — ${gpu ?? "显卡"}`, noGpuAccel: "未启用加速 — 使用处理器运行",
  gpuUnfit: (gpu) => `（不支持 ${gpu}）`,
  cloudIntro: "将通过您的云端 AI 生成摘要、会议纪要和任务（支持兼容 OpenAI 的服务，如 OpenRouter、OpenAI 等）。访问密钥仅保存在本机。",
  apiUrl: "API 地址", model: "模型", token: "访问密钥", test: "测试连接", testing: "正在测试…",
  testOkFallback: "正常", testSuccess: (reply) => `连接成功 — 云端回复：“${reply}”`,
};

export const aiProvider = { en, zh };
