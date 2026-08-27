//! Application Shell - Widget-based desktop environment.
//!
//! The Shell manages the desktop structure as a widget tree with layered rendering:
//!
//! - Layer 0: Background (ImageSurface)
//! - Layer 1: Desktop widget tree (bars, content area)
//! - Layer 2: Modal (app menu, dialogs)
//! - Layer 3: Toasts (notifications)
//!
//! # Example
//!
//! ```rust,ignore
//! let mut shell = DesktopShell::new();
//! shell.set_background_image("path/to/wallpaper.jpg");
//!
//! // In update loop
//! shell.ui(ctx, |ui| {
//!     // View content here
//! });
//! ```

use std::path::{Path, PathBuf};

use egui::{Align2, Color32, Rect, Vec2};
use egui_toast::Toasts;

use super::modal::{Modal, ModalResult};
use super::widget::{
    Align, Container, ImageId, Justify, Label, RenderContext, Size, Spacing, Style, Widget, pos2,
    vec2,
};
use super::{ImageSource, ImageSurface, ScaleMode};

/// Clock widget content - displays current time.
pub struct ClockWidget {
    id: String,
    style: Style,
    format: String,
}

impl ClockWidget {
    pub fn new() -> Self {
        Self {
            id: "clock".to_string(),
            style: Style::new(),
            format: "%I:%M %p".to_string(),
        }
    }

    pub fn with_format(format: impl Into<String>) -> Self {
        Self {
            id: "clock".to_string(),
            style: Style::new(),
            format: format.into(),
        }
    }
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ClockWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(80.0, 20.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        let now = chrono::Local::now();
        let time_str = now.format(&self.format).to_string();
        ctx.paint_text(rect.center(), Align2::CENTER_CENTER, &time_str, 14.0, Color32::WHITE);
    }
}

/// Date widget content - displays current date.
pub struct DateWidget {
    id: String,
    style: Style,
    format: String,
}

impl DateWidget {
    pub fn new() -> Self {
        Self {
            id: "date".to_string(),
            style: Style::new(),
            format: "%A, %B %d".to_string(),
        }
    }
}

impl Default for DateWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for DateWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(180.0, 20.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        let now = chrono::Local::now();
        let date_str = now.format(&self.format).to_string();
        let response = ctx.interact(rect, self.id());
        let color = if response.hovered {
            Color32::from_gray(180)
        } else {
            Color32::WHITE
        };
        ctx.paint_text(rect.center(), Align2::CENTER_CENTER, &date_str, 14.0, color);
        if response.clicked {
            // TODO: toggle calendar
        }
    }
}

/// Menu button widget - triggers modal.
pub struct MenuButton {
    id: String,
    style: Style,
    icon: String,
    size: f32,
}

impl MenuButton {
    pub fn new() -> Self {
        Self {
            id: "menu_button".to_string(),
            style: Style::new(),
            icon: "☰".to_string(),
            size: 40.0,
        }
    }

    pub fn with_icon(icon: impl Into<String>) -> Self {
        Self {
            id: "menu_button".to_string(),
            style: Style::new(),
            icon: icon.into(),
            size: 40.0,
        }
    }
}

impl Default for MenuButton {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MenuButton {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(self.size, self.size)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        let response = ctx.interact(rect, self.id());

        let bg_color = if response.hovered {
            Color32::from_gray(60)
        } else {
            Color32::from_gray(40)
        };

        ctx.paint_rect(rect, 8.0, bg_color);
        ctx.paint_text(rect.center(), Align2::CENTER_CENTER, &self.icon, 20.0, Color32::WHITE);

        if response.clicked {
            // TODO: emit an `activated` event via the typed event channel.
        }
    }
}

/// Windows XP Start button - displays the iconic green Start button image.
pub struct XpStartButton {
    /// Path to the start button image
    image_path: PathBuf,
    /// Cached texture handle
    texture: Option<egui::TextureHandle>,
    /// Whether we've attempted to load the texture
    load_attempted: bool,
    /// Button dimensions (scaled to match taskbar)
    size: Vec2,
    /// Target height to scale to (matches taskbar height)
    target_height: f32,
}

impl XpStartButton {
    pub fn with_height(image_path: impl Into<PathBuf>, height: f32) -> Self {
        Self {
            image_path: image_path.into(),
            texture: None,
            load_attempted: false,
            size: Vec2::new(97.0, height),
            target_height: height,
        }
    }

