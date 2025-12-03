pub fn show_menu(ui: &mut egui::Ui) {
    ui.set_width(500.0);
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(20.0, 10.0);
        ui.add_space(10.0);

        // Menu icons in a single row
        if ui.button("🏠\nHome").clicked() {
            println!("Home clicked");
            // *menu_open = false;
        }
        if ui.button("📊\nDashboard").clicked() {
            println!("Dashboard clicked");
            // *menu_open = false;
        }
        if ui.button("👤\nProfile").clicked() {
            println!("Profile clicked");
            // *menu_open = false;
        }
        if ui.button("⚙\nSettings").clicked() {
            println!("Settings clicked");
            // *menu_open = false;
        }
        if ui.button("📁\nFiles").clicked() {
            println!("Files clicked");
            // *menu_open = false;
        }
        if ui.button("📈\nAnalytics").clicked() {
            println!("Analytics clicked");
            // *menu_open = false;
        }
        ui.add_space(10.0);
    });
    ui.add_space(10.0);
}
