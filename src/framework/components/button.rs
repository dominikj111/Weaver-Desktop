use crate::framework::reactive::{listeners::Listeners, observable::Observable};

pub struct Button {
    label: String,
    disabled: Observable<bool>,
    padding: egui::Vec2,
    on_click: Listeners<Self>,
    on_press: Listeners<Self>,
    on_release: Listeners<Self>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            disabled: Observable::new(false),
            padding: egui::vec2(10.0, 6.0),
            on_click: Listeners::new(),
            on_press: Listeners::new(),
            on_release: Listeners::new(),
        }
    }

    /// Set button padding (builder pattern)
    pub fn padding(mut self, padding: impl Into<egui::Vec2>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Disable the button (grayed out, no interaction)
    pub fn disable(&mut self) {
        self.disabled.set(true);
    }

    /// Enable the button
    pub fn enable(&mut self) {
        self.disabled.set(false);
    }

    /// Check if button is disabled
    pub fn is_disabled(&self) -> bool {
        *self.disabled.get()
    }

    /// Set the label
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = label.into();
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    /// Subscribe to click events. Returns listener ID for removal.
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Self) + 'static,
    {
        self.on_click.subscribe(callback);
        self
    }

    /// Subscribe to press events. Returns listener ID for removal.
    pub fn on_press<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Self) + 'static,
    {
        self.on_press.subscribe(callback);
        self
    }

    /// Subscribe to release events. Returns listener ID for removal.
    pub fn on_release<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Self) + 'static,
    {
        self.on_release.subscribe(callback);
        self
    }

    /// Remove a click listener by ID
    pub fn remove_click_listener(&mut self, id: usize) {
        self.on_click.unsubscribe(id);
    }

    /// Remove a press listener by ID
    pub fn remove_press_listener(&mut self, id: usize) {
        self.on_press.unsubscribe(id);
    }

    /// Remove a release listener by ID
    pub fn remove_release_listener(&mut self, id: usize) {
        self.on_release.unsubscribe(id);
    }

    /// Render the button into a Ui
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let button = egui::Button::new(&self.label).min_size(egui::vec2(0.0, 0.0));

        // Apply padding via frame
        let button = button.frame(true);

        ui.add_enabled_ui(!*self.disabled.get(), |ui| {
            ui.style_mut().spacing.button_padding = self.padding;

            let response = ui.add(button);

            if response.clicked() {
                self.on_click.notify(self);
            }

            if response.is_pointer_button_down_on() {
                // Note: This fires every frame while pressed
                // For single press event, would need state tracking
            }

            // Detect press (transition from not pressed to pressed)
            if response.drag_started() {
                self.on_press.notify(self);
            }

            // Detect release
            if response.drag_stopped() || (response.hovered() && response.clicked()) {
                // clicked() already handles release, but for explicit release:
            }
        });
    }
}

impl From<&str> for Button {
    fn from(label: &str) -> Self {
        Button::new(label)
    }
}

impl From<String> for Button {
    fn from(label: String) -> Self {
        Button::new(label)
    }
}
