//! Unique ID generation.

use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// Returns a unique runtime ID (thread-safe, monotonically increasing).
pub fn next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
