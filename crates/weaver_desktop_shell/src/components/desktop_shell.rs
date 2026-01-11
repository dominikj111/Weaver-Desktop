//! Application Shell - Widget-based desktop environment.
//!
//! The Shell manages the desktop structure as a Widget tree with layered rendering:
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

use egui::{Align2, Color32, Context, TextureHandle, Vec2};
use egui_toast::Toasts;

use super::modal::{Modal, ModalResult};
use super::widget::{Align, Justify, Label, Size, Spacing, Widget, WidgetContent};
use super::{ImageSource, ImageSurface, ScaleMode};

/// Clock widget content - displays current time.
pub struct ClockWidget {
    format: String,
}

impl ClockWidget {
    pub fn new() -> Self {
        Self {
            format: "%I:%M %p".to_string(),
        }
    }

    pub fn with_format(format: impl Into<String>) -> Self {
        Self {
            format: format.into(),
        }
    }
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetContent for ClockWidget {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let now = chrono::Local::now();
        let time_str = now.format(&self.format).to_string();
        ui.label(time_str);
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(80.0, 20.0)
    }
}

/// Date widget content - displays current date.
pub struct DateWidget {
    format: String,
}

impl DateWidget {
    pub fn new() -> Self {
        Self {
            format: "%A, %B %d".to_string(),
        }
    }
}

impl Default for DateWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetContent for DateWidget {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let now = chrono::Local::now();
        let date_str = now.format(&self.format).to_string();
        if ui.button(date_str).clicked() {
            // TODO: toggle calendar
        }
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(180.0, 20.0)
    }
}

/// Menu button widget - triggers modal.
pub struct MenuButton {
    icon: String,
    size: f32,
}

impl MenuButton {
    pub fn new() -> Self {
        Self {
            icon: "☰".to_string(),
            size: 40.0,
        }
    }

    pub fn with_icon(icon: impl Into<String>) -> Self {
        Self {
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

impl WidgetContent for MenuButton {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let size = egui::vec2(self.size, self.size);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let bg_color = if response.hovered() {
                Color32::from_gray(60)
            } else {
                Color32::from_gray(40)
            };

            ui.painter().rect_filled(rect, 8.0, bg_color);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                &self.icon,
                egui::FontId::proportional(20.0),
                visuals.text_color(),
            );
        }

        // Store click in response - caller checks via context
        if response.clicked() {
            ui.ctx().memory_mut(|mem| {
                mem.data.insert_temp(egui::Id::new("menu_clicked"), true);
            });
        }
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(self.size, self.size)
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
    pub fn new(image_path: impl Into<PathBuf>) -> Self {
        Self {
            image_path: image_path.into(),
            texture: None,
            load_attempted: false,
            // Default size, will be updated when image loads
            size: Vec2::new(97.0, 30.0),
            target_height: 30.0,
        }
    }

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
    text: String,
}

impl StatusText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl WidgetContent for StatusText {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(&self.text);
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(self.text.len() as f32 * 8.0, 20.0)
    }
}

/// Version label widget.
pub struct VersionLabel {
    version: String,
}

impl VersionLabel {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Default for VersionLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetContent for VersionLabel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("v{}", self.version));
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(60.0, 20.0)
    }
}

/// Windows XP style taskbar widget - draws the iconic blue gradient bar.
pub struct XpTaskbar {
    height: f32,
}

impl XpTaskbar {
    pub fn new() -> Self {
        Self { height: 30.0 }
    }

    pub fn with_height(height: f32) -> Self {
        Self { height }
    }

    /// Draw the Windows XP taskbar gradient.
    fn paint_xp_gradient(painter: &egui::Painter, rect: egui::Rect) {
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
        painter.rect_filled(highlight_rect, 0.0, top_highlight);

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
            painter.rect_filled(strip_rect, 0.0, color);
        }

        // Draw bottom edge line
        let edge_rect = egui::Rect::from_min_size(
            egui::pos2(rect.min.x, rect.max.y - edge_height),
            egui::vec2(rect.width(), edge_height),
        );
        painter.rect_filled(edge_rect, 0.0, bottom_edge);
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

impl WidgetContent for XpTaskbar {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let desired_size = egui::vec2(available.x, self.height);
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            Self::paint_xp_gradient(ui.painter(), rect);
        }
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(100.0, self.height)
    }
}

