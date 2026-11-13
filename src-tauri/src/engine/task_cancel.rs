//! A transcription attempt's cancellation and commit share one linearization
//! lock. Tokens distinguish a finished attempt from a later retry of its ID.
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

const RETAIN: Duration = Duration::from_secs(60);
const MAX_RETAINED: usize = 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Pending, Running, Finished }

struct Entry {
    flag: Arc<AtomicBool>,
    phase: Phase,
    touched: Instant,
}

#[derive(Default)]
struct Registry { entries: HashMap<String, Entry> }

impl Registry {
    fn prune(&mut self, now: Instant) {
        self.entries.retain(|_, entry| entry.phase == Phase::Running
            || now.saturating_duration_since(entry.touched) < RETAIN);
        let mut retained: Vec<_> = self.entries.iter()
            .filter(|(_, entry)| entry.phase != Phase::Running)
            .map(|(id, entry)| (id.clone(), entry.touched)).collect();
        if retained.len() > MAX_RETAINED {
            retained.sort_by_key(|(_, touched)| *touched);
            let remove = retained.len() - MAX_RETAINED;
            for (id, _) in retained.into_iter().take(remove) { self.entries.remove(&id); }
        }
    }

    fn begin(&mut self, id: &str) -> Result<Arc<AtomicBool>, String> {
        let now = Instant::now();
        self.prune(now);
        if let Some(entry) = self.entries.get_mut(id) {
            match entry.phase {
                Phase::Running => return Err("Transcription with this ID is already running".into()),
                Phase::Pending => {
                    entry.phase = Phase::Running;
                    entry.touched = now;
                    return Ok(entry.flag.clone());
                }
                Phase::Finished => {},
            }
        }
        let flag = Arc::new(AtomicBool::new(false));
        self.entries.insert(id.into(), Entry { flag: flag.clone(), phase: Phase::Running, touched: now });
        Ok(flag)
    }

    fn cancel(&mut self, id: &str) {
        let now = Instant::now();
        self.prune(now);
        match self.entries.get_mut(id) {
            Some(entry) if entry.phase == Phase::Finished => {},
            Some(entry) => {
                entry.flag.store(true, Ordering::Relaxed);
                entry.touched = now;
            }
            None => {
                self.entries.insert(id.into(), Entry {
                    flag: Arc::new(AtomicBool::new(true)), phase: Phase::Pending, touched: now,
                });
            }
        }
        self.prune(now);
    }

    fn finish<T>(&mut self, id: &str, flag: &Arc<AtomicBool>, commit: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
        let Some(entry) = self.entries.get_mut(id) else { return Err("cancelled".into()); };
        if !Arc::ptr_eq(&entry.flag, flag) || entry.phase != Phase::Running { return Err("cancelled".into()); }
        // Mark first: even a panicking commit cannot leave a reusable Running
        // entry. The mutex remains held until commit returns or unwinds.
        entry.phase = Phase::Finished;
        entry.touched = Instant::now();
        let result = if flag.load(Ordering::Relaxed) { Err("cancelled".into()) } else { commit() };
        self.prune(Instant::now());
        result
    }

    fn cleanup(&mut self, id: &str, flag: &Arc<AtomicBool>) {
        if let Some(entry) = self.entries.get_mut(id) {
            if Arc::ptr_eq(&entry.flag, flag) {
                entry.phase = Phase::Finished;
                entry.touched = Instant::now();
            }
        }
        self.prune(Instant::now());
    }
}

fn registry() -> &'static Mutex<Registry> {
    static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::default()))
}

