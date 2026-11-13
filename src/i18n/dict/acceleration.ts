const zh = {
  title: "语音识别显卡加速", hint: "自动模式按可用显存选择；CUDA 初始化或识别失败会使用 CPU 重试同一段音频。切换后从下一次识别生效。",
  auto: "自动", cpu: "CPU", cuda: "CUDA", refresh: "重新检测", loading: "正在检测…",
  gpu: "显卡", memory: "可用 / 总显存", driver: "NVIDIA 驱动", driverCuda: "驱动支持的 CUDA 上限",
  toolkit: "CUDA 开发工具包", runtime: "本地 CUDA 运行库", cudnn: "cuDNN", ort: "ONNX Runtime",
  absent: "未安装", unknown: "尚未检测到", actual: "最近一次识别使用", pending: "尚未识别",
  fallback: "CPU 回退原因", missing: "缺少运行组件", install: "当前 CUDA 组件仅支持 Windows x64。运行组件尚未就绪时，请按项目 CHINESE_ASR.md 中的步骤运行 PowerShell 安装脚本。",
  distinction: "驱动支持版本、开发工具包和应用运行库是不同项目；无需安装完整开发工具包即可识别。",
  noGpu: "未检测到 NVIDIA 显卡", lowMemory: "可用显存不足，已选择 CPU", unavailable: "CUDA 运行组件未就绪或版本不匹配", failed: "CUDA 推理未完成，已切换 CPU", detectionFailed: "硬件或运行库检测失败", details: "查看技术详情",
  hybrid: "CUDA 模式仍可能包含 CPU 算子，实际速度取决于模型和硬件。",
};
type T = typeof zh;
const en: T = {
  title: "Speech recognition acceleration", hint: "Auto checks free VRAM. CUDA initialization or inference failure retries the same audio on CPU. Changes apply to the next recognition.",
  auto: "Auto", cpu: "CPU", cuda: "CUDA", refresh: "Detect again", loading: "Detecting…",
  gpu: "GPU", memory: "Free / total VRAM", driver: "NVIDIA driver", driverCuda: "Driver CUDA support limit",
  toolkit: "CUDA toolkit", runtime: "Local CUDA runtime", cudnn: "cuDNN", ort: "ONNX Runtime",
  absent: "Not installed", unknown: "Not detected", actual: "Last recognition backend", pending: "No recognition yet",
  fallback: "CPU fallback reason", missing: "Missing runtime components", install: "CUDA components currently support Windows x64 only. If not ready, run the PowerShell setup script as described in CHINESE_ASR.md.",
  distinction: "Driver support, toolkit and application runtime are separate versions. Recognition does not require the full toolkit.",
  noGpu: "No NVIDIA GPU detected", lowMemory: "Insufficient free VRAM; CPU selected", unavailable: "CUDA runtime is unavailable or incompatible", failed: "CUDA inference failed; switched to CPU", detectionFailed: "Hardware or runtime detection failed", details: "Technical details",
  hybrid: "CUDA may still run some operators on CPU. Speed depends on the model and hardware.",
};
const ru: T = {
  title: "Ускорение распознавания речи", hint: "Авто учитывает свободную видеопамять. При ошибке CUDA тот же фрагмент повторяется на CPU. Изменения действуют со следующего распознавания.",
  auto: "Авто", cpu: "CPU", cuda: "CUDA", refresh: "Проверить снова", loading: "Проверка…",
  gpu: "Видеокарта", memory: "Свободно / всего VRAM", driver: "Драйвер NVIDIA", driverCuda: "Предел поддержки CUDA драйвером",
  toolkit: "CUDA Toolkit", runtime: "Локальная среда CUDA", cudnn: "cuDNN", ort: "ONNX Runtime",
  absent: "Не установлено", unknown: "Не обнаружено", actual: "Последнее распознавание", pending: "Распознавание ещё не запускалось",
  fallback: "Причина перехода на CPU", missing: "Отсутствующие компоненты", install: "CUDA поддерживается только на Windows x64. Если компоненты не готовы, запустите PowerShell-скрипт по инструкции CHINESE_ASR.md.",
  distinction: "Поддержка драйвера, Toolkit и среда приложения — разные версии. Полный Toolkit не требуется.",
  noGpu: "Видеокарта NVIDIA не обнаружена", lowMemory: "Недостаточно видеопамяти; выбран CPU", unavailable: "Среда CUDA недоступна или несовместима", failed: "Ошибка CUDA; выбран CPU", detectionFailed: "Ошибка проверки оборудования или среды", details: "Технические подробности",
  hybrid: "Часть операций CUDA может выполняться на CPU. Скорость зависит от модели и оборудования.",
};
export const acceleration = { zh, en, ru };