    fn load_texture(&mut self, ctx: &egui::Context) {
        if self.load_attempted {
            return;
        }
        self.load_attempted = true;

        if self.image_path.exists() {
            if let Ok(image) = image::open(&self.image_path) {
                let rgba = image.to_rgba8();
                let width = rgba.width() as usize;
                let height = rgba.height() as usize;

                // The sprite has 3 buttons stacked vertically, use only the first one
                let single_button_height = height / 3;

                // Extract just the first button (top portion)
                let mut first_button_pixels = Vec::with_capacity(width * single_button_height * 4);
                for y in 0..single_button_height {
                    for x in 0..width {
                        let pixel = rgba.get_pixel(x as u32, y as u32);
                        first_button_pixels.extend_from_slice(&pixel.0);
                    }
                }

                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [width, single_button_height],
                    &first_button_pixels,
                );

                // Calculate scaled size to match target height while preserving aspect ratio
                let original_width = width as f32;
                let original_height = single_button_height as f32;
                let scale = self.target_height / original_height;
                self.size = Vec2::new(original_width * scale, self.target_height);

                self.texture = Some(ctx.load_texture(
                    "xp_start_button",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));
            }
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Ensure texture is loaded
        self.load_texture(ui.ctx());

        let (rect, response) = ui.allocate_exact_size(self.size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            if let Some(ref texture) = self.texture {
                // Draw the button image
                let mut tint = Color32::WHITE;
                if response.hovered() {
                    // Slight brightness boost on hover
                    tint = Color32::from_rgb(255, 255, 240);
                }
                if response.is_pointer_button_down_on() {
                    // Slight darkening when pressed
                    tint = Color32::from_rgb(220, 220, 220);
                }

                ui.painter().image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    tint,
                );
            } else {
                // Fallback: draw a green button with "Start" text
                let bg_color = if response.hovered() {
                    Color32::from_rgb(90, 190, 70)
                } else {
                    Color32::from_rgb(70, 170, 50)
                };
                ui.painter().rect_filled(rect, 4.0, bg_color);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "Start",
                    egui::FontId::proportional(14.0),
                    Color32::WHITE,
                );
            }
        }

        // Store click in response - caller checks via context
        if response.clicked() {
            ui.ctx().memory_mut(|mem| {
                mem.data.insert_temp(egui::Id::new("start_clicked"), true);
            });
        }
    }
}

/// Status text widget - displays simple status text.
pub struct StatusText {
    id: String,
    style: Style,
    text: String,
}

impl StatusText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: "status_text".to_string(),
            style: Style::new(),
            text: text.into(),
        }
    }
}

impl Widget for StatusText {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(self.text.len() as f32 * 8.0, 20.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        ctx.paint_text(rect.left_center(), Align2::LEFT_CENTER, &self.text, 14.0, Color32::WHITE);
    }
}

/// Version label widget.
pub struct VersionLabel {
    id: String,
    style: Style,
    version: String,
}

impl VersionLabel {
    pub fn new() -> Self {
        Self {
            id: "version".to_string(),
            style: Style::new(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for VersionLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for VersionLabel {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(60.0, 20.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        ctx.paint_text(
            rect.center(),
            Align2::CENTER_CENTER,
            &format!("v{}", self.version),
            14.0,
            Color32::WHITE,
        );
    }
}

/// Windows XP style taskbar widget - draws the iconic blue gradient bar.
pub struct XpTaskbar {
    id: String,
    style: Style,
    height: f32,
}

impl XpTaskbar {
    pub fn new() -> Self {
        Self {
            id: "xp_taskbar".to_string(),
            style: Style::new(),
            height: 30.0,
        }
    }

    pub fn with_height(height: f32) -> Self {
        Self {
            id: "xp_taskbar".to_string(),
            style: Style::new(),
            height,
        }
    }

    /// Draw the Windows XP taskbar gradient.
    fn paint_xp_gradient(ctx: &mut dyn RenderContext, rect: Rect) {
        // Windows XP taskbar colors (from top to bottom):
        // - Top highlight line: #4580C4 (light blue)
        // - Main gradient: #245EDC -> #1941A5 (bright to dark blue)
        // - Bottom edge: #18399A (dark blue)

        let top_highlight = Color32::from_rgb(0x45, 0x80, 0xC4);
        let gradient_top = Color32::from_rgb(0x24, 0x5E, 0xDC);
        let gradient_bottom = Color32::from_rgb(0x19, 0x41, 0xA5);
        let bottom_edge = Color32::from_rgb(0x18, 0x39, 0x9A);

        let height = rect.height();
        let highlight_height = 2.0;
        let edge_height = 1.0;
        let gradient_height = height - highlight_height - edge_height;

        // Draw top highlight line
        let highlight_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), highlight_height));
        ctx.paint_rect(highlight_rect, 0.0, top_highlight);

        // Draw main gradient (we'll use horizontal strips to simulate vertical gradient)
        let gradient_start_y = rect.min.y + highlight_height;
        let num_strips = gradient_height.ceil() as i32;

        for i in 0..num_strips {
            let t = i as f32 / (num_strips - 1).max(1) as f32;
            let color = Self::lerp_color(gradient_top, gradient_bottom, t);
            let strip_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, gradient_start_y + i as f32),
                egui::vec2(rect.width(), 1.0),
            );
            ctx.paint_rect(strip_rect, 0.0, color);
        }

