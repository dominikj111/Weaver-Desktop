//! Reactive primitives — provided by `weaver_fabric` (renderer-neutral).
//!
//! Re-exported here so existing `weaver_lib::reactive` / `weaver_lib::Observable`
//! call sites are unchanged.

pub use weaver_fabric::reactive::{Observable, Signal, SignalFn, SignalFnMulti};
