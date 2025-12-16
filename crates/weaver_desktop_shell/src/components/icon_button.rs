//! Circular icon button component - renders an icon image as a circular button with fallback text.
//!
//! Supports both PNG and SVG images. SVG icons are rasterized at the appropriate size
//! using resvg for crisp rendering at any scale.

use std::path::{Path, PathBuf};

use weaver_lib::{Interactable, InteractableHandlers};

/// Circular icon button that loads an icon image and renders it as a circle.
///
/// Supports both PNG and SVG images. SVG icons are rasterized at the button size
/// for crisp rendering. The image is loaded from disk on first use or when the
/// path changes, and cached as a texture for efficient rendering. If loading fails,
/// a fallback text character is displayed instead.
pub struct IconButton {
    /// Unique identifier for this button's texture.
    id: String,
    /// Internal UI id for interaction tracking.
    internal_ui_id: usize,
    /// Current image path being displayed.
    current_path: Option<PathBuf>,
    /// Whether we already attempted to load the current path (prevents repeated errors).
    load_attempted: bool,
    /// Cached texture handle.
    texture: Option<egui::TextureHandle>,
    /// Whether we're using the fallback (no valid image loaded).
    using_fallback: bool,
    /// Fallback text to display if image loading fails.
    fallback_text: String,
    /// Size of the button (diameter).
    size: f32,
    /// Background color for the circular button (None = no background).
    background_color: Option<egui::Color32>,
    /// Padding between the button edge and the image (in pixels).
    padding: f32,
    /// Border stroke (color and width).
    stroke: Option<egui::Stroke>,
    /// Interaction handler for press/release/click signals.
    interactable: Interactable<Self>,
}

impl IconButton {
    /// Create a new icon button with the given unique ID and fallback text.
    ///
    /// - `id`: Unique identifier for texture caching (must be unique per button instance).
    /// - `fallback_text`: Text to display if image loading fails (e.g., "☰").
    pub fn new(id: impl Into<String>, fallback_text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            internal_ui_id: weaver_lib::next_id(),
            current_path: None,
            load_attempted: false,
            texture: None,
            using_fallback: true,
            fallback_text: fallback_text.into(),
            size: 32.0,
            background_color: None,
            padding: 0.0,
            stroke: None,
            interactable: Interactable::new(),
        }
    }

    /// Set the button size (diameter in pixels).
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the background color for the circular button.
    pub fn with_background_color(mut self, color: egui::Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set the padding between the button edge and the image.
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the border stroke (color and width).
    pub fn with_stroke(mut self, stroke: egui::Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Set the background color at runtime.
    pub fn set_background_color(&mut self, color: egui::Color32) {
        self.background_color = Some(color);
    }

    /// Remove the background color.
    pub fn clear_background_color(&mut self) {
        self.background_color = None;
    }

    /// Set the button size at runtime.
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    /// Set the padding at runtime.
    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    /// Set the border stroke at runtime.
    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        self.stroke = Some(stroke);
    }

    /// Remove the border stroke.
    pub fn clear_stroke(&mut self) {
        self.stroke = None;
    }

    /// Returns whether an actual image is being used (not the fallback text).
    pub fn has_image(&self) -> bool {
        self.texture.is_some() && !self.using_fallback
    }

    /// Render the icon button and return whether it was clicked.
    ///
    /// - `ui`: The egui UI context.
    /// - `image_path`: Optional path to the icon image. If None or loading fails,
    ///   the fallback text is displayed.
    pub fn ui(&mut self, ui: &mut egui::Ui, image_path: Option<&Path>) -> egui::Response {
        // Check if path changed
        let path_changed = match (&self.current_path, image_path) {
            (Some(current), Some(new)) => current.as_path() != new,
            (None, Some(_)) | (Some(_), None) => true,
            (None, None) => false,
        };

        if path_changed {
            self.texture = None;
            self.load_attempted = false;
            self.using_fallback = true;
            self.current_path = image_path.map(|p| p.to_path_buf());
        }

        // Load texture if not cached and not yet attempted
        if self.texture.is_none() && !self.load_attempted {
            self.load_attempted = true;

            if let Some(path) = image_path {
                match load_image_from_path(path, self.size as u32) {
                    Ok(img) => {
                        self.using_fallback = false;
                        let texture = ui.ctx().load_texture(
                            &self.id,
                            img,
                            egui::TextureOptions::default(),
                        );
                        self.texture = Some(texture);
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to load icon image {:?}: {}. Using fallback text.",
                            path, e
                        );
                        self.using_fallback = true;
                    }
                }
            }
        }

        // Render the button
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(self.size, self.size),
            egui::Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = self.size / 2.0;

            // Draw circular background with press/hover state
            // Use interactable.is_pressed() to maintain pressed visual even when pointer leaves
            if let Some(bg_color) = self.background_color {
                let final_bg = if self.interactable.is_pressed() {
                    // Pressed: darken the background (persists even if pointer leaves)
                    darken_color(bg_color, 0.15)
                } else if response.hovered() {
                    // Hovered: slightly darken
                    darken_color(bg_color, 0.05)
                } else {
                    bg_color
                };
                painter.circle_filled(center, radius, final_bg);
            }

            // Draw border stroke
            if let Some(stroke) = self.stroke {
                painter.circle_stroke(center, radius, stroke);
            }

            if let Some(texture) = &self.texture {
                // Render the image clipped to a circle with padding
                // Image radius is smaller than button radius by the padding amount
                let image_radius = (radius - self.padding).max(0.0);
                
                // Create a circular mesh for the image
                let mut mesh = egui::Mesh::with_texture(texture.id());
                
                // Generate circle vertices
                const SEGMENTS: usize = 64;
                let center_uv = egui::pos2(0.5, 0.5);
                
                // Add center vertex
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: center,
                    uv: center_uv,
                    color: egui::Color32::WHITE,
                });
                
                // Add perimeter vertices
                for i in 0..=SEGMENTS {
                    let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
                    let (sin, cos) = angle.sin_cos();
                    
                    let pos = egui::pos2(
                        center.x + cos * image_radius,
                        center.y + sin * image_radius,
                    );
                    let uv_pos = egui::pos2(
                        0.5 + cos * 0.5,
                        0.5 + sin * 0.5,
                    );
                    
                    mesh.vertices.push(egui::epaint::Vertex {
                        pos,
                        uv: uv_pos,
                        color: egui::Color32::WHITE,
                    });
                }
                
                // Create triangles (fan from center)
                for i in 1..=SEGMENTS {
                    mesh.indices.push(0); // center
                    mesh.indices.push(i as u32);
                    mesh.indices.push((i % SEGMENTS + 1) as u32);
                }
                
                painter.add(egui::Shape::mesh(mesh));
            } else {
                // Render fallback text centered in the circle
                let font_id = egui::FontId::proportional(self.size * 0.6);
                let text_color = ui.style().visuals.text_color();
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    &self.fallback_text,
                    font_id,
                    text_color,
                );
            }
        }

        // Handle interactions (press/release/click signals)
        self.interactable.handle(self, ui, &response);

        response
    }
}