/// Windows XP-style clock widget with the iconic tray gradient background.
pub struct XpClock {
    height: f32,
}

impl XpClock {
    pub fn new() -> Self {
        Self { height: 30.0 }
    }

    pub fn with_height(height: f32) -> Self {
        Self { height }
    }

    /// Paint the XP taskbar tray gradient background.
    /// The tray area has a slightly different gradient than the main taskbar.
    fn paint_tray_gradient(painter: &egui::Painter, rect: egui::Rect) {
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
        painter.rect_filled(highlight_rect, 0.0, top_highlight);

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
            painter.rect_filled(strip_rect, 0.0, color);
        }

        // Draw bottom edge line
        let edge_rect = egui::Rect::from_min_size(
            egui::pos2(rect.min.x, rect.max.y - edge_height),
            egui::vec2(rect.width(), edge_height),
        );
        painter.rect_filled(edge_rect, 0.0, bottom_edge);

        // Draw left separator line (distinguishes from main taskbar)
        let separator_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(1.0, height),
        );
        painter.rect_filled(separator_rect, 0.0, Color32::from_rgb(0x0C, 0x34, 0x75));
    }
}

impl Default for XpClock {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetContent for XpClock {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let desired_width = 85.0; // Fixed width for clock area
        let desired_size = egui::vec2(desired_width, self.height);
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            // Paint the tray gradient background
            Self::paint_tray_gradient(ui.painter(), rect);

            // Draw the time text centered
            let now = chrono::Local::now();
            let time_str = now.format("%I:%M %p").to_string();
            
            // XP uses a specific font style, but we'll use default with proper styling
            let text_color = Color32::WHITE;
            let text_rect = rect.shrink(4.0); // Padding inside the tray
            
            ui.painter().text(
                text_rect.center(),
                egui::Align2::CENTER_CENTER,
                time_str,
                egui::FontId::proportional(13.0),
                text_color,
            );
        }
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(85.0, self.height)
    }
}

/// Content placeholder - for the central view area.
pub struct ViewPlaceholder {
    label: String,
}

impl ViewPlaceholder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl WidgetContent for ViewPlaceholder {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label(&self.label);
        });
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
    icons: Vec<DesktopIcon>,
    icon_size: f32,
    spacing: f32,
    columns: usize,
    /// Cached textures for icons
    textures: Vec<Option<TextureHandle>>,
    /// Track which textures we've tried to load
    load_attempted: Vec<bool>,
}

