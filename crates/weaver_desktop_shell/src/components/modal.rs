//! Modal component - displays a widget in a floating layer above the desktop.
//!
//! The modal appears centered on screen with configurable sizing.
//! The parent (Shell) is responsible for dimming/disabling the desktop
//! when a modal is active.
//!
//! # Example
//!
//! ```rust,ignore
//! // Create modal with app menu widget
//! let modal = Modal::new(app_menu_widget)
//!     .max_size_percent(0.8, 0.8);
//!
//! // In Shell::ui()
//! if let Some(ref mut modal) = self.modal {
//!     if modal.ui(ctx) == ModalResult::Dismissed {
//!         self.modal = None;
//!         self.desktop.set_disabled(false);
//!     }
//! }
//! ```

use egui::{Color32, Context, Rect, Vec2};

use super::WidgetStr;

/// Result of modal rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalResult {
    /// Modal is still active.
    Active,
    /// Modal was dismissed (backdrop clicked or close requested).
    Dismissed,
}

/// A modal container that displays a widget in a floating layer.
pub struct Modal {
    /// The widget content of the modal.
    content: WidgetStr,
    /// Maximum width as percentage of screen (0.0 - 1.0).
    max_width_percent: f32,
    /// Maximum height as percentage of screen (0.0 - 1.0).
    max_height_percent: f32,
    /// Whether clicking the backdrop dismisses the modal.
    dismiss_on_backdrop_click: bool,
    /// Request to dismiss (set by content or backdrop click).
    dismiss_requested: bool,
    /// Border radius for the modal container.
    border_radius: f32,
}

impl Modal {
    /// Create a new modal with the given content widget.
    pub fn new(content: WidgetStr) -> Self {
        Self {
            content,
            max_width_percent: 0.8,
            max_height_percent: 0.8,
            dismiss_on_backdrop_click: true,
            dismiss_requested: false,
            border_radius: 12.0,
        }
    }

    /// Set maximum size as percentage of screen (0.0 - 1.0).
    pub fn max_size_percent(mut self, width: f32, height: f32) -> Self {
        self.max_width_percent = width.clamp(0.1, 1.0);
        self.max_height_percent = height.clamp(0.1, 1.0);
        self
    }

    /// Set whether clicking the backdrop dismisses the modal.
    pub fn dismiss_on_backdrop(mut self, dismiss: bool) -> Self {
        self.dismiss_on_backdrop_click = dismiss;
        self
    }

    /// Set border radius.
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Request to dismiss the modal (can be called by content).
    pub fn request_dismiss(&mut self) {
        self.dismiss_requested = true;
    }

    /// Check if dismiss was requested.
    pub fn is_dismiss_requested(&self) -> bool {
        self.dismiss_requested
    }

    /// Get mutable access to the content widget.
    pub fn content_mut(&mut self) -> &mut WidgetStr {
        &mut self.content
    }

    /// Render the modal and return whether it should remain active.
    pub fn ui(&mut self, ctx: &Context) -> ModalResult {
        // Check if already dismissed
        if self.dismiss_requested {
            return ModalResult::Dismissed;
        }

        let screen = ctx.input(|i| i.viewport_rect());

        // Calculate max size
        let max_size = Vec2::new(
            screen.width() * self.max_width_percent,
            screen.height() * self.max_height_percent,
        );

        // Get content preferred size (use min_size as hint)
        let content_min = self.content.min_size();
        let content_size = Vec2::new(
            content_min.x.min(max_size.x).max(200.0), // At least 200px wide
            content_min.y.min(max_size.y).max(150.0), // At least 150px tall
        );

        // If content is larger than min, use max as fallback
        let modal_size = if content_min.x < 10.0 && content_min.y < 10.0 {
            // Content didn't specify size, use max
            max_size
        } else {
            content_size
        };

        // Calculate centered position
        let modal_rect = Rect::from_center_size(screen.center(), modal_size);

        // Render modal content as an Area
        egui::Area::new(egui::Id::new("modal_content"))
            .order(egui::Order::Foreground)
            .fixed_pos(modal_rect.min)
            .show(ctx, |ui| {
                // Background frame for modal
                let frame = egui::Frame::new()
                    .fill(Color32::from_gray(35))
                    .corner_radius(self.border_radius)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)));

                frame.show(ui, |ui| {
                    ui.set_min_size(modal_size);
                    ui.set_max_size(modal_size);
                    
                    // Clip content to modal bounds
                    let content_rect = ui.max_rect();
                    ui.set_clip_rect(content_rect);

                    // Render content widget
                    self.content.ui(ui);
                });
            });

        // Handle backdrop click (invisible sense rect covering screen except modal)
        // We need to detect clicks outside the modal
        let _backdrop_id = egui::Id::new("modal_backdrop_sense");
        let backdrop_response = ctx.input(|i| {
            if i.pointer.any_pressed() {
                if let Some(pos) = i.pointer.interact_pos() {
                    !modal_rect.contains(pos)
                } else {
                    false
                }
            } else {
                false
            }
        });

        if backdrop_response && self.dismiss_on_backdrop_click {
            return ModalResult::Dismissed;
        }

        ModalResult::Active
    }
}
