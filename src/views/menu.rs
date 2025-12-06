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
        let on_press_handler = |b: &Button| println!("{} pressed", b.get_label());
        let on_release_handler = |b: &Button| println!("{} released", b.get_label());

        let buttons = vec![
            Button::new("🏠\nHome")
                .on_click(on_click_handler)
                .on_press(on_press_handler)
                .on_release(on_release_handler)
                .disable(),
            Button::new("📊\nDashboard")
                .on_click(on_click_handler)
                .on_press(on_press_handler)
                .on_release(on_release_handler),
            Button::new("👤\nProfile")
                .on_click(on_click_handler)
                .on_press(on_press_handler)
                .on_release(on_release_handler),
            Button::new("⚙\nSettings")
                .on_click(on_click_handler)
                .on_press(on_press_handler)
                .on_release(on_release_handler),
            Button::new("📁\nFiles")
                .on_click(on_click_handler)
                .on_press(on_press_handler)
                .on_release(on_release_handler),
            Button::new("📈\nAnalytics")
                .on_click(on_click_handler)
                .on_press(on_press_handler)
                .on_release(on_release_handler),
        ];

        // NOTE:
        // Accept that callbacks are notification-only, parent is responsible to disable button
        // button.on_click(|b| b.disable())  // Won't work

        // 1. Keep ui() pure — Only rendering, no business logic
        // 2. Add event bus soon — You'll need it for cross-component communication
        // 3. Consider a Widget trait — For atomic components that render into &mut Ui (vs Component for top-level things with &Context)

        // ```
        // // 1. Button callback notifies
        // Button::new("Home").on_click(|b| {
        //     event_bus.emit(AppEvent::NavigateTo("home"));  // future
        //     // or just: println!("{} clicked", b.get_label());
        // });

        // // 2. Parent/reducer handles event, updates state
        // fn handle_event(&mut self, event: AppEvent) {
        //     match event {
        //         AppEvent::NavigateTo(page) => self.current_page = page,
        //     }
        // }

        // // 3. Next frame, ui() reads new state and renders accordingly
        // fn ui(&mut self, ctx: &Context) {
        //     // renders based on self.current_page
        // }
        // ```

        // callbacks pure (no mutation, just notification)
        // state changes centralized (easier to debug/test)
        // UI as a function of state (idiomatic immediate mode)

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
