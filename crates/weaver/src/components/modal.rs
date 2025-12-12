//! Modal dialog widget.

use egui::{Color32, Context, Id};

/// Show a modal dialog. Calls the callback when the modal should close.
pub fn show_modal(ctx: &Context, mut on_close: impl FnMut()) {
    let modal = egui::Modal::new(Id::new("modal"))
        .backdrop_color(Color32::from_black_alpha(0))
        .show(ctx, |ui| {
            ui.set_width(250.0);
            ui.heading("Edit User");
        });

    if modal.should_close() {
        on_close();
    }
}
