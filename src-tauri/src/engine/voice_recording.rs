//! A bounded microphone capture dedicated to voice enrollment, independent of dictation.
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::time::Duration;

struct Recording {
    stop: mpsc::Sender<()>,
    result: mpsc::Receiver<Result<Vec<f32>, String>>,
    model_id: &'static str,
}

fn state() -> &'static Mutex<Option<Recording>> {
    static STATE: OnceLock<Mutex<Option<Recording>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(None))
}

pub fn is_recording() -> bool { state().lock().unwrap().is_some() }

pub fn start() -> Result<(), String> {
    let mut current = state().lock().unwrap();
    if current.is_some() { return Err("已有声纹录音，请先保存或取消。".into()); }
    let model_id = super::models::diarization_embedding_id();
    let (stop, commands) = mpsc::channel();
    let (ready, readiness) = mpsc::channel();
    let (result, output) = mpsc::channel();
    std::thread::spawn(move || {
        let recording = capture(commands, ready);
        let _ = result.send(recording);
    });
    readiness.recv_timeout(Duration::from_secs(15)).map_err(|e| e.to_string())??;
    *current = Some(Recording { stop, result: output, model_id });
    Ok(())
}

pub fn finish() -> Result<(Vec<f32>, &'static str), String> {
    let recording = state().lock().unwrap().take().ok_or("尚未开始声纹录音。")?;
    let _ = recording.stop.send(());
    let samples = recording.result.recv_timeout(Duration::from_secs(15))
        .map_err(|e| e.to_string())??;
    Ok((samples, recording.model_id))
}

pub fn cancel() {
    if let Some(recording) = state().lock().unwrap().take() {
        let _ = recording.stop.send(());
    }
}

fn capture(stop: mpsc::Receiver<()>, ready: mpsc::Sender<Result<(), String>>) -> Result<Vec<f32>, String> {
    let open = || -> Result<_, String> {
        let device = cpal::default_host().default_input_device().ok_or("找不到默认麦克风。")?;
        let supported = device.default_input_config().map_err(|e| format!("无法读取麦克风设置：{e}"))?;
        let rate = supported.sample_rate().0;
        let channels = supported.channels() as usize;
        let config: cpal::StreamConfig = supported.clone().into();
        let limit = rate as usize * channels * 60;
        let samples = Arc::new(Mutex::new(Vec::<f32>::new()));
        let input_error = Arc::new(Mutex::new(None::<String>));
        let errors = input_error.clone();
        let on_error = move |e| { *errors.lock().unwrap() = Some(format!("麦克风采集失败：{e}")); };
        let stream = match supported.sample_format() {
            cpal::SampleFormat::F32 => {
                let buffer = samples.clone();
                device.build_input_stream(&config, move |data: &[f32], _: &_| {
                    append(&buffer, data.iter().copied(), limit);
                }, on_error, None)
            }
            cpal::SampleFormat::I16 => {
                let buffer = samples.clone();
                device.build_input_stream(&config, move |data: &[i16], _: &_| {
                    append(&buffer, data.iter().map(|&x| x as f32 / 32768.0), limit);
                }, on_error, None)
            }
            cpal::SampleFormat::U16 => {
                let buffer = samples.clone();
                device.build_input_stream(&config, move |data: &[u16], _: &_| {
                    append(&buffer, data.iter().map(|&x| (x as f32 - 32768.0) / 32768.0), limit);
                }, on_error, None)
            }
            other => return Err(format!("不支持的麦克风格式：{other:?}")),
        }.map_err(|e| format!("无法打开麦克风：{e}"))?;
        stream.play().map_err(|e| format!("无法启动麦克风：{e}"))?;
        Ok((stream, samples, rate, channels, input_error))
    };
    let (stream, samples, rate, channels, input_error) = match open() {
        Ok(value) => { let _ = ready.send(Ok(())); value }
        Err(error) => { let _ = ready.send(Err(error.clone())); return Err(error); }
    };
    // The worker owns cpal::Stream, which is not Send on all supported platforms.
    let _ = stop.recv_timeout(Duration::from_secs(60));
    drop(stream);
    if let Some(error) = input_error.lock().unwrap().take() { return Err(error); }
    let raw = std::mem::take(&mut *samples.lock().unwrap());
    let mono: Vec<f32> = raw.chunks(channels.max(1))
        .map(|frame| frame.iter().sum::<f32>() / frame.len() as f32).collect();
    Ok(super::decode::resample_linear(&mono, rate, 16000))
}

fn append(buffer: &Mutex<Vec<f32>>, data: impl Iterator<Item = f32>, limit: usize) {
    let mut buffer = buffer.lock().unwrap();
    let remaining = limit.saturating_sub(buffer.len());
    buffer.extend(data.take(remaining));
}
