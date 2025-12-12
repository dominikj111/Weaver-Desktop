//! Bottom bar component - status, notifications.

pub struct BottomBar;

impl Default for BottomBar {
    fn default() -> Self {
        Self
    }
}

impl BottomBar {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Ready");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("v0.1.0");
            });
        });
    }
}
