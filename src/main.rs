mod app;

use app::App;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]) // Window size
            .with_min_inner_size([800.0, 600.0]), // Minimum size
        ..Default::default()
    };

    eframe::run_native(
        "Weaver Desktop",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
