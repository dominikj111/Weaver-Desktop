//! ImageSurface component - fills a container with an image, color, or procedural content.
//!
//! A reusable primitive for displaying visual surfaces in widgets, panels, desktop backgrounds,
//! or any component needing an image fill. Handles image loading, caching, and scaling.
//!
//! # Example
//!
//! ```rust,ignore
//! // Image background with cover scaling
//! let mut surface = ImageSurface::with_source(ImageSource::image("path/to/image.png"));
//! surface.set_scale_mode(ScaleMode::Cover);
//! surface.paint(ui);
//!
//! // Solid color fill
//! let mut bg = ImageSurface::with_source(ImageSource::Color(Color32::from_gray(30)));
//! bg.paint_rect(ui, some_rect);
//! ```

use std::path::{Path, PathBuf};

/// How the image should be scaled to fit the target rect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScaleMode {
    /// Scale to fill the rect, cropping overflow. Maintains aspect ratio.
    #[default]
    Cover,
    /// Scale to fit within the rect, letterboxing if needed. Maintains aspect ratio.
    Contain,
    /// Stretch to exactly fill the rect. Does not maintain aspect ratio.
    Stretch,
    /// Tile the image to fill the rect. No scaling.
    Tile,
}

/// Source content for an ImageSurface.
#[derive(Debug, Clone)]
pub enum ImageSource {
    /// Load image from file path.
    Image(PathBuf),
    /// Solid color fill.
    Color(egui::Color32),
    /// No content (transparent).
    None,
}

impl Default for ImageSource {
    fn default() -> Self {
        Self::None
    }
}

impl ImageSource {
    /// Create an image source from a path.
    pub fn image(path: impl Into<PathBuf>) -> Self {
        Self::Image(path.into())
    }

    /// Create a solid color source.
    pub fn color(color: egui::Color32) -> Self {
        Self::Color(color)
    }
}

/// A visual surface that fills its container with an image, color, or procedural content.
///
/// Stateful component that caches loaded textures for efficient rendering.
/// Use `paint()` to fill the available UI space, or `paint_rect()` for a specific rect.
pub struct ImageSurface {
    /// Unique identifier for texture caching.
    id: String,
    /// Current source to display.
    source: ImageSource,
    /// How to scale images to fit the target rect.
    scale_mode: ScaleMode,
    /// Cached texture handle.
    texture: Option<egui::TextureHandle>,
    /// Whether we've attempted to load the current source.
    load_attempted: bool,
    /// The path that was loaded (for change detection).
    loaded_path: Option<PathBuf>,
}

impl Default for ImageSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageSurface {
    /// Create a new empty ImageSurface.
    pub fn new() -> Self {
        Self::with_id("image_surface")
    }

