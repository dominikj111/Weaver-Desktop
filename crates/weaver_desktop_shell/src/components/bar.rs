//! Bar component - unified top/bottom bar with pluggable styles.
//!
//! The bar system separates concerns:
//! - `BarStyle` trait: defines visual appearance (frame, background, custom painting)
//! - `Bar`: handles position, interaction, and content rendering
//!
//! Built-in styles:
//! - `TransparentOverlay`: semi-transparent dark background
//! - `SolidRounded`: solid color with rounded corners

use weaver::{Interactable, InteractableHandlers};

/// Position of the bar on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarPosition {
    Top,
    Bottom,
}

/// Trait for bar visual styles.
///
/// Implement this trait to create custom bar appearances.
/// The bar component will call these methods to render the background.
pub trait BarStyle {
    /// Returns the egui Frame to use for the bar panel.
    fn frame(&self, position: BarPosition) -> egui::Frame;

    /// Optional custom painting after the frame is rendered.
    /// Use this for effects like rounded corners, shadows, gradients, etc.
    #[allow(unused_variables)]
    fn paint(&self, painter: &egui::Painter, rect: egui::Rect, position: BarPosition) {}
}

/// Semi-transparent dark overlay style.
///
/// This is the default style that blends with the background image.
#[derive(Debug, Clone)]
pub struct TransparentOverlay {
    /// Alpha value for the dark overlay (0-255).
    pub alpha: u8,
}

impl Default for TransparentOverlay {
    fn default() -> Self {
        Self { alpha: 128 }
    }
}

impl BarStyle for TransparentOverlay {
    fn frame(&self, _position: BarPosition) -> egui::Frame {
        egui::Frame::NONE.fill(egui::Color32::from_black_alpha(self.alpha))
    }
}

/// Solid color bar with rounded corners.
///
/// Renders as a floating rounded rectangle, not attached to screen edges.
#[derive(Debug, Clone)]
pub struct SolidRounded {
    /// Background color.
    pub color: egui::Color32,
    /// Corner rounding radius.
    pub rounding: f32,
    /// Horizontal margin from screen edges.
    pub margin_x: f32,
    /// Vertical margin from screen edge.
    pub margin_y: f32,
}

impl Default for SolidRounded {
    fn default() -> Self {
        Self {
            color: egui::Color32::BLACK,
            rounding: 12.0,
            margin_x: 20.0,
            margin_y: 10.0,
        }
    }
}

impl BarStyle for SolidRounded {
    fn frame(&self, _position: BarPosition) -> egui::Frame {
        egui::Frame::NONE
            .fill(self.color)
            .corner_radius(egui::CornerRadius::same(self.rounding as u8))
            .inner_margin(egui::Margin::symmetric(self.margin_x as i8, 8))
    }
}

/// A bar component with pluggable style and interaction support.
///
/// Generic over `S: BarStyle` to allow different visual appearances.
/// Includes `Interactable` for press/release/click signals.
pub struct Bar<S: BarStyle> {
    position: BarPosition,
    style: S,
    interactable: Interactable<Self>,
}

impl<S: BarStyle> Bar<S> {
    /// Create a new bar with the given position and style.
    pub fn new(position: BarPosition, style: S) -> Self {
        Self {
            position,
            style,
            interactable: Interactable::new(),
        }
    }

    /// Get the bar position.
    pub fn position(&self) -> BarPosition {
        self.position
    }

    /// Get a reference to the style.
    pub fn style(&self) -> &S {
        &self.style
    }

    /// Get a mutable reference to the style.
    pub fn style_mut(&mut self) -> &mut S {
        &mut self.style
    }

    /// Render the bar using egui's TopBottomPanel.
    ///
    /// The `content` closure receives a `&mut Ui` to render bar contents.
    pub fn ui(&mut self, ctx: &egui::Context, content: impl FnOnce(&mut egui::Ui)) {
        let frame = self.style.frame(self.position);
        let id = match self.position {
            BarPosition::Top => "bar_top",
            BarPosition::Bottom => "bar_bottom",
        };

        let panel = match self.position {
            BarPosition::Top => egui::TopBottomPanel::top(id),
            BarPosition::Bottom => egui::TopBottomPanel::bottom(id),
        };

        let response = panel.frame(frame).show(ctx, |ui| {
            // Custom painting if the style needs it
            let rect = ui.available_rect_before_wrap();
            self.style.paint(ui.painter(), rect, self.position);

            // Render content
            content(ui);

            // Return the full rect for interaction
            ui.min_rect()
        });

        // Handle interactions on the bar area
        let _bar_rect = response.inner;
        // Note: interaction handling would need UI context, skipping for panel-based rendering
        // For full interaction support, use ui_floating() instead
    }

    /// Render the bar as a floating Area (for more positioning control).
    ///
    /// Use this when you need the bar to float freely, overlap other elements,
    /// or have custom positioning not tied to panel edges.
    pub fn ui_floating(
        &mut self,
        ctx: &egui::Context,
        pos: egui::Pos2,
        width: f32,
        content: impl FnOnce(&mut egui::Ui),
    ) {
        let id = match self.position {
            BarPosition::Top => "bar_top_floating",
            BarPosition::Bottom => "bar_bottom_floating",
        };

        egui::Area::new(egui::Id::new(id))
            .fixed_pos(pos)
            .order(egui::Order::Middle)
            .interactable(true)
            .show(ctx, |ui| {
                let frame = self.style.frame(self.position);
                frame.show(ui, |ui| {
                    ui.set_width(width);

                    // Custom painting
                    let rect = ui.available_rect_before_wrap();
                    self.style.paint(ui.painter(), rect, self.position);

                    // Render content
                    content(ui);
                });

                // Handle interactions
                let response = ui.interact(
                    ui.min_rect(),
                    egui::Id::new(format!("{}_interact", id)),
                    egui::Sense::click(),
                );
                self.interactable.handle(self, ui, &response);
            });
    }
}

impl<S: BarStyle> InteractableHandlers<Self> for Bar<S> {
    fn get_interactable_mut(&mut self) -> &mut Interactable<Self> {
        &mut self.interactable
    }
}
