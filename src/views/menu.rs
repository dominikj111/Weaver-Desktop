use crate::framework::{component::Component, reactive::visibility::Visibility};

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

pub struct Menu {
    visible: Visibility,
}

impl Menu {
    pub fn new(_args: &[&str]) -> Self {
        let visible = Visibility::new(true);

        Self { visible }
    }

    pub fn show(&mut self) {
        self.visible.show();
    }

    pub fn hide(&mut self) {
        self.visible.hide();
    }

    pub fn is_visible(&self) -> bool {
        self.visible.is_visible()
    }
}

impl Component for Menu {
    fn ui(&mut self, ctx: &egui::Context) {
        if !self.is_visible() {
            return;
        }

        let screen_rect = ctx.content_rect();
        egui::Window::new("control_menu")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(screen_rect.right() - 520.0, 50.0))
            .show(ctx, show_menu);
    }
}
