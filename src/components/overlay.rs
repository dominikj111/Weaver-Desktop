use std::sync::Arc;

use egui::{Context, Rect, Style};

pub fn show_overlay(
    ctx: &Context,
    rect: Rect,
    style: Arc<Style>,
    mut back_overlay_click: impl FnMut(),
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
                back_overlay_click();
            }
        });
}

pub fn show_fullscreen_overlay(ctx: &Context, back_overlay_click: impl FnMut()) {
    show_overlay(ctx, ctx.screen_rect(), ctx.style(), back_overlay_click);
}
