//! Owns exactly one transcription worker and its descendants. Cancellation does
//! not depend on native inference returning to Rust or checking an atomic flag.
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::transcription::{Event, Output, Progress, Request};

const POLL: Duration = Duration::from_millis(50);

struct LogFile { path: PathBuf }
impl LogFile {
    fn create() -> Result<(Self, File), String> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
        let path = std::env::temp_dir().join(format!("speakagent-transcription-{}-{stamp}-{}.log",
            std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let file = OpenOptions::new().write(true).create_new(true).open(&path).map_err(|e| e.to_string())?;
        Ok((Self { path }, file))
    }
    fn tail(&self) -> String {
        let Ok(mut file) = File::open(&self.path) else { return String::new(); };
        let length = file.metadata().map(|m| m.len()).unwrap_or(0);
        let _ = file.seek(SeekFrom::Start(length.saturating_sub(8192)));
        let mut bytes = Vec::new();
        let _ = file.take(8192).read_to_end(&mut bytes);
        String::from_utf8_lossy(&bytes).trim().to_owned()
    }
}
impl Drop for LogFile { fn drop(&mut self) { let _ = std::fs::remove_file(&self.path); } }

#[cfg(windows)]
struct ProcessTree(windows::Win32::Foundation::HANDLE);
#[cfg(windows)]
impl ProcessTree {
    fn create() -> Result<Self, String> {
        use windows::Win32::System::JobObjects::*;
        let handle = unsafe { CreateJobObjectW(None, windows::core::PCWSTR::null()) }.map_err(|e| e.to_string())?;
        let tree = Self(handle);
        let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        unsafe { SetInformationJobObject(handle, JobObjectExtendedLimitInformation,
            &limits as *const _ as *const _, std::mem::size_of_val(&limits) as u32) }.map_err(|e| e.to_string())?;
        Ok(tree)
    }
    fn assign_and_resume(&self, child: &Child) -> Result<(), String> {
        use std::os::windows::io::AsRawHandle;
        use windows::Win32::{Foundation::HANDLE, System::JobObjects::AssignProcessToJobObject};
        let process = HANDLE(child.as_raw_handle());
        unsafe { AssignProcessToJobObject(self.0, process) }.map_err(|e| e.to_string())?;
        // std::process closes the primary thread handle. Resume through the
        // process handle only after job assignment, so no descendant can escape.
        #[link(name = "ntdll")]
        unsafe extern "system" { fn NtResumeProcess(process: *mut std::ffi::c_void) -> i32; }
        let status = unsafe { NtResumeProcess(process.0) };
        if status < 0 { return Err(format!("Could not resume transcription worker: NTSTATUS {status:#x}")); }
        Ok(())
    }
    fn terminate(&self) {
        use windows::Win32::System::JobObjects::*;
        let _ = unsafe { TerminateJobObject(self.0, 1) };
        // Termination is asynchronous. Do not release the queue slot while a
        // descendant still owns the microphone, GPU context or stdout pipe.
        loop {
            let mut info = JOBOBJECT_BASIC_ACCOUNTING_INFORMATION::default();
            if unsafe { QueryInformationJobObject(Some(self.0), JobObjectBasicAccountingInformation,
                &mut info as *mut _ as *mut _, std::mem::size_of_val(&info) as u32, None) }.is_err()
                || info.ActiveProcesses == 0 { break; }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
#[cfg(windows)]
impl Drop for ProcessTree {
    fn drop(&mut self) {
        self.terminate();
        let _ = unsafe { windows::Win32::Foundation::CloseHandle(self.0) };
    }
}

struct Worker {
    child: Child,
    terminated: bool,
    #[cfg(windows)]
    tree: ProcessTree,
}
impl Worker {
    fn spawn(command: &mut Command) -> Result<Self, String> {
        #[cfg(windows)]
        let tree = {
            use std::os::windows::process::CommandExt;
            // CREATE_SUSPENDED | CREATE_NO_WINDOW
            command.creation_flags(0x00000004 | 0x08000000);
            ProcessTree::create()?
        };
        #[cfg(unix)]
        { use std::os::unix::process::CommandExt; command.process_group(0); }
        let child = command.spawn().map_err(|e| e.to_string())?;
        let mut worker = Self { child, terminated: false, #[cfg(windows)] tree };
        #[cfg(windows)]
        if let Err(error) = worker.tree.assign_and_resume(&worker.child) {
            let _ = worker.child.kill();
            let _ = worker.child.wait();
            return Err(error);
        }
        Ok(worker)
    }
    fn terminate(&mut self) {
        if self.terminated { return; }
        #[cfg(windows)] self.tree.terminate();
        #[cfg(unix)]
        unsafe { libc::kill(-(self.child.id() as i32), libc::SIGKILL); }
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.terminated = true;
    }
}
impl Drop for Worker { fn drop(&mut self) { self.terminate(); } }

pub fn run(request: &Request, cancel: &AtomicBool, mut on_progress: impl FnMut(Progress)) -> Result<Output, String> {
    if cancel.load(Ordering::Relaxed) { return Err("cancelled".into()); }
    let request = serde_json::to_vec(request).map_err(|e| e.to_string())?;
    let (log, stderr) = LogFile::create()?;
    let mut command = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
    command.arg("--transcription-worker").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(stderr);
    let mut worker = Worker::spawn(&mut command)?;
    let mut input = worker.child.stdin.take().ok_or("Worker stdin unavailable")?;
    let output = worker.child.stdout.take().ok_or("Worker stdout unavailable")?;
    enum Message { Event(Event), Written(Result<(), String>) }
    let (sender, receiver) = mpsc::channel();
    let writer_sender = sender.clone();
    // Native initialization may hang before the worker reads stdin. Keep even
    // a full request pipe outside the cancellation supervisor's thread.
    let writer = std::thread::spawn(move || {
        let result = input.write_all(&request).and_then(|_| input.write_all(b"\n"))
            .map_err(|e| format!("Could not send transcription request: {e}"));
        drop(input);
        let _ = writer_sender.send(Message::Written(result));
    });
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(output).lines() {
            let Ok(line) = line else { break; };
            // Native runtimes sometimes write diagnostic lines to stdout.
            if let Ok(event) = serde_json::from_str::<Event>(&line) {
                if sender.send(Message::Event(event)).is_err() { break; }
            }
        }
    });
    let result = (|| {
        let mut result = None;
        loop {
            if cancel.load(Ordering::Relaxed) { return Err("cancelled".into()); }
            match receiver.recv_timeout(POLL) {
                Ok(Message::Event(Event::Progress(progress))) => on_progress(progress),
                Ok(Message::Event(Event::Result(output))) => result = Some(Ok(output)),
                Ok(Message::Event(Event::Error(error))) => result = Some(Err(error)),
                Ok(Message::Written(Err(error))) => return Err(error),
                Ok(Message::Written(Ok(()))) => {},
                Err(mpsc::RecvTimeoutError::Timeout) => {},
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // No busy spin while a worker that closed stdout shuts down.
                    std::thread::sleep(POLL);
                }
            }
            if let Some(status) = worker.child.try_wait().map_err(|e| e.to_string())? {
                // Drain the pipe reader before deciding no terminal event was
                // sent: process exit may race its final stdout delivery.
                worker.terminate();
                loop {
                    if cancel.load(Ordering::Relaxed) { return Err("cancelled".into()); }
                    match receiver.recv_timeout(POLL) {
                        Ok(Message::Event(Event::Progress(progress))) => on_progress(progress),
                        Ok(Message::Event(Event::Result(output))) => result = Some(Ok(output)),
                        Ok(Message::Event(Event::Error(error))) => result = Some(Err(error)),
                        Ok(Message::Written(Err(error))) => return Err(error),
                        Ok(Message::Written(Ok(()))) => {},
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                }
                if cancel.load(Ordering::Relaxed) { return Err("cancelled".into()); }
                if status.success() {
                    return result.unwrap_or_else(|| Err("Transcription worker exited without a result".into()));
                }
                return Err(format!("Transcription worker exited ({status}): {}", log.tail()));
            }
        }
    })();
    worker.terminate();
    drop(receiver);
    let _ = writer.join();
    let _ = reader.join();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_tail_is_bounded_and_deleted() {
        let (log, mut file) = LogFile::create().unwrap();
        file.write_all(&vec![b'x'; 20_000]).unwrap();
        file.write_all(b"native failure").unwrap();
        drop(file);
        assert_eq!(log.tail().len(), 8192);
        assert!(log.tail().ends_with("native failure"));
        let path = log.path.clone();
        drop(log);
        assert!(!path.exists());
    }

    #[cfg(windows)]
    #[test]
    fn terminating_worker_unblocks_full_request_pipe() {
        let mut command = Command::new("powershell.exe");
        command.args(["-NoProfile", "-NonInteractive", "-Command", "Start-Sleep -Seconds 120"])
            .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null());
        let mut worker = Worker::spawn(&mut command).unwrap();
        let mut input = worker.child.stdin.take().unwrap();
        let (sent, result) = mpsc::channel();
        let writer = std::thread::spawn(move || {
            let answer = input.write_all(&vec![b'x'; 4 * 1024 * 1024]);
            drop(input);
            let _ = sent.send(answer);
        });
        assert!(matches!(result.recv_timeout(POLL), Err(mpsc::RecvTimeoutError::Timeout)));
        worker.terminate();
        assert!(result.recv_timeout(Duration::from_secs(2)).unwrap().is_err());
        writer.join().unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn terminating_worker_reaps_its_descendant() {
        use windows::Win32::{Foundation::{CloseHandle, WAIT_OBJECT_0},
            System::Threading::{OpenProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE}};
        let (log, file) = LogFile::create().unwrap();
        drop(file);
        let escaped = log.path.to_string_lossy().replace('\'', "''");
        let script = format!("$p = Start-Process -WindowStyle Hidden -PassThru powershell.exe -ArgumentList '-NoProfile','-Command','Start-Sleep -Seconds 120'; [IO.File]::WriteAllText('{escaped}', [string]$p.Id); Start-Sleep -Seconds 120");
        let mut command = Command::new("powershell.exe");
        command.args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        let mut worker = Worker::spawn(&mut command).unwrap();
        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        let pid = loop {
            if let Ok(pid) = std::fs::read_to_string(&log.path).unwrap_or_default().trim().parse::<u32>() { break pid; }
            assert!(std::time::Instant::now() < deadline, "descendant did not start");
            std::thread::sleep(POLL);
        };
        let descendant = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, false, pid) }.unwrap();
        let started = std::time::Instant::now();
        worker.terminate();
        assert!(worker.child.try_wait().unwrap().is_some());
        assert_eq!(unsafe { WaitForSingleObject(descendant, 1000) }, WAIT_OBJECT_0);
        let _ = unsafe { CloseHandle(descendant) };
        assert!(started.elapsed() < Duration::from_secs(3));
    }
}