impl IconGridWidget {
    pub fn new() -> Self {
        Self {
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

    fn load_icon_texture(&mut self, ui: &egui::Ui, index: usize) {
        if index >= self.icons.len() || self.load_attempted[index] {
            return;
        }
        self.load_attempted[index] = true;

        if let Some(ref path) = self.icons[index].icon_path {
            if path.exists() {
                if let Ok(image) = image::open(path) {
                    let rgba = image.to_rgba8();
                    let size = [rgba.width() as usize, rgba.height() as usize];
                    let pixels = rgba.into_raw();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                    let texture = ui.ctx().load_texture(
                        format!("desktop_icon_{}", index),
                        color_image,
                        egui::TextureOptions::LINEAR,
                    );
                    self.textures[index] = Some(texture);
                }
            }
        }
    }
}

impl Default for IconGridWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetContent for IconGridWidget {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let cell_size = self.icon_size + self.spacing;

        // Calculate actual columns based on available width
        let actual_cols = ((available.x / cell_size) as usize)
            .max(1)
            .min(self.columns);

        // Pre-load all textures first to avoid borrow issues
        for i in 0..self.icons.len() {
            if i < self.load_attempted.len() && !self.load_attempted[i] {
                self.load_attempted[i] = true;
                if let Some(ref path) = self.icons[i].icon_path {
                    if path.exists() {
                        if let Ok(image) = image::open(path) {
                            let rgba = image.to_rgba8();
                            let size = [rgba.width() as usize, rgba.height() as usize];
                            let pixels = rgba.into_raw();
                            let color_image =
                                egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                            let texture = ui.ctx().load_texture(
                                format!("desktop_icon_{}", i),
                                color_image,
                                egui::TextureOptions::LINEAR,
                            );
                            self.textures[i] = Some(texture);
                        }
                    }
                }
            }
        }

        ui.vertical(|ui| {
            let mut col = 0;

            for (i, icon) in self.icons.iter().enumerate() {
                // Draw icon
                let response = ui.allocate_ui(Vec2::splat(cell_size), |ui| {
                    ui.vertical_centered(|ui| {
                        // Icon image or fallback
                        let icon_rect = ui.allocate_space(Vec2::splat(self.icon_size)).1;

                        if let Some(ref texture) = self.textures[i] {
                            ui.painter().image(
                                texture.id(),
                                icon_rect,
                                egui::Rect::from_min_max(
                                    egui::pos2(0.0, 0.0),
                                    egui::pos2(1.0, 1.0),
                                ),
                                Color32::WHITE,
                            );
                        } else {
                            // Fallback: folder icon
                            ui.painter()
                                .rect_filled(icon_rect, 8.0, Color32::from_gray(60));
                            ui.painter().text(
                                icon_rect.center(),
                                Align2::CENTER_CENTER,
                                "📁",
                                egui::FontId::proportional(24.0),
                                Color32::WHITE,
                            );
                        }

                        // Label below icon
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(&icon.label)
                                    .size(11.0)
                                    .color(Color32::WHITE),
                            )
                            .wrap_mode(egui::TextWrapMode::Truncate),
                        );
                    });
                });

                // Handle click
                if response.response.interact(egui::Sense::click()).clicked() {
                    println!("Desktop icon clicked: {}", icon.action_id);
                }

                col += 1;
                if col >= actual_cols {
                    col = 0;
                    ui.end_row();
                }
            }
        });
    }

    fn min_size(&self) -> Vec2 {
        let rows = (self.icons.len() + self.columns - 1) / self.columns.max(1);
        let cell_size = self.icon_size + self.spacing;
        Vec2::new(self.columns as f32 * cell_size, rows as f32 * cell_size)
    }
}

/// Desktop image widget - displays an image (like a photo frame).
pub struct DesktopImageWidget {
    source: ImageSource,
    surface: ImageSurface,
    border_radius: f32,
    title: Option<String>,
}

impl DesktopImageWidget {
    pub fn new() -> Self {
        Self {
            source: ImageSource::None,
            surface: ImageSurface::with_id("desktop_image"),
            border_radius: 12.0,
            title: None,
        }
    }

    pub fn with_image(mut self, path: impl Into<PathBuf>) -> Self {
        self.source = ImageSource::Image(path.into());
        self.surface.set_source(self.source.clone());
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.source = ImageSource::Color(color);
        self.surface.set_source(self.source.clone());
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

    pub fn scale_mode(mut self, mode: ScaleMode) -> Self {
        self.surface.set_scale_mode(mode);
        self
    }
}

impl Default for DesktopImageWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetContent for DesktopImageWidget {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let (rect, _response) = ui.allocate_exact_size(available, egui::Sense::hover());

        // Draw background/border
        ui.painter()
            .rect_filled(rect, self.border_radius, Color32::from_gray(40));

        // Draw image (inset for border effect)
        let image_rect = rect.shrink(4.0);
        self.surface.paint_rect(ui, image_rect);

