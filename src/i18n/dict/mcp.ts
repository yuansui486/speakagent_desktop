

const en = {
  title: "MCP Server",
  intro:
    "A local Model Context Protocol server. Any AI or coding agent (Claude Code, Cursor, VS Code) can reach the SpeakAgent engine — transcribe files, run diarization, build protocols, and browse history. Works offline, on this computer only.",

  running: "Server running",
  stopped: "Server stopped",
  start: "Start",

  connectHeading: "Connect an agent",
  quickAdd: "Quick add to popular clients:",
  claudeCodeHint: "Claude Code — run in the terminal:",
  configHint: "Cursor / VS Code / Claude Desktop — add to your MCP config:",
  bearerToken: "Bearer <your token>",

  toolsHeading: "Tools",
  settingsHeading: "Settings",
  port: "Port",
  token: "Token (optional)",
  tokenPlaceholder: "empty — no authorization (localhost only)",
  autostart: "Start the server when the app launches",
  saveApply: "Save & apply",

  tools: {
    status: "version, active model, readiness",
    transcribe: "transcribe a file → text",
    diarize: "transcribe with speaker separation",
    protocol: "meeting protocol/summary (LLM)",
    todo: "task list from a recording (LLM)",
    summarize: "summary of a recording from history",
    list_jobs: "list of history recordings",
    get_transcript: "recording text by id",
  },
};

type T = typeof en;

const zh: T = {
  title: "MCP 服务器", intro: "本地 Model Context Protocol 服务器。AI 或编程代理（如 Claude Code、Cursor、VS Code）可调用 SpeakAgent 引擎，转写文件、区分说话人、生成会议纪要并查看历史记录。支持离线运行，仅限本机访问。",
  running: "服务器运行中", stopped: "服务器已停止", start: "启动", connectHeading: "连接代理",
  quickAdd: "快速添加到常用客户端：", claudeCodeHint: "Claude Code — 在终端执行：",
  configHint: "Cursor / VS Code / Claude Desktop — 添加到 MCP 配置：", bearerToken: "Bearer <您的访问密钥>",
  toolsHeading: "工具", settingsHeading: "设置", port: "端口", token: "访问密钥（可选）",
  tokenPlaceholder: "留空表示无需身份验证（仅限本机）", autostart: "随应用启动服务器", saveApply: "保存并应用",
  tools: { status: "版本、当前模型和就绪状态", transcribe: "将文件转写为文本", diarize: "转写并区分说话人",
    protocol: "生成会议纪要／摘要（大语言模型）", todo: "从录音中提取任务清单（大语言模型）",
    summarize: "总结历史记录中的录音", list_jobs: "列出历史录音", get_transcript: "根据 ID 获取转写文本" },
};

export const mcp = { en, zh };
