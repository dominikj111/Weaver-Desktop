use egui::{Color32, Context, Id};

pub fn show_modal(ctx: &Context, mut should_close: impl FnMut()) {
    let modal = egui::Modal::new(Id::new("modal"))
        .backdrop_color(Color32::from_black_alpha(0))
        .show(ctx, |ui| {
            ui.set_width(250.0);

            ui.heading("Edit User");

            // ui.label("Name:");
            // ui.text_edit_singleline(name);

            // ComboBox::new("role", "Role")
            //     .selected_text(*role)
            //     .show_ui(ui, |ui| {
            //         for r in Self::ROLES {
            //             ui.selectable_value(role, r, r);
            //         }
            //     });

            // ui.separator();

            // Confirm buttons ("Confirm", "Cancel", "Close", ...) ?
            // egui::Sides::new().show(
            //     ui,
            //     |_ui| {},
            //     |ui| {
            //         if ui.button("Save").clicked() {
            //             toasts.add(Toast {
            //                 text: "Hello, World".into(),
            //                 kind: ToastKind::Info,
            //                 options: ToastOptions::default()
            //                     .duration_in_seconds(10.0)
            //                     .show_progress(true)
            //                     .show_icon(true),
            //                 ..Default::default()
            //             });
            //             // *save_modal_open = true;
            //             println!("Save");
            //         }
            //         if ui.button("Cancel").clicked() {
            //             // You can call `ui.close()` to close the modal.
            //             // (This causes the current modals `should_close` to return true)
            //             ui.close();
            //         }
            //     },
            // );
        });

    if modal.should_close() {
        should_close();
    }
}
