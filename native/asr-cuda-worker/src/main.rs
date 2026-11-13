//! An isolated CUDA process: CUDA DLL/driver failures cannot terminate the app.
//! stdin: JSON initialization line, then [u32 sample count][little-endian f32 PCM].
//! stdout: one JSON response per request. Models stay loaded between segments.

#[path = "../../../src-tauri/src/engine/asr_config.rs"]
mod asr_config;

use serde::Deserialize;
use sherpa_onnx::OfflineRecognizer;
use std::io::{self, BufRead, Read, Write};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Init {
    files: asr_config::AsrFiles,
    num_threads: i32,
}

fn reply(value: serde_json::Value) -> io::Result<()> {
    let mut out = io::stdout().lock();
    serde_json::to_writer(&mut out, &value)?;
    writeln!(out)?;
    out.flush()
}

#[cfg(windows)]
mod versions {
    use std::ffi::{c_char, c_void, CStr};
    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryW(name: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn FreeLibrary(module: *mut c_void) -> i32;
    }

    unsafe fn load(name: &str) -> *mut c_void {
        let wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        LoadLibraryW(wide.as_ptr())
    }

    pub fn probe() -> serde_json::Value {
        let mut missing = Vec::new();
        // The installed track is CUDA 11.8/cuDNN 8, compatible with Pascal GPUs.
        let (mut cuda, mut cudnn, mut ort) = (None, None, None);
        unsafe {
            // ORT initializes its shared provider host before loading the CUDA
            // provider. Loading that plugin directly can fail with Win32 1114
            // even when real CUDA session creation succeeds. Probe its presence
            // here; model initialization/inference verifies execution separately.
            let executable = std::env::current_exe().ok();
            let directory = executable.as_deref().and_then(std::path::Path::parent);
            for name in [
                "onnxruntime_providers_shared.dll",
                "onnxruntime_providers_cuda.dll",
            ] {
                if !directory.is_some_and(|dir| dir.join(name).is_file()) {
                    missing.push(name);
                }
            }
            for name in [
                "cublas64_11.dll",
                "cublasLt64_11.dll",
                "cufft64_10.dll",
                "cudart64_110.dll",
                "cudnn64_8.dll",
            ] {
                let module = load(name);
                if module.is_null() {
                    missing.push(name);
                    continue;
                }
                if name == "cudart64_110.dll" {
                    let symbol = GetProcAddress(module, b"cudaRuntimeGetVersion\0".as_ptr());
                    if !symbol.is_null() {
                        let get: unsafe extern "C" fn(*mut i32) -> i32 =
                            std::mem::transmute(symbol);
                        let mut value = 0;
                        if get(&mut value) == 0 {
                            cuda = Some(value);
                        }
                    }
                }
                if name == "cudnn64_8.dll" {
                    let symbol = GetProcAddress(module, b"cudnnGetVersion\0".as_ptr());
                    if !symbol.is_null() {
                        let get: unsafe extern "C" fn() -> usize = std::mem::transmute(symbol);
                        cudnn = Some(get());
                    }
                }
                FreeLibrary(module);
            }
            let module = load("onnxruntime.dll");
            if !module.is_null() {
                #[repr(C)]
                struct ApiBase {
                    get_api: *const c_void,
                    version: unsafe extern "C" fn() -> *const c_char,
                }
                let symbol = GetProcAddress(module, b"OrtGetApiBase\0".as_ptr());
                if !symbol.is_null() {
                    let get: unsafe extern "C" fn() -> *const ApiBase = std::mem::transmute(symbol);
                    let base = get();
                    if !base.is_null() {
                        ort = Some(
                            CStr::from_ptr(((*base).version)())
                                .to_string_lossy()
                                .into_owned(),
                        );
                    }
                }
                FreeLibrary(module);
            } else {
                missing.push("onnxruntime.dll");
            }
        }
        serde_json::json!({"cudaRuntimeVersion": cuda, "cudnnVersion": cudnn,
            "onnxruntimeVersion": ort, "missingLibraries": missing,
            "sherpaVersion": sherpa_onnx::version()})
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    if std::env::args().any(|arg| arg == "--probe") {
        reply(versions::probe())?;
        return Ok(());
    }
    let mut input = io::BufReader::new(io::stdin().lock());
    let mut line = String::new();
    input.read_line(&mut line)?;
    let init: Init = serde_json::from_str(&line)?;
    let config = init.files.config(init.num_threads.clamp(1, 16), "cuda");
    let recognizer =
        OfflineRecognizer::create(&config).ok_or("CUDA recognizer initialization failed")?;
    reply(serde_json::json!({"ready": true, "provider": "cuda"}))?;
    loop {
        let mut header = [0; 4];
        if let Err(error) = input.read_exact(&mut header) {
            if error.kind() == io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(error.into());
        }
        let count = u32::from_le_bytes(header) as usize;
        if count == 0 || count > 16_000 * 60 {
            return Err("invalid PCM request length".into());
        }
        let mut bytes = vec![0; count * 4];
        input.read_exact(&mut bytes)?;
        let samples: Vec<f32> = bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        if samples.iter().any(|value| !value.is_finite()) {
            return Err("non-finite audio samples".into());
        }
        let stream = recognizer.create_stream();
        stream.accept_waveform(16_000, &samples);
        recognizer.decode(&stream);
        let result = stream
            .get_result()
            .ok_or("CUDA recognizer returned no result")?;
        reply(
            serde_json::json!({"text": result.text, "tokens": result.tokens,
            "timestamps": result.timestamps, "durations": result.durations}),
        )?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        let _ = reply(serde_json::json!({"error": error.to_string()}));
        std::process::exit(1);
    }
}
