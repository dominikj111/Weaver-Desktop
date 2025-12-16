//! Reusable UI components and interaction primitives.
//!
//! Components are larger UI structures that manage their own rendering context,
//! as opposed to widgets which are atomic elements rendered into a Ui.

mod interactable;
mod modal;
mod overlay;

pub use interactable::Interactable;
pub use modal::show_modal;
pub use overlay::{show_fullscreen_overlay, show_overlay};
