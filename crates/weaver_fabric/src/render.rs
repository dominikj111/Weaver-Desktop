//! The render facade — backend-agnostic paint + interaction utilities.
//!
//! Widgets call these methods from `Widget::render()`; each toolkit adapter
//! (immediate-mode desktop shell today, GTK later) implements the trait against its
//! own primitives. This is the "render as a translation layer" contract: `render()`
//! stays thin, the facade does the toolkit-specific work.

use std::path::Path;

use crate::{Align2, Color32, Pos2, Rect};

/// A stable handle to an image loaded by the backend (resolved via
/// [`RenderContext::load_image`]). Opaque to the fabric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(pub u64);

/// Interaction state for a widget rect this frame.
#[derive(Debug, Clone, Copy, Default)]
pub struct Interaction {
    pub hovered: bool,
    pub clicked: bool,
}

/// Backend-agnostic paint + interaction facade. Implemented by each toolkit adapter.
pub trait RenderContext {
    /// Paint a single line of text, anchored at `pos` according to `align`.
    fn paint_text(&mut self, pos: Pos2, align: Align2, text: &str, font_size: f32, color: Color32);

    /// Paint a filled, optionally rounded rectangle.
    fn paint_rect(&mut self, rect: Rect, radius: f32, color: Color32);

    /// Paint a loaded image, tinted.
    fn paint_image(&mut self, rect: Rect, image: ImageId, tint: Color32);

    /// Load an image from disk (cached by the backend) and return a handle.
    fn load_image(&mut self, key: &str, path: &Path) -> ImageId;

    /// Register an interaction region and report this frame's state.
    fn interact(&mut self, rect: Rect, id: &str) -> Interaction;
}
