mod bottom_panel;
mod calendar;
pub mod menu;
mod top_panel;

pub use bottom_panel::show_bottom_panel;
pub use calendar::show_calendar;
pub use top_panel::show_top_panel;

use egui::Ui;

pub fn show_view(ui: &mut Ui) {
    ui.heading("My egui Application");

    ui.separator();

    // ui.horizontal(|ui| {
    //     let name_label = ui.label("Your name: ");
    //     ui.text_edit_singleline(name).labelled_by(name_label.id);
    // });

    // ui.add(egui::Slider::new(age, 0..=120).text("age"));

    // ui.horizontal(|ui| {
    //     if ui.button("Increment").clicked() {
    //         *age += 1;
    //         // Simulate loading state
    //         *loading = true;
    //     }

    //     // Toggle loading for demo purposes
    //     if ui.button("Toggle Loading").clicked() {
    //         *loading = !*loading;
    //     }
    // });

    // // Display spinner when loading
    // if *loading {
    //     ui.horizontal(|ui| {
    //         ui.spinner();
    //         ui.label("Processing...");
    //     });
    // }
    // ui.label(format!("Hello '{}', age {}", name, age));

    ui.label("Hello World");
    ui.separator();
}