        // Draw bottom edge line
        let edge_rect = egui::Rect::from_min_size(
            egui::pos2(rect.min.x, rect.max.y - edge_height),
            egui::vec2(rect.width(), edge_height),
        );
        ctx.paint_rect(edge_rect, 0.0, bottom_edge);
    }

    /// Linear interpolation between two colors.
    fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0);
        Color32::from_rgb(
            (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
            (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
            (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        )
    }
}

impl Default for XpTaskbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for XpTaskbar {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(100.0, self.height)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        Self::paint_xp_gradient(ctx, rect);
    }
}

/// Windows XP-style clock widget with the iconic tray gradient background.
pub struct XpClock {
    id: String,
    style: Style,
    height: f32,
}

impl XpClock {
    pub fn new() -> Self {
        Self {
            id: "xp_clock".to_string(),
            style: Style::new(),
            height: 30.0,
        }
    }

    pub fn with_height(height: f32) -> Self {
        Self {
            id: "xp_clock".to_string(),
            style: Style::new(),
            height,
        }
    }

    /// Paint the XP taskbar tray gradient background.
    /// The tray area has a slightly different gradient than the main taskbar.
    fn paint_tray_gradient(ctx: &mut dyn RenderContext, rect: Rect) {
        // Windows XP system tray colors (slightly lighter/different than taskbar)
        // Top edge highlight
        let top_highlight = Color32::from_rgb(0x5F, 0x9D, 0xF7);
        // Main gradient
        let gradient_top = Color32::from_rgb(0x31, 0x6A, 0xC5);
        let gradient_bottom = Color32::from_rgb(0x1F, 0x4A, 0xAE);
        // Bottom edge
        let bottom_edge = Color32::from_rgb(0x18, 0x39, 0x9A);

        let height = rect.height();
        let highlight_height = 1.0;
        let edge_height = 1.0;
        let gradient_height = height - highlight_height - edge_height;

        // Draw top highlight line
        let highlight_rect =
            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), highlight_height));
        ctx.paint_rect(highlight_rect, 0.0, top_highlight);

        // Draw main gradient
        let gradient_start_y = rect.min.y + highlight_height;
        let num_strips = gradient_height.ceil() as i32;

        for i in 0..num_strips {
            let t = i as f32 / (num_strips - 1).max(1) as f32;
            let color = XpTaskbar::lerp_color(gradient_top, gradient_bottom, t);
            let strip_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, gradient_start_y + i as f32),
                egui::vec2(rect.width(), 1.0),
            );
            ctx.paint_rect(strip_rect, 0.0, color);
        }

        // Draw bottom edge line
        let edge_rect = egui::Rect::from_min_size(
            egui::pos2(rect.min.x, rect.max.y - edge_height),
            egui::vec2(rect.width(), edge_height),
        );
        ctx.paint_rect(edge_rect, 0.0, bottom_edge);

        // Draw left separator line (distinguishes from main taskbar)
        let separator_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(1.0, height),
        );
        ctx.paint_rect(separator_rect, 0.0, Color32::from_rgb(0x0C, 0x34, 0x75));
    }
}

