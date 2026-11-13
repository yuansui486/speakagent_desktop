//! CUDA discovery and an isolated, persistent Rust inference worker.
//! The main executable keeps its CPU runtime; optional CUDA DLLs live together
//! beside the worker and are never loaded into the main process.
use super::asr_config::AsrFiles;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CudaDevice {
    pub index: u32,
    pub name: String,
    pub driver_version: String,
    pub total_memory_mb: u64,
    pub free_memory_mb: u64,
    pub compute_capability: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeProbe {
    pub cuda_runtime_version: Option<i32>,
    pub cudnn_version: Option<u64>,
    pub onnxruntime_version: Option<String>,
    pub missing_libraries: Vec<String>,
    pub sherpa_version: Option<String>,
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccelerationInfo {
    pub preference: String,
    pub devices: Vec<CudaDevice>,
    /// The NVIDIA driver's maximum supported CUDA API, NOT an installed toolkit.
    pub driver_cuda_version: Option<String>,
    pub toolkit_version: Option<String>,
    pub runtime_installed: bool,
    pub runtime: Option<RuntimeProbe>,
    pub detection_error: Option<String>,
    pub last_provider: Option<String>,
    pub fallback_reason: Option<String>,
}

#[derive(Default)]
struct LastRun {
    provider: Option<String>,
    reason: Option<String>,
}
fn last_run() -> &'static Mutex<LastRun> {
    static LAST: OnceLock<Mutex<LastRun>> = OnceLock::new();
    LAST.get_or_init(|| Mutex::new(LastRun::default()))
}
pub fn record(provider: &str, reason: Option<String>) {
    *last_run().lock().unwrap() = LastRun {
        provider: Some(provider.into()),
        reason,
    };
}

pub fn last_snapshot() -> (Option<String>, Option<String>) {
    let last = last_run().lock().unwrap();
    (last.provider.clone(), last.reason.clone())
}

pub fn preference() -> String {
    super::store::get_setting("asr_provider")
        .filter(|p| matches!(p.as_str(), "auto" | "cpu" | "cuda"))
        .unwrap_or_else(|| "auto".into())
}

pub fn runtime_dir() -> PathBuf {
    super::models::models_dir().join("bin/asr-cuda")
}
fn worker_path() -> PathBuf {
    runtime_dir().join("speakagent-asr-cuda.exe")
}

fn hidden(command: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    command
}

fn output_timeout(command: &mut Command, timeout: Duration) -> Result<String, String> {
    let mut child = hidden(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if start.elapsed() < timeout => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("process timed out".into());
            }
        }
    }
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "{}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn parse_devices(csv: &str) -> Vec<CudaDevice> {
    csv.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(',').map(str::trim).collect();
            if fields.len() != 6 {
                return None;
            }
            Some(CudaDevice {
                index: fields[0].parse().ok()?,
                name: fields[1].into(),
                driver_version: fields[2].into(),
                total_memory_mb: fields[3].parse().ok()?,
                free_memory_mb: fields[4].parse().ok()?,
                compute_capability: fields[5].into(),
            })
        })
        .collect()
}

#[cfg(windows)]
fn driver_cuda_version() -> Option<String> {
    use std::ffi::c_void;
    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryW(name: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn FreeLibrary(module: *mut c_void) -> i32;
    }
    // Load only the installed NVIDIA driver. CUDA runtime/provider DLLs stay in
    // the isolated worker to avoid colliding with the app's CPU ONNX runtime.
    let system = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
    let name: Vec<u16> = PathBuf::from(system)
        .join("System32/nvcuda.dll")
        .to_string_lossy()
        .encode_utf16()
        .chain(Some(0))
        .collect();
    unsafe {
        let module = LoadLibraryW(name.as_ptr());
        if module.is_null() {
            return None;
        }
        let symbol = GetProcAddress(module, b"cuDriverGetVersion\0".as_ptr());
        let mut version = 0;
        let result = if !symbol.is_null() {
            let get: unsafe extern "system" fn(*mut i32) -> i32 = std::mem::transmute(symbol);
            if get(&mut version) == 0 {
                Some(format!("{}.{}", version / 1000, version % 1000 / 10))
            } else {
                None
            }
        } else {
            None
        };
        FreeLibrary(module);
        result
    }
}
#[cfg(not(windows))]
fn driver_cuda_version() -> Option<String> {
    None
}

fn worker_command() -> Command {
    let mut command = Command::new(worker_path());
    command.current_dir(runtime_dir());
    // Keep all CUDA dependencies local. Never change the system PATH.
    let mut paths = vec![runtime_dir()];
    if let Some(path) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&path));
    }
    if let Ok(path) = std::env::join_paths(paths) {
        command.env("PATH", path);
    }
    command
}

