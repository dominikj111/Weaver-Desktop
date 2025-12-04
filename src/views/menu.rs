use crate::framework::{
    component::Component, components::button::Button, reactive::observable::Observable,
};

pub struct Menu {
    visible: Observable<bool>,
    buttons: Vec<Button>,
}

impl Menu {
    pub fn new(_args: &[&str]) -> Self {
        let visible = Observable::new(true);

        let on_click_handler = |b: &Button| println!("{} clicked", b.get_label());

        let buttons = vec![
            Button::new("🏠\nHome").on_click(on_click_handler),
            Button::new("📊\nDashboard").on_click(on_click_handler),
            Button::new("👤\nProfile").on_click(on_click_handler),
            Button::new("⚙\nSettings").on_click(on_click_handler),
            Button::new("📁\nFiles").on_click(on_click_handler),
            Button::new("📈\nAnalytics").on_click(on_click_handler),
        ];

        Self { visible, buttons }
    }

    pub fn show(&mut self) {
        self.visible.set(true);
    }

    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    pub fn is_visible(&self) -> bool {
        *self.visible.get()
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
            .show(ctx, |ui| {
                ui.set_width(500.0);
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(20.0, 10.0);
                    ui.add_space(10.0);

                    for button in &mut self.buttons {
                        button.ui(ui);
                    }

                    ui.add_space(10.0);
                });
                ui.add_space(10.0);
            });
    }
}
