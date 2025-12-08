//! Home view - the main application view.

use egui::Ui;

pub fn show_home(ui: &mut Ui) {
    ui.centered_and_justified(|ui| {
        ui.heading("Welcome to SystemWeaver");
    });
}