// A commit panic poisons the lock; phase has already been set to Finished.
// Recover the valid registry so cleanup and later tasks continue to work.
pub fn begin(id: &str) -> Result<Arc<AtomicBool>, String> {
    registry().lock().unwrap_or_else(|e| e.into_inner()).begin(id)
}
pub fn cancel(id: &str) {
    registry().lock().unwrap_or_else(|e| e.into_inner()).cancel(id);
}
pub fn finish<T>(id: &str, flag: &Arc<AtomicBool>, commit: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    registry().lock().unwrap_or_else(|e| e.into_inner()).finish(id, flag, commit)
}
pub fn cleanup(id: &str, flag: &Arc<AtomicBool>) {
    registry().lock().unwrap_or_else(|e| e.into_inner()).cleanup(id, flag);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;

    #[test]
    fn early_cancel_duplicate_start_and_retry_tokens() {
        let mut registry = Registry::default();
        registry.cancel("job");
        let cancelled = registry.begin("job").unwrap();
        assert!(cancelled.load(Ordering::Relaxed));
        assert!(registry.begin("job").is_err());
        assert_eq!(registry.finish("job", &cancelled, || Ok(1)), Err("cancelled".into()));
        let retry = registry.begin("job").unwrap();
        assert!(!retry.load(Ordering::Relaxed));
        assert!(!Arc::ptr_eq(&cancelled, &retry));
        registry.cleanup("job", &cancelled);
        assert!(registry.begin("job").is_err());
        assert_eq!(registry.finish("job", &cancelled, || Ok(1)), Err("cancelled".into()));
        assert_eq!(registry.finish("job", &retry, || Ok(2)), Ok(2));
        registry.cancel("job");
        assert!(!retry.load(Ordering::Relaxed));
    }

    #[test]
    fn racing_cancel_and_commit_have_one_outcome() {
        for _ in 0..100 {
            let registry = Mutex::new(Registry::default());
            let flag = registry.lock().unwrap().begin("job").unwrap();
            let barrier = Barrier::new(2);
            let committed = AtomicBool::new(false);
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    barrier.wait();
                    registry.lock().unwrap().cancel("job");
                });
                barrier.wait();
                let result = registry.lock().unwrap().finish("job", &flag, || {
                    committed.store(true, Ordering::Relaxed);
                    Ok(())
                });
                match result {
                    Ok(()) => assert!(!flag.load(Ordering::Relaxed)),
                    Err(error) => { assert_eq!(error, "cancelled"); assert!(!committed.load(Ordering::Relaxed)); },
                }
            });
            assert_eq!(committed.load(Ordering::Relaxed), !flag.load(Ordering::Relaxed));
        }
    }

    #[test]
    fn retention_expires_only_inactive_entries_and_is_bounded() {
        let mut registry = Registry::default();
        let running = registry.begin("running").unwrap();
        registry.cancel("pending");
        let completed = registry.begin("finished").unwrap();
        registry.finish("finished", &completed, || Ok(())).unwrap();
        let future = Instant::now() + RETAIN + Duration::from_secs(1);
        registry.prune(future);
        assert_eq!(registry.entries.len(), 1);
        assert!(Arc::ptr_eq(&registry.entries["running"].flag, &running));
        for i in 0..(MAX_RETAINED + 100) { registry.cancel(&format!("pending-{i}")); }
        assert_eq!(registry.entries.len(), MAX_RETAINED + 1);
        assert!(registry.entries.contains_key("running"));
    }

    #[test]
    fn commit_failure_and_panic_allow_retry_without_overwriting_new_attempt() {
        let registry = Mutex::new(Registry::default());
        let first = registry.lock().unwrap().begin("job").unwrap();
        let result = registry.lock().unwrap().finish("job", &first, || Err::<(),_>("write failed".into()));
        assert_eq!(result, Err("write failed".into()));
        let second = registry.lock().unwrap().begin("job").unwrap();
        let panic = std::panic::catch_unwind(|| {
            registry.lock().unwrap().finish("job", &second, || -> Result<(), String> { panic!("commit panic"); })
        });
        assert!(panic.is_err());
        let mut registry = registry.lock().unwrap_or_else(|e| e.into_inner());
        let third = registry.begin("job").unwrap();
        registry.cleanup("job", &second);
        assert!(registry.begin("job").is_err());
        registry.finish("job", &third, || Ok(())).unwrap();
    }
}
