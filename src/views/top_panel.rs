pub fn show_top_panel(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        // Left spacer
        ui.add_space(ui.available_width() / 2.0 - 100.0);

        // Center: Date/Time
        let now = chrono::Local::now();
        let date_time_str = now.format("%A, %B %d, %Y  %I:%M %p").to_string();
        if ui.button(&date_time_str).clicked() {
            // *calendar_open = !*calendar_open;
        }

        // Right: Menu icon
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            if ui
                .add(egui::Button::new(egui::RichText::new("☰").size(24.0)))
                .clicked()
            {
                // *menu_open = !*menu_open;
            }
        });
    });
}
