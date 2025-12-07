//! Time utilities with caching.

use std::time::{Duration, Instant};

use chrono::{DateTime, Local};

/// Cached time that only updates once per second.
/// Useful for UI elements that display time but don't need millisecond precision.
pub struct CachedTime {
    last_check: Instant,
    cached: DateTime<Local>,
}

impl Default for CachedTime {
    fn default() -> Self {
        Self {
            last_check: Instant::now(),
            cached: Local::now(),
        }
    }
}

impl CachedTime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the current time, updating the cache if more than 1 second has passed.
    pub fn now(&mut self) -> &DateTime<Local> {
        if self.last_check.elapsed() > Duration::from_secs(1) {
            self.cached = Local::now();
            self.last_check = Instant::now();
        }
        &self.cached
    }
}
