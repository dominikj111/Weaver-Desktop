mod views;

use views::show_home;
use weaver::shell::Shell;

struct App {
    shell: Shell,
}

impl Default for App {
    fn default() -> Self {
        Self {
            shell: Shell::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.shell.ui(ctx, show_home);
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "SystemWeaver",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
