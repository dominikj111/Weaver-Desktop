//! Background image component - renders an image behind all other UI elements.

use std::path::{Path, PathBuf};

/// Size of the fallback gradient texture (reasonable quality without being huge).
const FALLBACK_SIZE: usize = 512;

/// Background image component that loads and renders an image to the background layer.
///
/// The image is loaded from disk on first use or when the path changes,
/// and cached as a texture for efficient rendering. If loading fails,
/// a fallback gradient is generated once and used instead.
pub struct Background {
    /// Whether background rendering is enabled (can be disabled for performance).
    enabled: bool,
    /// Current image path being displayed.
    current_path: Option<PathBuf>,
    /// Whether we already attempted to load the current path (prevents repeated errors).
    load_attempted: bool,
    /// Cached texture handle (image or fallback gradient).
    texture: Option<egui::TextureHandle>,
    /// Whether we're using the fallback gradient (affects rendering layer).
    using_fallback: bool,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            enabled: true,
            current_path: None,
            load_attempted: false,
            texture: None,
            using_fallback: false,
        }
    }
}

impl Background {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable background rendering.
    /// Disabling can improve performance on resource-constrained devices.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns whether background rendering is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns whether an actual image is being used (not the fallback gradient).
    /// Useful for deciding whether panels should be transparent.
    pub fn has_image(&self) -> bool {
        self.texture.is_some() && !self.using_fallback
    }

    /// Render the background image. Call this before any other UI rendering.
    ///
    /// - `ctx`: The egui context
    /// - `image_path`: Optional path to the background image. If None or loading fails,
    ///   a fallback gradient is rendered.
    pub fn ui(&mut self, ctx: &egui::Context, image_path: Option<&Path>) {
        if !self.enabled {
            return;
        }

        // Check if path changed
        let path_changed = match (&self.current_path, image_path) {
            (Some(current), Some(new)) => current.as_path() != new,
            (None, Some(_)) | (Some(_), None) => true,
            (None, None) => false,
        };

        if path_changed {
            self.texture = None;
            self.load_attempted = false;
            self.using_fallback = false;
            self.current_path = image_path.map(|p| p.to_path_buf());
        }

        // Load texture if not cached and not yet attempted
        if self.texture.is_none() && !self.load_attempted {
            self.load_attempted = true;

            let color_image = if let Some(path) = image_path {
                match load_image_from_path(path) {
                    Ok(img) => {
                        self.using_fallback = false;
                        img
                    }
                    Err(e) => {
                        eprintln!("Failed to load background image {:?}: {}. Using fallback gradient.", path, e);
                        self.using_fallback = true;
                        generate_fallback_gradient(FALLBACK_SIZE)
                    }
                }
            } else {
                self.using_fallback = true;
                generate_fallback_gradient(FALLBACK_SIZE)
            };

            let texture = ctx.load_texture(
                "background",
                color_image,
                egui::TextureOptions::default(),
            );
            self.texture = Some(texture);
        }

        // Render the background
        if let Some(texture) = &self.texture {
            let screen_rect = ctx.screen_rect();
            let screen_size = screen_rect.size();
            let tex_size = texture.size_vec2();

            // Calculate UV rect for "cover" behavior:
            // - Maintain aspect ratio
            // - Fill the screen (crop overflow)
            // - Center the image
            let screen_aspect = screen_size.x / screen_size.y;
            let tex_aspect = tex_size.x / tex_size.y;

            let uv = if screen_aspect > tex_aspect {
                // Screen is wider than image - crop top/bottom
                let visible_height = tex_aspect / screen_aspect;
                let y_offset = (1.0 - visible_height) / 2.0;
                egui::Rect::from_min_max(
                    egui::pos2(0.0, y_offset),
                    egui::pos2(1.0, y_offset + visible_height),
                )
            } else {
                // Screen is taller than image - crop left/right
                let visible_width = screen_aspect / tex_aspect;
                let x_offset = (1.0 - visible_width) / 2.0;
                egui::Rect::from_min_max(
                    egui::pos2(x_offset, 0.0),
                    egui::pos2(x_offset + visible_width, 1.0),
                )
            };

            // Use background layer - this renders below all panels
            let painter = ctx.layer_painter(egui::LayerId::background());
            painter.image(texture.id(), screen_rect, uv, egui::Color32::WHITE);
        }
    }

