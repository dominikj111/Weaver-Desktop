//! Top bar component - status, date/time, menu trigger.

use std::fmt::Write;
use std::path::Path;

use super::icon_button::IconButton;

/// Thread-local buffer for datetime formatting to avoid per-frame allocations.
thread_local! {
    static DATETIME_BUF: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(64));
}

pub struct TopBar {
    /// Menu icon button with PNG support and fallback.
    menu_button: IconButton,
}

impl Default for TopBar {
    fn default() -> Self {
        Self {
            menu_button: IconButton::new("menu_icon", "☰")
                .with_size(40.0)
                .with_background_color(egui::Color32::WHITE)
                .with_stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                .with_padding(4.0),
        }
    }
}

impl TopBar {
    /// Set the menu button background color.
    pub fn set_menu_button_background(&mut self, color: egui::Color32) {
        self.menu_button.set_background_color(color);
    }

    /// Set the menu button size.
    pub fn set_menu_button_size(&mut self, size: f32) {
        self.menu_button.set_size(size);
    }

    /// Render the top bar.
    ///
    /// - `ui`: The egui UI context.
    /// - `menu_icon_path`: Optional path to a PNG image for the menu icon.
    ///   If None or loading fails, the fallback "☰" character is displayed.
    pub fn ui(&mut self, ui: &mut egui::Ui, menu_icon_path: Option<&Path>) {
        ui.horizontal(|ui| {
            // Left spacer
            ui.add_space(ui.available_width() / 2.0 - 100.0);

            // Center: Date/Time
            let now = chrono::Local::now();
            let date_time_str = DATETIME_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                buf.clear();
                write!(buf, "{}", now.format("%A, %B %d, %Y  %I:%M %p")).unwrap();
                buf.clone()
            });

            if ui.button(&date_time_str).clicked() {
                // TODO: toggle calendar popup
                // *calendar_open = !*calendar_open;
            }

            // Right: Menu icon (positioned to hang below the bar)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.add_space(10.0);
                // Push the button down so it extends below the top bar
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.add_space(12.0); // Offset down from top to hang below bar
                    if self.menu_button.ui(ui, menu_icon_path).clicked() {
                        // TODO: toggle menu
                        // *menu_open = !*menu_open;
                    }
                });
            });
        });
    }
}
