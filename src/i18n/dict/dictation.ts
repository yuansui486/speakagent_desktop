

const en = {
  title: "Dictation",
  intro:
    "Hold the hotkey and dictate — the text is recognized locally in an instant, copied to the clipboard, and pasted at the cursor in any app.",

  recording: "Recording…",
  processing: "Recognizing…",
  ready: "Ready to dictate",
  holdToRecord: "Hold to record",

  settings: "Settings",
  hotkeyLabel: "Hotkey (push-to-talk) — a single key works too, e.g. right Shift",
  capturing: "Press a key…",
  record: "Record",
  mode: "Mode",
  modeHold: "Hold and speak",
  modeToggle: "Tap to start/stop",
  autopaste: "Auto-paste at cursor",
  sound: "Start/stop sound",
  mic: "Microphone",
  defaultDevice: "Default device",
  asrModel: "Recognition model",
  asActive: "Same as active (in Settings)",
  macHelp:
    "On macOS, grant the app two permissions (System Settings → Privacy & Security) and restart it: Input Monitoring — so the global hotkey fires even when the window is inactive, and Accessibility — for auto-paste. Without Accessibility the text is still copied to the clipboard — paste it manually (⌘V).",

  permsTitle: "macOS permissions",
  permsIntro:
    "Dictation needs system permissions. After granting them, a restart is sometimes required.",
  permInputTitle: "Input Monitoring",
  permInputDesc: "so the hotkey fires while another app is in the foreground",
  permAccessTitle: "Accessibility",
  permAccessDesc: "for auto-pasting the recognized text at the cursor",
  permGranted: "granted",
  permRequest: "Request",
  repairHint:
    "Permissions reset after an update and toggling them in Settings doesn't help? Click — the app resets the stuck system records and re-requests access. (Cause: until the app has a paid Apple signature, macOS treats each update as a “new app”.)",
  repairBtn: "Repair permissions",

  historyTitle: "Dictation history",
  searchPlaceholder: "Search text…",
  emptyHint: "Your quick recognitions will show up here.",
  clearConfirmTitle: "Clear dictation history?",
  clearConfirmBody: "All quick recognitions will be permanently deleted.",

  keyLabels: {
    ShiftLeft: "Left Shift",
    ShiftRight: "Right Shift",
    ControlLeft: "Left Ctrl",
    ControlRight: "Right Ctrl",
    Alt: "Alt (left)",
    AltGr: "Alt (right)",
    MetaLeft: "Left ⌘/Win",
    MetaRight: "Right ⌘/Win",
    Space: "Space",
    Return: "Enter",
    UpArrow: "↑",
    DownArrow: "↓",
    LeftArrow: "←",
    RightArrow: "→",
  },
};

type T = typeof en;

const zh: T = {
  title: "语音输入", intro: "按住快捷键说话，即可在本地快速识别为文字，复制到剪贴板，并粘贴到任意应用的光标位置。",
  recording: "正在录音…", processing: "正在识别…", ready: "可以开始语音输入", holdToRecord: "按住开始录音",
  settings: "设置", hotkeyLabel: "快捷键（按住说话）— 也可使用单个按键，例如右 Shift",
  capturing: "请按下按键…", record: "设置按键", mode: "模式", modeHold: "按住说话", modeToggle: "按下开始／停止",
  autopaste: "自动粘贴到光标位置", sound: "开始／停止提示音", mic: "麦克风", defaultDevice: "默认设备",
  asrModel: "识别模型", asActive: "使用当前模型（在设置中选择）",
  macHelp: "在 macOS 上，请前往“系统设置 → 隐私与安全性”，授予应用两项权限后重新启动应用：“输入监控”用于在窗口未激活时响应全局快捷键；“辅助功能”用于自动粘贴。未授予辅助功能权限时，文字仍会复制到剪贴板，可手动粘贴（⌘V）。",
  permsTitle: "macOS 权限", permsIntro: "语音输入需要系统权限。授权后可能需要重新启动应用。",
  permInputTitle: "输入监控", permInputDesc: "在其他应用处于前台时响应快捷键",
  permAccessTitle: "辅助功能", permAccessDesc: "将识别结果自动粘贴到光标位置", permGranted: "已授权", permRequest: "请求授权",
  repairHint: "更新后权限失效，且在系统设置中重新开关也无效？点击此按钮可重置异常的系统权限记录并重新请求授权。（原因：应用尚未使用付费 Apple 签名，macOS 会将每次更新视为新应用。）",
  repairBtn: "修复权限", historyTitle: "语音输入历史", searchPlaceholder: "搜索文本…", emptyHint: "您的语音输入记录将显示在这里。",
  clearConfirmTitle: "清空语音输入历史？", clearConfirmBody: "所有语音输入记录都将被永久删除。",
  keyLabels: { ShiftLeft: "左 Shift", ShiftRight: "右 Shift", ControlLeft: "左 Ctrl", ControlRight: "右 Ctrl",
    Alt: "左 Alt", AltGr: "右 Alt", MetaLeft: "左 ⌘/Win", MetaRight: "右 ⌘/Win", Space: "空格",
    Return: "Enter", UpArrow: "↑", DownArrow: "↓", LeftArrow: "←", RightArrow: "→" },
};

export const dictation = { en, zh };
