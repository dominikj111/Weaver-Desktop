//! Weaver Fabric — the renderer-neutral widget model.
//!
//! The **thin contract** between application and UI toolkit:
//!
//! ```text
//! [ application backend ] <-> [ thin contract ] <-> [ ui toolkit ]
//! ```
//!
//! The fabric owns application state (the model), the layout engine, and semantic
//! event objects. It has **no UI-toolkit dependency** — rendering is the job of
//! per-backend adapters (the desktop shell's immediate-mode adapter today, GTK later).
//!
//! Geometry comes from [`emath`] (a standalone math crate, re-exported below), so the
//! layout engine is toolkit-neutral: `Rect`/`Vec2`/`Pos2` are the same math types the
//! immediate-mode adapter already uses, without depending on it.

pub mod event;
pub mod layout;
pub mod reactive;
pub mod render;
pub mod widget;

// Re-export emath geometry as the fabric's geometry, and ecolor/emath color+align
// types used by the render facade.
pub use ecolor::Color32;
pub use emath::{pos2, vec2, Align2, Pos2, Rect, Vec2};
