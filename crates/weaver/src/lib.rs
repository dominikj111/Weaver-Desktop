//! Weaver - UI Framework for SystemWeaver
//!
//! Provides reactive primitives, component abstractions, and reusable widgets.

pub mod commands;
pub mod components;
pub mod icons;
pub mod reactive;
pub mod services;
pub mod traits;
pub mod widgets;

// Re-export commonly used types at crate root
pub use commands::{
    CommandBus, ExternalReceiver, ExternalSender, TaskContext, TaskId, TaskSpawner,
    external_channel,
};
pub use components::Interactable;
pub use reactive::{Observable, Signal, SignalFn, SignalFnMulti};
pub use services::next_id;
pub use traits::{Component, InteractableHandlers};
pub use icons::{IconContext, IconTheme};