impl Default for XpClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for XpClock {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(85.0, self.height)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        // Paint the tray gradient background
        Self::paint_tray_gradient(ctx, rect);

        // Draw the time text centered
        let now = chrono::Local::now();
        let time_str = now.format("%I:%M %p").to_string();

        let text_rect = rect.shrink(4.0); // Padding inside the tray
        ctx.paint_text(
            text_rect.center(),
            Align2::CENTER_CENTER,
            &time_str,
            13.0,
            Color32::WHITE,
        );
    }
}

// ============================================================================
// Desktop Widgets (for placement on the desktop surface)
// ============================================================================

/// A single icon entry for the desktop.
#[derive(Clone)]
pub struct DesktopIcon {
    /// Display label
    pub label: String,
    /// Path to icon image
    pub icon_path: Option<PathBuf>,
    /// Callback ID when clicked
    pub action_id: String,
}

impl DesktopIcon {
    pub fn new(label: impl Into<String>, action_id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon_path: None,
            action_id: action_id.into(),
        }
    }

    pub fn with_icon(mut self, path: impl Into<PathBuf>) -> Self {
        self.icon_path = Some(path.into());
        self
    }
}

/// Desktop icon grid widget - displays icons in a grid layout.
pub struct IconGridWidget {
    id: String,
    style: Style,
    icons: Vec<DesktopIcon>,
    icon_size: f32,
    spacing: f32,
    columns: usize,
    /// Cached image handles for icons (resolved by the backend facade).
    textures: Vec<Option<ImageId>>,
    /// Track which textures we've tried to load
    load_attempted: Vec<bool>,
}

impl IconGridWidget {
    pub fn new() -> Self {
        Self {
            id: "icon_grid".to_string(),
            style: Style::new(),
            icons: Vec::new(),
            icon_size: 48.0,
            spacing: 16.0,
            columns: 3,
            textures: Vec::new(),
            load_attempted: Vec::new(),
        }
    }

    pub fn with_icons(mut self, icons: Vec<DesktopIcon>) -> Self {
        let count = icons.len();
        self.icons = icons;
        self.textures = vec![None; count];
        self.load_attempted = vec![false; count];
        self
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }

}

impl Default for IconGridWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for IconGridWidget {
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
        let rows = (self.icons.len() + self.columns - 1) / self.columns.max(1);
        let cell_size = self.icon_size + self.spacing;
        Vec2::new(self.columns as f32 * cell_size, rows as f32 * cell_size)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        let cell_size = self.icon_size + self.spacing;
        let actual_cols = ((rect.width() / cell_size) as usize).max(1).min(self.columns);

        for (i, icon) in self.icons.iter().enumerate() {
            let row = i / actual_cols;
            let col = i % actual_cols;
            let cell_min = pos2(
                rect.min.x + col as f32 * cell_size,
                rect.min.y + row as f32 * cell_size,
            );
            let cell_rect = Rect::from_min_size(cell_min, vec2(cell_size, cell_size));

            // Load the icon image once, lazily.
            if !self.load_attempted[i] {
                self.load_attempted[i] = true;
                if let Some(ref path) = icon.icon_path {
                    if path.exists() {
                        let id = ctx.load_image(&format!("desktop_icon_{}", i), path);
                        self.textures[i] = Some(id);
                    }
                }
            }

            let icon_rect = Rect::from_min_size(cell_min, vec2(self.icon_size, self.icon_size));
            if let Some(id) = self.textures[i] {
                ctx.paint_image(icon_rect, id, Color32::WHITE);
            } else {
                ctx.paint_rect(icon_rect, 8.0, Color32::from_gray(60));
                ctx.paint_text(
                    icon_rect.center(),
                    Align2::CENTER_CENTER,
                    "📁",
                    24.0,
                    Color32::WHITE,
                );
            }

            // Label below the icon
            let label_pos = pos2(icon_rect.center().x, icon_rect.max.y + 8.0);
            ctx.paint_text(label_pos, Align2::CENTER_TOP, &icon.label, 11.0, Color32::WHITE);

            // Handle click
            let response = ctx.interact(cell_rect, &format!("icon_{}", i));
            if response.clicked {
                println!("Desktop icon clicked: {}", icon.action_id);
            }
        }
    }
}

/// Desktop image widget - displays an image (like a photo frame).
pub struct DesktopImageWidget {
    id: String,
    style: Style,
    source: ImageSource,
    border_radius: f32,
    title: Option<String>,
}

