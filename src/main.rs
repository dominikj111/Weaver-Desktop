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
        "SystemWeaver",
        options,
        Box::new(|_cc| {
            // DPI - scaling (when working on custom DRM backend)
            // let current_scale = cc.egui_ctx.pixels_per_point();
            // println!("Current scale: {}", current_scale);
            // cc.egui_ctx.set_pixels_per_point(1.0);
            Ok(Box::new(App::new()))
        }),
    )
}
