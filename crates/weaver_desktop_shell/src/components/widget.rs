//! Widget system — re-exported from `weaver_fabric`, plus the immediate-mode
//! render adapter and the `Label` leaf.
//!
//! The widget model (trait, layout, style, container, events, state) lives in
//! `weaver_fabric` with no toolkit dependency. This module adds the **render
//! adapter**: the implementation of the fabric's [`RenderContext`] facade over
//! the immediate-mode toolkit, and the presentational leaves that render through it.

use std::collections::HashMap;
use std::path::Path;

// Re-export the fabric's model for the rest of the shell crate (these are also
// in scope locally).
pub use weaver_fabric::layout::{Align, Axis, Justify, Size, Spacing};
pub use weaver_fabric::render::{ImageId, Interaction, RenderContext};
pub use weaver_fabric::widget::{Container, Style, Widget};
pub use weaver_fabric::{Align2, Color32, Pos2, Rect, Vec2, pos2, vec2};

/// Loads and caches image textures for the immediate-mode adapter.
pub struct TextureRegistry {
    next_id: u64,
    by_key: HashMap<String, ImageId>,
    textures: HashMap<ImageId, egui::TextureHandle>,
}

impl Default for TextureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureRegistry {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            by_key: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

/// The immediate-mode adapter: implements the fabric's render facade.
pub struct EguiRenderContext<'a> {
    ui: &'a mut egui::Ui,
    textures: &'a mut TextureRegistry,
}

impl<'a> EguiRenderContext<'a> {
    pub fn new(ui: &'a mut egui::Ui, textures: &'a mut TextureRegistry) -> Self {
        Self { ui, textures }
    }
}

impl RenderContext for EguiRenderContext<'_> {
    fn paint_text(
        &mut self,
        pos: Pos2,
        align: Align2,
        text: &str,
        font_size: f32,
        color: Color32,
    ) {
        self.ui
            .painter()
            .text(pos, align, text, egui::FontId::proportional(font_size), color);
    }

    fn paint_rect(&mut self, rect: Rect, radius: f32, color: Color32) {
        self.ui.painter().rect_filled(rect, radius, color);
    }

    fn paint_image(&mut self, rect: Rect, image: ImageId, tint: Color32) {
        if let Some(texture) = self.textures.textures.get(&image) {
            self.ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                tint,
            );
        }
    }

    fn load_image(&mut self, key: &str, path: &Path) -> ImageId {
        if let Some(id) = self.textures.by_key.get(key) {
            return *id;
        }

        let id = ImageId(self.textures.next_id);
        self.textures.next_id += 1;
        self.textures.by_key.insert(key.to_string(), id);

        if path.exists() {
            if let Ok(img) = image::open(path) {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba.into_raw());
                let texture = self
                    .ui
                    .ctx()
                    .load_texture(key, color_image, egui::TextureOptions::LINEAR);
                self.textures.textures.insert(id, texture);
            }
        }

        id
    }

    fn interact(&mut self, rect: Rect, id: &str) -> Interaction {
        let response = self
            .ui
            .interact(rect, egui::Id::new(id), egui::Sense::click());
        Interaction {
            hovered: response.hovered(),
            clicked: response.clicked(),
        }
    }
}

// ============================================================================
// Presentational leaves
// ============================================================================

/// Leaf widget that renders a single line of text.
pub struct Label {
    id: String,
    style: Style,
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: "label".to_string(),
            style: Style::new(),
            text: text.into(),
        }
    }

    /// Set the widget id (used for layout caching and identity).
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

impl Widget for Label {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> Vec2 {
        vec2(self.text.len() as f32 * 8.0, 20.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        if self.text.is_empty() {
            return;
        }
        let color = Color32::WHITE;
        ctx.paint_text(rect.left_center(), Align2::LEFT_CENTER, &self.text, 14.0, color);
    }
}