impl DesktopImageWidget {
    pub fn new() -> Self {
        Self {
            id: "desktop_image".to_string(),
            style: Style::new(),
            source: ImageSource::None,
            border_radius: 12.0,
            title: None,
        }
    }

    pub fn with_image(mut self, path: impl Into<PathBuf>) -> Self {
        self.source = ImageSource::Image(path.into());
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.source = ImageSource::Color(color);
        self
    }

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Scale mode is a render concern handled by the backend facade (stretch for
    /// now; `Cover` cropping is deferred). Kept for API compatibility.
    pub fn scale_mode(self, _mode: ScaleMode) -> Self {
        self
    }
}

impl Default for DesktopImageWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for DesktopImageWidget {
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
        Vec2::new(120.0, 90.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        // Draw background/border
        ctx.paint_rect(rect, self.border_radius, Color32::from_gray(40));

        // Draw image (inset for border effect)
        let image_rect = rect.shrink(4.0);
        match &self.source {
            ImageSource::None => {}
            ImageSource::Color(color) => ctx.paint_rect(image_rect, 0.0, *color),
            ImageSource::Image(path) => {
                let id = ctx.load_image("desktop_image", path);
                ctx.paint_image(image_rect, id, Color32::WHITE);
            }
        }

        // Draw title if present
        if let Some(ref title) = self.title {
            let title_rect = Rect::from_min_size(
                pos2(rect.min.x, rect.max.y - 28.0),
                vec2(rect.width(), 28.0),
            );
            ctx.paint_rect(title_rect, 0.0, Color32::from_black_alpha(180));
            ctx.paint_text(title_rect.center(), Align2::CENTER_CENTER, title, 12.0, Color32::WHITE);
        }
    }
}

// ============================================================================
// Desktop Shell
// ============================================================================

/// The application shell using Widget-based layout.
pub struct DesktopShell {
    /// Layer 0: Background surface
    background: ImageSurface,

    /// Layer 1: Desktop widget tree
    desktop: Container,

    /// Layer 2: Active modal (if any)
    modal: Option<Modal>,

    /// Layer 3: Toast notifications
    toasts: Toasts,

    /// Whether desktop is disabled (dimmed for modal)
    desktop_disabled: bool,

    /// Image textures loaded by the render facade (persists across frames).
    textures: super::widget::TextureRegistry,
}

impl Default for DesktopShell {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopShell {
    /// Create a new desktop shell with default layout.
    pub fn new() -> Self {
        let desktop = Self::build_desktop_widget(Vec::new());

        Self {
            background: ImageSurface::with_id("desktop_background"),
            desktop,
            modal: None,
            toasts: Toasts::new()
                .anchor(Align2::RIGHT_TOP, (-10.0, 60.0))
                .direction(egui::Direction::TopDown),
            desktop_disabled: false,
            textures: super::widget::TextureRegistry::new(),
        }
    }

    /// Create a new desktop shell with content widgets.
    pub fn with_content(content_widgets: Vec<Box<dyn Widget>>) -> Self {
        let desktop = Self::build_desktop_widget(content_widgets);

        Self {
            background: ImageSurface::with_id("desktop_background"),
            desktop,
            modal: None,
            toasts: Toasts::new()
                .anchor(Align2::RIGHT_TOP, (-10.0, 60.0))
                .direction(egui::Direction::TopDown),
            desktop_disabled: false,
            textures: super::widget::TextureRegistry::new(),
        }
    }

    /// Set the content area widgets.
    pub fn set_content(&mut self, content_widgets: Vec<Box<dyn Widget>>) {
        self.desktop = Self::build_desktop_widget(content_widgets);
    }

    /// Build the desktop widget tree.
    fn build_desktop_widget(content_widgets: Vec<Box<dyn Widget>>) -> Container {
        // Build content area with provided widgets
        let mut content_area = Container::row("content-area")
            .height(Size::Flex(1.0))
            .padding(Spacing::all(16.0))
            .gap(16.0)
            .align(Align::Start);

        for widget in content_widgets {
            content_area = content_area.child(widget);
        }

        let taskbar_row = Container::row("xp-taskbar-row")
            .height(Size::Fixed(30.0))
            .child_sized(
                Box::new(XpTaskbar::with_height(30.0)),
                Size::Flex(1.0),
                Size::Fixed(30.0),
            )
            .child_sized(
                Box::new(XpClock::with_height(30.0)),
                Size::Fixed(85.0),
                Size::Fixed(30.0),
            );

        Container::column("desktop")
            .child(Box::new(content_area))
            .child_sized(Box::new(taskbar_row), Size::Flex(1.0), Size::Fixed(30.0))
    }

