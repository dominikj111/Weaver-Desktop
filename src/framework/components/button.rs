use crate::framework::reactive::{listeners::Listeners, observable::Observable};

pub struct Button {
    label: String,
    disabled: Observable<bool>,
    padding: egui::Vec2,
    on_click: Listeners<Self>,
    on_press: Listeners<Self>,
    on_release: Listeners<Self>,
    /// Track pressed state for edge detection
    is_pressed: bool,
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
            is_pressed: false,
        }
    }

    /// Set button padding (builder pattern)
    pub fn padding(mut self, padding: impl Into<egui::Vec2>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Disable the button (grayed out, no interaction)
    pub fn disable(mut self) -> Self
    where
        Self: 'static,
    {
        self.disabled.set(true);
        self
    }

    /// Enable the button
    pub fn enable(mut self) -> Self
    where
        Self: 'static,
    {
        self.disabled.set(false);
        self
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
    pub fn remove_click_listener(mut self, id: usize) -> Self
    where
        Self: 'static,
    {
        self.on_click.unsubscribe(id);
        self
    }

    /// Remove a press listener by ID
    pub fn remove_press_listener(mut self, id: usize) -> Self
    where
        Self: 'static,
    {
        self.on_press.unsubscribe(id);
        self
    }

    /// Remove a release listener by ID
    pub fn remove_release_listener(mut self, id: usize) -> Self
    where
        Self: 'static,
    {
        self.on_release.unsubscribe(id);
        self
    }

    /// Render the button into a Ui
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let button = egui::Button::new(&self.label).min_size(egui::vec2(0.0, 0.0));

        // Apply padding via frame
        let button = button.frame(true);

        ui.add_enabled_ui(!*self.disabled.get(), |ui| {
            ui.style_mut().spacing.button_padding = self.padding;

            // Push unique ID based on label to avoid widget conflicts
            let id = self.label.clone();
            ui.push_id(id, |ui| {
                let response = ui.add(button);

                // Check global pointer state
                let pointer_down = ui.input(|i| i.pointer.primary_down());
                let pointer_released = ui.input(|i| i.pointer.primary_released());

                // Press started on this button
                let press_started_here = response.is_pointer_button_down_on() && !self.is_pressed;

                if press_started_here {
                    self.is_pressed = true;
                    self.on_press.notify(self);
                }

                // Release: fires whenever we were tracking a press and pointer released anywhere
                if self.is_pressed && pointer_released {
                    self.on_release.notify(self);

                    // Click: only if still hovering over button
                    if response.hovered() {
                        self.on_click.notify(self);
                    }

                    self.is_pressed = false;
                }

                // Safety: if pointer is not down at all, reset state
                if !pointer_down {
                    self.is_pressed = false;
                }
            });
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