impl InteractableHandlers<Self> for IconButton {
    fn get_interactable_mut(&mut self) -> &mut Interactable<Self> {
        &mut self.interactable
    }
}

/// Darken a color by a factor (0.0 = no change, 1.0 = black).
fn darken_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    let factor = 1.0 - factor.clamp(0.0, 1.0);
    egui::Color32::from_rgba_unmultiplied(
        (color.r() as f32 * factor) as u8,
        (color.g() as f32 * factor) as u8,
        (color.b() as f32 * factor) as u8,
        color.a(),
    )
}

/// Error type for image loading.
#[derive(Debug)]
pub enum ImageLoadError {
    Io(std::io::Error),
    Image(image::ImageError),
    Svg(String),
}

impl std::fmt::Display for ImageLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageLoadError::Io(e) => write!(f, "IO error: {}", e),
            ImageLoadError::Image(e) => write!(f, "Image error: {}", e),
            ImageLoadError::Svg(e) => write!(f, "SVG error: {}", e),
        }
    }
}

impl From<std::io::Error> for ImageLoadError {
    fn from(e: std::io::Error) -> Self {
        ImageLoadError::Io(e)
    }
}

impl From<image::ImageError> for ImageLoadError {
    fn from(e: image::ImageError) -> Self {
        ImageLoadError::Image(e)
    }
}

/// Load an image from a file path into an egui ColorImage.
///
/// Supports PNG, JPEG, and SVG formats. SVG images are rasterized at the specified size.
fn load_image_from_path(path: &Path, size: u32) -> Result<egui::ColorImage, ImageLoadError> {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    
    if extension.eq_ignore_ascii_case("svg") {
        load_svg_from_path(path, size)
    } else {
        let image = image::ImageReader::open(path)?.decode()?;
        let size = [image.width() as usize, image.height() as usize];
        let rgba = image.into_rgba8();
        Ok(egui::ColorImage::from_rgba_unmultiplied(size, &rgba))
    }
}

/// Load and rasterize an SVG file at the specified size.
fn load_svg_from_path(path: &Path, size: u32) -> Result<egui::ColorImage, ImageLoadError> {
    let svg_data = std::fs::read(path)?;
    
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &options)
        .map_err(|e| ImageLoadError::Svg(e.to_string()))?;
    
    let original_size = tree.size();
    let scale = size as f32 / original_size.width().max(original_size.height());
    
    let scaled_width = (original_size.width() * scale).ceil() as u32;
    let scaled_height = (original_size.height() * scale).ceil() as u32;
    
    let mut pixmap = resvg::tiny_skia::Pixmap::new(scaled_width, scaled_height)
        .ok_or_else(|| ImageLoadError::Svg("Failed to create pixmap".to_string()))?;
    
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    let pixels = pixmap.data();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        [scaled_width as usize, scaled_height as usize],
        pixels,
    ))
}
