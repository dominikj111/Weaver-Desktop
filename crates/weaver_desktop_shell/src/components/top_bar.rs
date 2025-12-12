//! Top bar component - status, date/time, menu trigger.

use std::fmt::Write;
use std::path::Path;

/// Thread-local buffer for datetime formatting to avoid per-frame allocations.
thread_local! {
    static DATETIME_BUF: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(64));
}

pub struct TopBar {}

impl Default for TopBar {
    fn default() -> Self {
        Self {}
    }
}

impl TopBar {
    /// Render the top bar.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
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
        });
    }
}