    /// Set the background image.
    pub fn set_background_image(&mut self, path: impl AsRef<Path>) {
        self.background
            .set_source(ImageSource::Image(path.as_ref().to_path_buf()));
    }

    /// Set background color.
    pub fn set_background_color(&mut self, color: Color32) {
        self.background.set_source(ImageSource::Color(color));
    }

    /// Show the app menu modal.
    pub fn show_app_menu(&mut self, content: Box<dyn Widget>) {
        self.modal = Some(Modal::new(content).max_size_percent(0.85, 0.85));
        self.desktop_disabled = true;
    }

    /// Close any open modal.
    pub fn close_modal(&mut self) {
        self.modal = None;
        self.desktop_disabled = false;
    }

    /// Check if modal is open.
    pub fn is_modal_open(&self) -> bool {
        self.modal.is_some()
    }

    /// Get mutable access to the desktop widget for customization.
    pub fn desktop_mut(&mut self) -> &mut Container {
        &mut self.desktop
    }

    /// Render the shell.
    pub fn ui(&mut self, ui: &mut egui::Ui, _view: impl FnOnce(&mut egui::Ui)) {
        let ctx = ui.ctx().clone();
        let screen_rect = ctx.input(|i| i.viewport_rect());

        // Layer 0: Background
        self.background.paint_background(&ctx, screen_rect);

        // Check for menu button click
        let menu_clicked = ctx.memory_mut(|mem| {
            mem.data
                .get_temp::<bool>(egui::Id::new("menu_clicked"))
                .unwrap_or(false)
        });
        if menu_clicked {
            // Clear the flag
            ctx.memory_mut(|mem| {
                mem.data.insert_temp(egui::Id::new("menu_clicked"), false);
            });
            // Toggle modal
            if self.modal.is_some() {
                self.close_modal();
            } else {
                // Create app menu content
                self.show_app_menu(Box::new(Self::build_app_menu()));
            }
        }

        // Layer 1: Desktop widget tree
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                // Render desktop (layout caching: compute only when dirty)
                let rect = ui.max_rect();
                if self.desktop.needs_layout(rect) {
                    self.desktop.compute_layout(rect);
                }
                let mut render_ctx =
                    super::widget::EguiRenderContext::new(ui, &mut self.textures);
                self.desktop.render(&mut render_ctx, rect);

                // Draw disabled overlay if modal is active
                if self.desktop_disabled {
                    let screen = ui.ctx().input(|i| i.viewport_rect());
                    ui.painter()
                        .rect_filled(screen, 0.0, Color32::from_black_alpha(180));
                }
            });

        // Floating menu button (always on top of desktop, below modal)
        // egui::Area::new(egui::Id::new("floating_menu_button"))
        //     .order(egui::Order::Middle)
        //     .fixed_pos(egui::pos2(screen_rect.right() - 52.0, 10.0))
        //     .interactable(!self.desktop_disabled)
        //     .show(ctx, |ui| {
        //         let mut btn = MenuButton::new();
        //         btn.ui(ui);
        //     });

        // XP Start button (bottom-left, on top of taskbar)
        let taskbar_height = 30.0;
        egui::Area::new(egui::Id::new("xp_start_button"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(0.0, screen_rect.bottom() - taskbar_height))
            // .interactable(!self.desktop_disabled)
            .show(&ctx, |ui| {
                let mut btn = XpStartButton::with_height("assets/xp_start.png", taskbar_height);
                btn.ui(ui);
            });

        // Layer 2: Modal
        if let Some(ref mut modal) = self.modal {
            match modal.ui(&ctx, &mut self.textures) {
                ModalResult::Active => {}
                ModalResult::Dismissed => {
                    self.close_modal();
                }
            }
        }

        // Layer 3: Toasts
        self.toasts.show(ui);
    }

