//! Weaver - UI Framework for Weaver Desktop
//!
//! Provides reactive primitives, component abstractions, and reusable widgets.

pub mod commands;
pub mod components;
pub mod icons;
pub mod reactive;
pub mod services;
pub mod theme;
pub mod traits;
pub mod widgets;

// Re-export commonly used types at crate root
pub use commands::{
    CommandBus, ExternalReceiver, ExternalSender, TaskContext, TaskId, TaskSpawner,
    external_channel,
};
pub use components::Interactable;
pub use icons::{IconContext, IconTheme};
pub use reactive::{Observable, Signal, SignalFn, SignalFnMulti};
pub use services::next_id;
pub use theme::{Theme, ThemeColors, ThemeSpacing};
pub use traits::{Component, InteractableHandlers};
