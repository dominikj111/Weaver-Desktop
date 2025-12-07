use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};

use chrono::{DateTime, Local};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// Returns a unique runtime ID (thread-safe, monotonically increasing)
pub fn next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub struct CachedTime {
    last_check: Instant,
    cached: DateTime<Local>,
}

impl CachedTime {
    pub fn now(&mut self) -> &DateTime<Local> {
        if self.last_check.elapsed() > Duration::from_secs(1) {
            self.cached = chrono::Local::now();
            self.last_check = Instant::now();
        }
        &self.cached
    }
}
