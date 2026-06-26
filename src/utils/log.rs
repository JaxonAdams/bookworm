use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(enabled: bool) {
    VERBOSE.store(enabled, Ordering::Relaxed);
}

pub fn log_debug(msg: &str) {
    if VERBOSE.load(Ordering::Relaxed) {
        eprintln!("[DEBUG] {msg}");
    }
}