    /// Returns whether we're using the fallback gradient (not an actual image).
    pub fn is_fallback(&self) -> bool {
        self.using_fallback
    }
}

/// Load an image from a file path into an egui ColorImage.
fn load_image_from_path(path: &Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::ImageReader::open(path)?.decode()?;
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.into_rgba8();
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &image_buffer))
}

/// Generate a fallback gradient background with a subtle rainbow arc (bow shape).
/// Colors follow real rainbow order: red (outer) to violet (inner).
/// The arc rises from the bottom, blending into a base color matching egui's dark theme.
fn generate_fallback_gradient(size: usize) -> egui::ColorImage {
    let mut pixels = Vec::with_capacity(size * size);
    
    // Base color matching egui's dark theme background (Color32::from_gray(27))
    let base = egui::Color32::LIGHT_BLUE;
    
    // Rainbow arc parameters - center well below the bottom for a gentle arc
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 * 1.4; // Further below for flatter arc
    let arc_inner = size as f32 * 0.85;  // Inner radius of rainbow
    let arc_outer = size as f32 * 1.05;  // Outer radius - thinner band
    let arc_width = arc_outer - arc_inner;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx);
            
            // Only render in the upper semicircle (angle between -PI and 0, i.e., above center)
            let in_angle_range = angle < -0.1 && angle > -std::f32::consts::PI + 0.1;
            
            // Check if we're in the radial range
            let in_radial_range = dist >= arc_inner && dist <= arc_outer;
            
            if in_angle_range && in_radial_range {
                // Calculate position within the rainbow band (0 = inner/violet, 1 = outer/red)
                let band_pos = (dist - arc_inner) / arc_width;
                
                // Rainbow hue: 0 (red) at outer edge, ~0.75 (violet) at inner edge
                let hue = (1.0 - band_pos) * 0.75;
                let (hr, hg, hb) = hue_to_rgb(hue);
                
                // Calculate fade at radial edges (smooth blend to base)
                let radial_fade = {
                    let edge_dist = (band_pos - 0.5).abs() * 2.0; // 0 at center, 1 at edges
                    let fade = 1.0 - (edge_dist * edge_dist * edge_dist); // Cubic falloff
                    fade.max(0.0)
                };
                
                // Angular fade at the ends of the arc (near horizontal)
                let angular_fade = {
                    // Normalize angle: 0 at horizontal edges, 1 at top
                    let norm = (angle + std::f32::consts::PI / 2.0).abs() / (std::f32::consts::PI / 2.0);
                    let fade = (1.0 - norm * 1.5).max(0.0); // Faster fade at edges
                    fade
                };
                
                // Blend rainbow with base color (max 25% rainbow intensity)
                let intensity = radial_fade * angular_fade * 0.25;
                let inv = 1.0 - intensity;
                let r = (base.r() as f32 * inv + hr * 255.0 * intensity) as u8;
                let g = (base.g() as f32 * inv + hg * 255.0 * intensity) as u8;
                let b = (base.b() as f32 * inv + hb * 255.0 * intensity) as u8;
                
                pixels.push(egui::Color32::from_rgb(r, g, b));
            } else {
                // Base color (opaque) - matches theme background
                pixels.push(base);
            }
        }
    }

    egui::ColorImage::new([size, size], pixels)
}

/// Convert hue (0.0-1.0) to RGB (each 0.0-1.0).
fn hue_to_rgb(hue: f32) -> (f32, f32, f32) {
    let h = hue * 6.0;
    let c = 1.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());

    match h as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    }
}
