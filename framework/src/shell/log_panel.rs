//! Log panel component - tabbed view for app, weaver, and system logs.

/// Which log source is currently selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogTab {
    #[default]
    App,
    Weaver,
    System,
}

/// Log panel that displays above the bottom bar.
/// Shows tabbed logs at 50% height, 80% width, centered.
pub struct LogPanel {
    active_tab: LogTab,
    // Placeholder log entries - replace with real log sources later
    app_logs: Vec<String>,
    weaver_logs: Vec<String>,
    system_logs: Vec<String>,
}

impl Default for LogPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl LogPanel {
    pub fn new() -> Self {
        // Demo log entries
        Self {
            active_tab: LogTab::App,
            app_logs: vec![
                "[INFO] Application started".to_string(),
                "[INFO] Command bus initialized".to_string(),
                "[DEBUG] Rendering home view".to_string(),
            ],
            weaver_logs: vec![
                "[INFO] Weaver framework v0.1.0".to_string(),
                "[DEBUG] Shell initialized".to_string(),
            ],
            system_logs: vec![
                "[INFO] systemd: Started session".to_string(),
                "[INFO] kernel: USB device connected".to_string(),
            ],
        }
    }

    /// Render the log panel. Call this from Shell after central panel, before bottom bar.
    pub fn ui(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.content_rect();

        // Panel dimensions: 80% width, 50% height
        let panel_width = screen_rect.width() * 0.8;
        let panel_height = screen_rect.height() * 0.5;

        // Position: centered horizontally, attached to bottom (above bottom bar)
        let panel_x = (screen_rect.width() - panel_width) / 2.0;
        let panel_y = screen_rect.height() - panel_height - 30.0; // 30px for bottom bar

        egui::Window::new("log_panel")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .fixed_pos(egui::pos2(panel_x, panel_y))
            .fixed_size(egui::vec2(panel_width, panel_height))
            .frame(egui::Frame::window(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                self.render_content(ui);
            });
    }

    fn render_content(&mut self, ui: &mut egui::Ui) {
        // Tab bar
        ui.horizontal(|ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);

            if self.tab_button(ui, "App", LogTab::App) {
                self.active_tab = LogTab::App;
            }
            if self.tab_button(ui, "Weaver", LogTab::Weaver) {
                self.active_tab = LogTab::Weaver;
            }
            if self.tab_button(ui, "System", LogTab::System) {
                self.active_tab = LogTab::System;
            }
        });

        ui.separator();

        // Log content area with scroll
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 2.0);

                let logs = match self.active_tab {
                    LogTab::App => &self.app_logs,
                    LogTab::Weaver => &self.weaver_logs,
                    LogTab::System => &self.system_logs,
                };

                for log in logs {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(log).monospace().size(12.0),
                        )
                        .wrap(),
                    );
                }
            });
    }

    fn tab_button(&self, ui: &mut egui::Ui, label: &str, tab: LogTab) -> bool {
        let is_active = self.active_tab == tab;

        let button = egui::Button::new(
            egui::RichText::new(label).strong(),
        )
        .fill(if is_active {
            ui.style().visuals.selection.bg_fill
        } else {
            egui::Color32::TRANSPARENT
        })
        .corner_radius(egui::CornerRadius::ZERO)
        .min_size(egui::vec2(80.0, 28.0));

        ui.add(button).clicked()
    }

    // Public methods for adding logs (for future use)
    #[allow(dead_code)]
    pub fn log_app(&mut self, message: String) {
        self.app_logs.push(message);
    }

    #[allow(dead_code)]
    pub fn log_weaver(&mut self, message: String) {
        self.weaver_logs.push(message);
    }

    #[allow(dead_code)]
    pub fn log_system(&mut self, message: String) {
        self.system_logs.push(message);
    }
}
