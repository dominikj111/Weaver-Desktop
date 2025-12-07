//! Weaver - UI Framework for SystemWeaver
//!
//! Provides reactive primitives, component abstractions, and reusable widgets.

pub mod component;
pub mod reactive;
pub mod services;
pub mod widgets;

// Re-export commonly used types at crate root
pub use component::Component;
pub use reactive::{Interactable, InteractableHandlers, Observable, Signal, SignalFn, SignalFnMulti};
pub use services::next_id;