        // Draw title if present
        if let Some(ref title) = self.title {
            let title_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.max.y - 28.0),
                Vec2::new(rect.width(), 28.0),
            );
            let bottom_radius = self.border_radius as u8;
            ui.painter().rect_filled(
                title_rect,
                egui::CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: bottom_radius,
                    se: bottom_radius,
                },
                Color32::from_black_alpha(180),
            );
            ui.painter().text(
                title_rect.center(),
                Align2::CENTER_CENTER,
                title,
                egui::FontId::proportional(12.0),
                Color32::WHITE,
            );
        }
    }

    fn min_size(&self) -> Vec2 {
        Vec2::new(120.0, 90.0)
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
    desktop: Widget,

    /// Layer 2: Active modal (if any)
    modal: Option<Modal>,

    /// Layer 3: Toast notifications
    toasts: Toasts,

    /// Whether desktop is disabled (dimmed for modal)
    desktop_disabled: bool,
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
        }
    }

    /// Create a new desktop shell with content widgets.
    pub fn with_content(content_widgets: Vec<Widget>) -> Self {
        let desktop = Self::build_desktop_widget(content_widgets);

        Self {
            background: ImageSurface::with_id("desktop_background"),
            desktop,
            modal: None,
            toasts: Toasts::new()
                .anchor(Align2::RIGHT_TOP, (-10.0, 60.0))
                .direction(egui::Direction::TopDown),
            desktop_disabled: false,
        }
    }

    /// Set the content area widgets.
    pub fn set_content(&mut self, content_widgets: Vec<Widget>) {
        self.desktop = Self::build_desktop_widget(content_widgets);
    }

    /// Build the desktop widget tree.
    fn build_desktop_widget(content_widgets: Vec<Widget>) -> Widget {
        // Build content area with provided widgets
        let mut content_area = Widget::row("content-area")
            .height(Size::Flex(1.0))
            .padding(Spacing::all(16.0))
            .gap(16.0)
            .align(Align::Start);

        for widget in content_widgets {
            content_area = content_area.child(widget);
        }

        Widget::column("desktop")
            // Top bar
            // .child(
            //     Widget::row("top-bar")
            //         .height(Size::Fixed(44.0))
            //         .padding(Spacing::xy(12.0, 6.0))
            //         .align(Align::Center)
            //         .justify(Justify::SpaceBetween)
            //         .gap(8.0)
            //         .background(ImageSurface::with_source(ImageSource::Color(
            //             Color32::from_rgba_unmultiplied(30, 30, 30, 220),
            //         )))
            //         .border_radius(12.0)
            //         .margin(Spacing::new(8.0, 50.0, 0.0, 8.0)) // top, right (for menu btn), bottom, left
            //         // Left: spacer or future content
            //         .child(
            //             Widget::leaf("left-spacer", Label::new(""))
            //                 .width(Size::Fixed(40.0)),
            //         )
            //         // Center: Date/Time
            //         .child(
            //             Widget::leaf("date-time", DateWidget::new())
            //                 .width(Size::Content),
            //         )
            //         // Right: spacer (menu button is floating)
            //         .child(
            //             Widget::leaf("right-spacer", Label::new(""))
            //                 .width(Size::Fixed(40.0)),
            //         ),
            // )
            // Main content area with widgets
            .child(content_area)
            // Bottom bar - Windows XP style with taskbar and clock
            .child(
                Widget::row("xp-taskbar-row")
                    .height(Size::Fixed(30.0))
                    .gap(0.0)
                    .child(
                        Widget::leaf("xp-taskbar", XpTaskbar::with_height(30.0))
                            .width(Size::Flex(1.0))
                    )
                    .child(
                        Widget::leaf("xp-clock", XpClock::with_height(30.0))
                            .width(Size::Fixed(85.0))
                    )
            )
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
    pub fn show_app_menu(&mut self, content: Widget) {
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
    pub fn desktop_mut(&mut self) -> &mut Widget {
        &mut self.desktop
    }

    /// Render the shell.
    pub fn ui(&mut self, ctx: &Context, _view: impl FnOnce(&mut egui::Ui)) {
        let screen_rect = ctx.input(|i| i.viewport_rect());

        // Layer 0: Background
        self.background.paint_background(ctx, screen_rect);

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
                let app_menu = Self::build_app_menu();
                self.show_app_menu(app_menu);
            }
        }

        // Layer 1: Desktop widget tree
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // Render desktop
                self.desktop.ui(ui);

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
            .show(ctx, |ui| {
                let mut btn = XpStartButton::with_height("assets/xp_start.png", taskbar_height);
                btn.ui(ui);
            });

        // Layer 2: Modal
        if let Some(ref mut modal) = self.modal {
            match modal.ui(ctx) {
                ModalResult::Active => {}
                ModalResult::Dismissed => {
                    self.close_modal();
                }
            }
        }

        // Layer 3: Toasts
        self.toasts.show(ctx);
    }

    /// Build the app menu widget.
    fn build_app_menu() -> Widget {
        Widget::column("app-menu")
            .padding(Spacing::all(24.0))
            .gap(16.0)
            .align(Align::Stretch)
            // Title
            .child(
                Widget::row("menu-header")
                    .height(Size::Fixed(40.0))
                    .justify(Justify::Center)
                    .child(Widget::leaf("title", Label::new("App Menu")).width(Size::Content)),
            )
            // Menu grid (2x3 for now)
            .child(
                Widget::row("menu-row-1")
                    .height(Size::Fixed(80.0))
                    .gap(16.0)
                    .justify(Justify::Center)
                    .child(Self::menu_item("🏠", "Dashboard"))
                    .child(Self::menu_item("🔧", "Hardware"))
                    .child(Self::menu_item("📋", "Profiles")),
            )
            .child(
                Widget::row("menu-row-2")
                    .height(Size::Fixed(80.0))
                    .gap(16.0)
                    .justify(Justify::Center)
                    .child(Self::menu_item("📦", "System"))
                    .child(Self::menu_item("📁", "Files"))
                    .child(Self::menu_item("⚙", "Settings")),
            )
            // Power row
            .child(
                Widget::row("power-row")
                    .height(Size::Fixed(60.0))
                    .gap(16.0)
                    .justify(Justify::Center)
                    .margin(Spacing::new(24.0, 0.0, 0.0, 0.0))
                    .child(Self::power_button("🔄", "Restart"))
                    .child(Self::power_button("⏻", "Shutdown")),
            )
    }

    fn menu_item(icon: &str, label: &str) -> Widget {
        Widget::leaf(
            format!("menu-{}", label.to_lowercase()),
            MenuItemContent::new(icon, label),
        )
        .width(Size::Fixed(100.0))
        .height(Size::Fixed(80.0))
    }

    fn power_button(icon: &str, label: &str) -> Widget {
        Widget::leaf(
            format!("power-{}", label.to_lowercase()),
            PowerButtonContent::new(icon, label),
        )
        .width(Size::Fixed(80.0))
        .height(Size::Fixed(50.0))
    }
}

