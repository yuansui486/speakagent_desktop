//! User-facing native messages. Technical error details remain intact.

pub fn text(ru: &'static str, en: &'static str, zh: &'static str) -> &'static str {
    match super::store::get_setting("ui_lang").as_deref() {
        Some("ru") => ru,
        Some("en") => en,
        _ => zh,
    }
}

pub fn error(message: &str) -> String {
    if matches!(super::store::get_setting("ui_lang").as_deref(), Some("ru" | "en")) {
        return message.to_owned();
    }
    match message {
        "unknown summary type" => "未知的摘要类型".into(),
        "recording not found in history" => "历史记录中找不到此录音".into(),
        "recording has no text" => "此录音暂无转写文本".into(),
        "recording not found" => "找不到此录音".into(),
        "assistant is not running" => "AI 助手尚未启动".into(),
        "could not generate a name" => "无法生成录音标题".into(),
        "unknown backend mode" => "未知的 AI 后端模式".into(),
        "invalid address" => "无效的地址".into(),
        "unknown permission" => "未知的权限类型".into(),
        "Recognition model is not installed. Open Settings and choose a model." => "尚未安装语音识别模型，请打开设置并选择模型。".into(),
        "Speaker recognition models are not installed. Download them in Settings." => "尚未安装说话人识别模型，请在设置中下载。".into(),
        "no audio track" => "文件中没有音轨".into(),
        "unknown sample rate" => "无法确定音频采样率".into(),
        "empty stream" => "音频流为空".into(),
        "ffmpeg returned an empty stream" => "ffmpeg 返回的音频流为空".into(),
        "microphone not found" => "找不到麦克风".into(),
        "failed to create the diarizer (check segmentation/embedding models)" => "无法创建说话人分离器，请检查分段模型和声纹模型".into(),
        "diarization produced no result" => "说话人分离未产生结果".into(),
        "failed to create the recognizer (check model files)" => "无法创建语音识别器，请检查模型文件".into(),
        "incomplete model download" => "模型下载不完整，请重试".into(),
        "unknown model" => "未知的模型".into(),
        "model file not found after download" => "下载完成后未找到模型文件".into(),
        "not a language model" => "所选模型不是语音识别模型".into(),
        "not a summary model" => "所选模型不是摘要生成模型".into(),
        "provide a token" => "请填写 API 密钥".into(),
        "the cloud returned an empty response" => "云端返回了空响应".into(),
        "The summary model is not installed. Download it in Settings." => "尚未安装摘要生成模型，请在设置中下载。".into(),
        "the summary helper did not respond within 2 minutes" => "摘要服务在两分钟内未响应".into(),
        "local engine not started" => "本地 AI 引擎尚未启动".into(),
        _ => {
            for (prefix, translated) in [
                ("cloud provider: ", "云端服务错误："),
                ("no free port: ", "没有可用端口："),
                ("failed to start the summary helper: ", "无法启动摘要服务："),
                ("summary helper: ", "摘要服务错误："),
                ("network error: ", "网络错误："),
                ("extraction: ", "解压失败："),
                ("microphone failed to start: ", "麦克风启动失败："),
                ("microphone config: ", "无法读取麦克风配置："),
                ("unsupported microphone format: ", "不支持的麦克风格式："),
                ("failed to open microphone: ", "无法打开麦克风："),
                ("failed to decode (", "音频解码失败（"),
                ("failed to run ffmpeg (", "无法运行 ffmpeg（"),
            ] {
                if let Some(detail) = message.strip_prefix(prefix) {
                    return format!("{translated}{detail}");
                }
            }
            message.to_owned()
        },
    }
}
