//! Overlay widgets for modal backgrounds and click-away areas.

use egui::{Context, Rect, Style};

/// Show a semi-transparent overlay over a specific rect.
/// Calls the callback when the overlay is clicked.
pub fn show_overlay(
    ctx: &Context,
    rect: Rect,
    style: &Style,
    mut on_click: impl FnMut(),
) {
    egui::Area::new(egui::Id::new("overlay"))
        .fixed_pos(rect.min)
        .order(egui::Order::Middle)
        .interactable(true)
        .show(ctx, |ui| {
            let painter = ui.painter();
            // Use the current theme's background color with high transparency
            let bg_color = style.visuals.window_shadow.color;
            let overlay_color = egui::Color32::from_rgba_unmultiplied(
                bg_color.r(),
                bg_color.g(),
                bg_color.b(),
                100, // High alpha for semi-transparent effect
            );
            painter.rect_filled(rect, 0.0, overlay_color);

            // Invisible button to capture all clicks on central panel
            let response = ui.allocate_rect(rect, egui::Sense::click());

            if response.clicked() {
                on_click();
            }
        });
}

/// Show a fullscreen overlay that covers the entire content area.
pub fn show_fullscreen_overlay(ctx: &Context, on_click: impl FnMut()) {
    show_overlay(ctx, ctx.content_rect(), &ctx.style(), on_click);
}