pub fn info() -> AccelerationInfo {
    let mut command = Command::new("nvidia-smi");
    command.args([
        "--query-gpu=index,name,driver_version,memory.total,memory.free,compute_cap",
        "--format=csv,noheader,nounits",
    ]);
    let (devices, mut error) = match output_timeout(&mut command, Duration::from_secs(4)) {
        Ok(csv) => (parse_devices(&csv), None),
        Err(error) => (Vec::new(), Some(format!("nvidia-smi: {error}"))),
    };
    let toolkit = output_timeout(
        Command::new("nvcc").arg("--version"),
        Duration::from_secs(2),
    )
    .ok()
    .and_then(|s| {
        s.split("release ")
            .nth(1)
            .and_then(|part| part.split(',').next())
            .map(str::to_owned)
    });
    let installed = cfg!(windows) && worker_path().is_file();
    let runtime = if installed {
        match output_timeout(worker_command().arg("--probe"), Duration::from_secs(8))
            .and_then(|s| serde_json::from_str::<RuntimeProbe>(&s).map_err(|e| e.to_string()))
        {
            Ok(probe) => Some(probe),
            Err(e) => {
                error = Some(format!("CUDA runtime: {e}"));
                None
            }
        }
    } else {
        None
    };
    let last = last_run().lock().unwrap();
    AccelerationInfo {
        preference: preference(),
        devices,
        driver_cuda_version: driver_cuda_version(),
        toolkit_version: toolkit,
        runtime_installed: installed,
        runtime,
        detection_error: error,
        last_provider: last.provider.clone(),
        fallback_reason: last.reason.clone(),
    }
}

pub fn try_worker(files: &AsrFiles, threads: i32) -> Result<Option<CudaWorker>, String> {
    try_worker_with_provider(files, threads, &preference())
}

pub fn try_worker_with_provider(files: &AsrFiles, threads: i32, provider: &str) -> Result<Option<CudaWorker>, String> {
    if provider == "cpu" {
        return Ok(None);
    }
    if !cfg!(windows) {
        return Err("CUDA worker currently supports Windows x64".into());
    }
    let detected = info();
    let device = detected
        .devices
        .iter()
        .max_by_key(|d| d.free_memory_mb)
        .ok_or("No NVIDIA CUDA device detected")?;
    if !detected.runtime_installed {
        return Err("CUDA runtime not installed".into());
    }
    let probe = detected.runtime.ok_or_else(|| {
        detected
            .detection_error
            .unwrap_or_else(|| "CUDA runtime unavailable".into())
    })?;
    if !probe.missing_libraries.is_empty() {
        return Err(format!(
            "CUDA DLLs missing: {}",
            probe.missing_libraries.join(", ")
        ));
    }
    if !matches!(probe.cuda_runtime_version, Some(11080))
        || !matches!(probe.cudnn_version, Some(8000..=8999))
        || probe.sherpa_version.as_deref() != Some("1.13.4")
        || probe.onnxruntime_version.is_none()
    {
        return Err(format!("CUDA runtime version mismatch: expected sherpa-onnx 1.13.4, CUDA 11.8 and cuDNN 8; found {probe:?}"));
    }
    // Automatic mode avoids loading large models onto a nearly full display GPU.
    // Explicit CUDA mode may try, with process-isolated CPU fallback on failure.
    let minimum = if matches!(files.engine, super::asr_config::Engine::Qwen3Asr) {
        2048
    } else {
        768
    };
    if provider == "auto" && device.free_memory_mb < minimum {
        return Err(format!(
            "Insufficient free VRAM: {} MB available, {minimum} MB required for automatic CUDA",
            device.free_memory_mb
        ));
    }
    CudaWorker::start(files, threads, device.index).map(Some)
}

pub struct CudaWorker {
    child: Child,
    input: Arc<Mutex<ChildStdin>>,
    output: mpsc::Receiver<String>,
    log: std::fs::File,
    log_path: PathBuf,
}