/// Menu item content widget.
struct MenuItemContent {
    icon: String,
    label: String,
}

impl MenuItemContent {
    fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: label.into(),
        }
    }
}

impl WidgetContent for MenuItemContent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if response.hovered() {
                Color32::from_gray(60)
            } else {
                Color32::from_gray(45)
            };

            ui.painter().rect_filled(rect, 12.0, bg_color);

            // Icon
            let icon_pos = egui::pos2(rect.center().x, rect.center().y - 10.0);
            ui.painter().text(
                icon_pos,
                Align2::CENTER_CENTER,
                &self.icon,
                egui::FontId::proportional(28.0),
                Color32::WHITE,
            );

            // Label
            let label_pos = egui::pos2(rect.center().x, rect.max.y - 12.0);
            ui.painter().text(
                label_pos,
                Align2::CENTER_CENTER,
                &self.label,
                egui::FontId::proportional(12.0),
                Color32::from_gray(200),
            );
        }

        if response.clicked() {
            println!("Menu item clicked: {}", self.label);
        }
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(80.0, 70.0)
    }
}

/// Power button content widget.
struct PowerButtonContent {
    icon: String,
    label: String,
}

impl PowerButtonContent {
    fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: label.into(),
        }
    }
}

impl WidgetContent for PowerButtonContent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if response.hovered() {
                Color32::from_rgba_unmultiplied(180, 60, 60, 255)
            } else {
                Color32::from_rgba_unmultiplied(120, 40, 40, 255)
            };

            ui.painter().rect_filled(rect, 8.0, bg_color);

            // Icon + label horizontal
            let text = format!("{} {}", self.icon, self.label);
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
        }

        if response.clicked() {
            println!("Power action: {}", self.label);
        }
    }

    fn min_size(&self) -> egui::Vec2 {
        egui::vec2(70.0, 40.0)
    }
}