    /// Build the app menu widget.
    fn build_app_menu() -> Container {
        let menu_header = Container::row("menu-header")
            .height(Size::Fixed(40.0))
            .justify(Justify::Center)
            .child_sized(
                Box::new(Label::new("App Menu").with_id("menu-title")),
                Size::Content,
                Size::Fixed(40.0),
            );

        let menu_row_1 = Container::row("menu-row-1")
            .height(Size::Fixed(80.0))
            .gap(16.0)
            .justify(Justify::Center)
            .child(Self::menu_item("🏠", "Dashboard"))
            .child(Self::menu_item("🔧", "Hardware"))
            .child(Self::menu_item("📋", "Profiles"));

        let menu_row_2 = Container::row("menu-row-2")
            .height(Size::Fixed(80.0))
            .gap(16.0)
            .justify(Justify::Center)
            .child(Self::menu_item("📦", "System"))
            .child(Self::menu_item("📁", "Files"))
            .child(Self::menu_item("⚙", "Settings"));

        let power_row = Container::row("power-row")
            .height(Size::Fixed(60.0))
            .gap(16.0)
            .justify(Justify::Center)
            .margin(Spacing::new(24.0, 0.0, 0.0, 0.0))
            .child(Self::power_button("🔄", "Restart"))
            .child(Self::power_button("⏻", "Shutdown"));

        Container::column("app-menu")
            .padding(Spacing::all(24.0))
            .gap(16.0)
            .align(Align::Stretch)
            .child(Box::new(menu_header))
            .child(Box::new(menu_row_1))
            .child(Box::new(menu_row_2))
            .child(Box::new(power_row))
    }

    fn menu_item(icon: &str, label: &str) -> Box<dyn Widget> {
        let mut item = MenuItemContent::new(icon, label);
        item.style_mut().width = Size::Fixed(100.0);
        item.style_mut().height = Size::Fixed(80.0);
        Box::new(item)
    }

    fn power_button(icon: &str, label: &str) -> Box<dyn Widget> {
        let mut btn = PowerButtonContent::new(icon, label);
        btn.style_mut().width = Size::Fixed(80.0);
        btn.style_mut().height = Size::Fixed(50.0);
        Box::new(btn)
    }
}

/// Menu item content widget.
struct MenuItemContent {
    id: String,
    style: Style,
    icon: String,
    label: String,
}

impl MenuItemContent {
    fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: "menu_item".to_string(),
            style: Style::new(),
            icon: icon.into(),
            label: label.into(),
        }
    }
}

impl Widget for MenuItemContent {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(80.0, 70.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        let response = ctx.interact(rect, self.id());

        let bg_color = if response.hovered {
            Color32::from_gray(60)
        } else {
            Color32::from_gray(45)
        };

        ctx.paint_rect(rect, 12.0, bg_color);

        // Icon
        let icon_pos = pos2(rect.center().x, rect.center().y - 10.0);
        ctx.paint_text(icon_pos, Align2::CENTER_CENTER, &self.icon, 28.0, Color32::WHITE);

        // Label
        let label_pos = pos2(rect.center().x, rect.max.y - 12.0);
        ctx.paint_text(label_pos, Align2::CENTER_CENTER, &self.label, 12.0, Color32::from_gray(200));

        if response.clicked {
            println!("Menu item clicked: {}", self.label);
        }
    }
}

/// Power button content widget.
struct PowerButtonContent {
    id: String,
    style: Style,
    icon: String,
    label: String,
}

impl PowerButtonContent {
    fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: "power_button".to_string(),
            style: Style::new(),
            icon: icon.into(),
            label: label.into(),
        }
    }
}

impl Widget for PowerButtonContent {
    fn id(&self) -> &str {
        &self.id
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(70.0, 40.0)
    }

    fn render(&mut self, ctx: &mut dyn RenderContext, rect: Rect) {
        let response = ctx.interact(rect, self.id());

        let bg_color = if response.hovered {
            Color32::from_rgba_unmultiplied(180, 60, 60, 255)
        } else {
            Color32::from_rgba_unmultiplied(120, 40, 40, 255)
        };

        ctx.paint_rect(rect, 8.0, bg_color);

        // Icon + label horizontal
        let text = format!("{} {}", self.icon, self.label);
        ctx.paint_text(rect.center(), Align2::CENTER_CENTER, &text, 14.0, Color32::WHITE);

        if response.clicked {
            println!("Power action: {}", self.label);
        }
    }
}
