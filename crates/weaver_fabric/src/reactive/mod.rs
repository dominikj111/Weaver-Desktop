//! Reactive primitives for zero-allocation event handling and state observation.

mod observable;
mod signal_fn;

pub use observable::{Observable, Signal};
pub use signal_fn::{SignalFn, SignalFnMulti};