impl CudaWorker {
    fn start(files: &AsrFiles, threads: i32, device: u32) -> Result<Self, String> {
        let mut command = worker_command();
        // Match nvidia-smi's PCI ordering on systems with multiple CUDA devices.
        command.env("CUDA_DEVICE_ORDER", "PCI_BUS_ID");
        command.env("CUDA_VISIBLE_DEVICES", device.to_string());
        let log_path = std::env::temp_dir().join(format!(
            "speakagent-cuda-{}-{}.log",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let log = std::fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| e.to_string())?;
        // Native stderr is flushed before the JSON ready/result line. Read the
        // file synchronously after that line, avoiding an asynchronous reader race.
        let mut child = match hidden(&mut command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::from(log.try_clone().map_err(|e| e.to_string())?))
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                drop(log);
                let _ = std::fs::remove_file(&log_path);
                return Err(error.to_string());
            }
        };
        let input = child.stdin.take().ok_or("CUDA worker stdin unavailable")?;
        let stdout = child
            .stdout
            .take()
            .ok_or("CUDA worker stdout unavailable")?;
        let (tx, output) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        let mut worker = Self {
            child,
            input: Arc::new(Mutex::new(input)),
            output,
            log,
            log_path,
        };
        let init = serde_json::json!({"files": files, "numThreads": threads});
        worker.send(format!("{init}\n").into_bytes())?;
        let ready = worker.response()?;
        if ready.get("ready").and_then(|v| v.as_bool()) != Some(true) {
            return Err(ready.to_string());
        }
        // A CPU-only replacement runtime must never be presented as CUDA.
        if cpu_fallback(&worker.diagnostics()) {
            return Err("The installed runtime fell back to CPU".into());
        }
        Ok(worker)
    }

    fn diagnostics(&mut self) -> String {
        let length = self.log.metadata().map(|m| m.len()).unwrap_or(0);
        let _ = self
            .log
            .seek(SeekFrom::Start(length.saturating_sub(64 * 1024)));
        let mut bytes = Vec::new();
        let _ = self.log.read_to_end(&mut bytes);
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn response(&mut self) -> Result<serde_json::Value, String> {
        let line = match self.output.recv_timeout(Duration::from_secs(90)) {
            Ok(line) => line,
            Err(e) => {
                // A timed-out response cannot be reused for the next PCM request.
                let _ = self.child.kill();
                let _ = self.child.wait();
                return Err(format!("CUDA worker: {e}. {}", self.diagnostics()));
            }
        };
        let value: serde_json::Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        if let Some(error) = value.get("error") {
            return Err(format!("{error}. {}", self.diagnostics()));
        }
        Ok(value)
    }

    fn send(&mut self, bytes: Vec<u8>) -> Result<(), String> {
        // Pipe writes can block when the native runtime hangs before reading PCM.
        let input = self.input.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let result = input
                .lock()
                .map_err(|e| e.to_string())
                .and_then(|mut input| {
                    input
                        .write_all(&bytes)
                        .and_then(|_| input.flush())
                        .map_err(|e| e.to_string())
                });
            let _ = tx.send(result);
        });
        match rx.recv_timeout(Duration::from_secs(15)) {
            Ok(Ok(())) => Ok(()),
            result => {
                let _ = self.child.kill();
                let _ = self.child.wait();
                Err(format!("CUDA worker input failed: {result:?}"))
            }
        }
    }

    pub fn decode(
        &mut self,
        samples: &[f32],
    ) -> Result<sherpa_onnx::OfflineRecognizerResult, String> {
        if samples.is_empty() || samples.len() > 16_000 * 60 {
            return Err("Invalid CUDA PCM request length".into());
        }
        let mut bytes = Vec::with_capacity(4 + samples.len() * 4);
        bytes.extend_from_slice(&(samples.len() as u32).to_le_bytes());
        for sample in samples {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        self.send(bytes)?;
        let value = self.response()?;
        if cpu_fallback(&self.diagnostics()) {
            return Err("The installed CUDA runtime fell back to CPU".into());
        }
        serde_json::from_value(value).map_err(|e| e.to_string())
    }
}

impl Drop for CudaWorker {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        // Windows permits deleting an open file with Rust's default sharing flags.
        let _ = std::fs::remove_file(&self.log_path);
    }
}

fn cpu_fallback(log: &str) -> bool {
    let log = log.to_ascii_lowercase();
    log.contains("fallback to cpu")
        || log.contains("fall back to cpu")
        || log.contains("falling back to cpu")
        || log.contains("default to cpu")
        || log.contains("defaulting to cpu")
        || log.contains("cudaexecutionprovider is not")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_nvidia_csv_without_confusing_driver_and_toolkit_versions() {
        let devices =
            parse_devices("0, NVIDIA GeForce GTX 1050, 581.57, 2048, 560, 6.1\ninvalid\n");
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].compute_capability, "6.1");
        assert_eq!(devices[0].total_memory_mb, 2048);
        assert_eq!(devices[0].driver_version, "581.57");
    }

    #[test]
    fn detects_native_cpu_fallback_messages() {
        assert!(cpu_fallback("Please check. Fallback to CPU"));
        assert!(cpu_fallback("CUDA not available; falling back to CPU"));
        assert!(!cpu_fallback("CUDAExecutionProvider initialized"));
    }
}