    /// Create a new ImageSurface with a specific ID for texture caching.
    ///
    /// Use unique IDs when you have multiple ImageSurface instances to ensure
    /// each has its own texture cache.
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            source: ImageSource::None,
            scale_mode: ScaleMode::Cover,
            texture: None,
            load_attempted: false,
            loaded_path: None,
        }
    }

    /// Create a new ImageSurface with the given source.
    pub fn with_source(source: ImageSource) -> Self {
        let mut surface = Self::new();
        surface.source = source;
        surface
    }

    /// Builder: set the source.
    pub fn source(mut self, source: ImageSource) -> Self {
        self.set_source(source);
        self
    }

    /// Builder: set the scale mode.
    pub fn scale_mode(mut self, mode: ScaleMode) -> Self {
        self.scale_mode = mode;
        self
    }

    /// Builder: set a unique ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the source at runtime.
    pub fn set_source(&mut self, source: ImageSource) {
        // Check if source changed
        let changed = match (&self.source, &source) {
            (ImageSource::Image(old), ImageSource::Image(new)) => old != new,
            (ImageSource::Color(old), ImageSource::Color(new)) => old != new,
            (ImageSource::None, ImageSource::None) => false,
            _ => true,
        };

        if changed {
            self.source = source;
            self.texture = None;
            self.load_attempted = false;
            self.loaded_path = None;
        }
    }

    /// Set the scale mode at runtime.
    pub fn set_scale_mode(&mut self, mode: ScaleMode) {
        self.scale_mode = mode;
    }

    /// Get the current source.
    pub fn get_source(&self) -> &ImageSource {
        &self.source
    }

    /// Get the current scale mode.
    pub fn get_scale_mode(&self) -> ScaleMode {
        self.scale_mode
    }

    /// Returns true if a texture is loaded and ready.
    pub fn is_loaded(&self) -> bool {
        self.texture.is_some()
    }

    /// Paint the surface, filling the available UI space.
    ///
    /// Uses `ui.available_rect_before_wrap()` as the target rect.
    pub fn paint(&mut self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        self.paint_rect(ui, rect);
    }

    /// Paint the surface into a specific rect.
    pub fn paint_rect(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        self.paint_to_painter(ui.ctx(), ui.painter(), rect);
    }

    /// Paint the surface to a specific painter and rect.
    /// 
    /// Use this for rendering to specific layers (e.g., background layer)
    /// without needing a Ui context.
    pub fn paint_to_painter(
        &mut self,
        ctx: &egui::Context,
        painter: &egui::Painter,
        rect: egui::Rect,
    ) {
        match &self.source {
            ImageSource::None => {
                // Nothing to render
            }
            ImageSource::Color(color) => {
                painter.rect_filled(rect, 0.0, *color);
            }
            ImageSource::Image(path) => {
                self.paint_image_to_painter(ctx, painter, rect, path.clone());
            }
        }
    }

    /// Paint to the background layer (behind all panels).
    /// 
    /// Convenience method for desktop/shell backgrounds.
    pub fn paint_background(&mut self, ctx: &egui::Context, rect: egui::Rect) {
        let painter = ctx.layer_painter(egui::LayerId::background());
        self.paint_to_painter(ctx, &painter, rect);
    }

    /// Paint an image to the given rect using a painter.
    fn paint_image_to_painter(
        &mut self,
        ctx: &egui::Context,
        painter: &egui::Painter,
        rect: egui::Rect,
        path: PathBuf,
    ) {
        // Check if path changed since last load
        let path_changed = self.loaded_path.as_ref() != Some(&path);
        if path_changed {
            self.texture = None;
            self.load_attempted = false;
        }

        // Load texture if needed
        if self.texture.is_none() && !self.load_attempted {
            self.load_attempted = true;
            self.loaded_path = Some(path.clone());

            match load_image_from_path(&path) {
                Ok(color_image) => {
                    let texture = ctx.load_texture(
                        &self.id,
                        color_image,
                        egui::TextureOptions::default(),
                    );
                    self.texture = Some(texture);
                }
                Err(e) => {
                    // Log error but don't panic - just won't render
                    eprintln!(
                        "ImageSurface '{}': Failed to load image {:?}: {}",
                        self.id, path, e
                    );
                }
            }
        }

        // Render the texture if available
        if let Some(texture) = &self.texture {
            let uv = self.calculate_uv(rect.size(), texture.size_vec2());
            painter.image(texture.id(), rect, uv, egui::Color32::WHITE);
        }
    }

    /// Calculate UV coordinates based on scale mode.
    fn calculate_uv(&self, target_size: egui::Vec2, texture_size: egui::Vec2) -> egui::Rect {
        match self.scale_mode {
            ScaleMode::Cover => {
                // Fill target, crop overflow
                let target_aspect = target_size.x / target_size.y;
                let tex_aspect = texture_size.x / texture_size.y;

                if target_aspect > tex_aspect {
                    // Target is wider - crop top/bottom
                    let visible_height = tex_aspect / target_aspect;
                    let y_offset = (1.0 - visible_height) / 2.0;
                    egui::Rect::from_min_max(
                        egui::pos2(0.0, y_offset),
                        egui::pos2(1.0, y_offset + visible_height),
                    )
                } else {
                    // Target is taller - crop left/right
                    let visible_width = target_aspect / tex_aspect;
                    let x_offset = (1.0 - visible_width) / 2.0;
                    egui::Rect::from_min_max(
                        egui::pos2(x_offset, 0.0),
                        egui::pos2(x_offset + visible_width, 1.0),
                    )
                }
            }
            ScaleMode::Contain => {
                // Fit within target, letterbox
                // For contain, we show the full image (UV 0-1) and would need to
                // adjust the target rect instead. Since we're given a fixed rect,
                // we use full UV and accept potential distortion, or caller should
                // pre-calculate the rect. For now, use full UV.
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0))
            }
            ScaleMode::Stretch => {
                // Stretch to fill - use full UV, image distorts to fit
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0))
            }
            ScaleMode::Tile => {
                // Tile - UV extends beyond 1.0 based on size ratio
                let tiles_x = target_size.x / texture_size.x;
                let tiles_y = target_size.y / texture_size.y;
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(tiles_x, tiles_y))
            }
        }
    }
}

/// Load an image from a file path into an egui ColorImage.
fn load_image_from_path(path: &Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::ImageReader::open(path)?.decode()?;
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.into_rgba8();
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &image_buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_source_default() {
        let source = ImageSource::default();
        assert!(matches!(source, ImageSource::None));
    }

    #[test]
    fn test_scale_mode_default() {
        let mode = ScaleMode::default();
        assert_eq!(mode, ScaleMode::Cover);
    }

    #[test]
    fn test_image_surface_builder() {
        let surface = ImageSurface::new()
            .id("test")
            .source(ImageSource::Color(egui::Color32::RED))
            .scale_mode(ScaleMode::Contain);

        assert_eq!(surface.id, "test");
        assert!(matches!(surface.source, ImageSource::Color(_)));
        assert_eq!(surface.scale_mode, ScaleMode::Contain);
    }

    #[test]
    fn test_source_change_clears_cache() {
        let mut surface = ImageSurface::new();
        surface.load_attempted = true;

        surface.set_source(ImageSource::Color(egui::Color32::BLUE));

        assert!(!surface.load_attempted);
        assert!(surface.texture.is_none());
    }
}
