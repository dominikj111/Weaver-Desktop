use std::cell::Cell;

use egui::{Ui, Widget};

use crate::framework::reactive::{listener::Listener, observable::Observable};

use crate::services::next_id;

pub trait InteractableHandlers<T>: Sized {
    fn get_interactable_mut(&mut self) -> &mut Interactable<T>;

    // Chainable mutation
    fn on_click<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&T) + 'static,
    {
        self.get_interactable_mut().click.subscribe(callback);
        self
    }

    fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&T) + 'static,
    {
        self.on_click(callback);
        self
    }

    // Chainable mutation
    fn on_press<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&T) + 'static,
    {
        self.get_interactable_mut().press.subscribe(callback);
        self
    }

    fn with_on_press<F>(mut self, callback: F) -> Self
    where
        F: Fn(&T) + 'static,
    {
        self.on_press(callback);
        self
    }

    // Chainable mutation
    fn on_release<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&T) + 'static,
    {
        self.get_interactable_mut().release.subscribe(callback);
        self
    }

    fn with_on_release<F>(mut self, callback: F) -> Self
    where
        F: Fn(&T) + 'static,
    {
        self.on_release(callback);
        self
    }
}

/// Reusable pointer interaction tracker
pub struct Interactable<T> {
    is_pressed: Cell<bool>, // Interior mutability
    pub click: Listener<T>,
    pub press: Listener<T>,
    pub release: Listener<T>,
}

impl<T> Interactable<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Now takes &self instead of &mut self (interior mutability via Cell)
    pub fn handle(&self, target: &T, ui: &egui::Ui, response: &egui::Response) {
        let pointer_down = ui.input(|i| i.pointer.primary_down());
        let pointer_released = ui.input(|i| i.pointer.primary_released());

        if response.is_pointer_button_down_on() && !self.is_pressed.get() {
            self.is_pressed.set(true);
            self.press.notify(target);
        }

        if self.is_pressed.get() && pointer_released {
            self.release.notify(target);
            if response.hovered() {
                self.click.notify(target);
            }
            self.is_pressed.set(false);
        }

        if !pointer_down {
            self.is_pressed.set(false);
        }
    }

    pub fn handle_widget(&self, target: &T, ui: &mut Ui, id: usize, widget: impl Widget) {
        ui.push_id(id, |ui| {
            let response = ui.add(widget);
            self.handle(target, ui, &response);
            response
        })
        .inner;
    }
}

impl<T> Default for Interactable<T> {
    fn default() -> Self {
        Self {
            is_pressed: Cell::new(false),
            click: Listener::new(),
            press: Listener::new(),
            release: Listener::new(),
        }
    }
}

pub struct ButtonOptions {
    pub click_handler: Box<dyn Fn(&Button)>,
    pub press_handler: Box<dyn Fn(&Button)>,
    pub release_handler: Box<dyn Fn(&Button)>,
    pub disabled: bool,
}

impl Default for ButtonOptions {
    fn default() -> Self {
        Self {
            click_handler: Box::new(|_| {}),
            press_handler: Box::new(|_| {}),
            release_handler: Box::new(|_| {}),
            disabled: false,
        }
    }
}

pub struct Button {
    internal_ui_id: usize,
    padding: egui::Vec2,
    interactable: Interactable<Self>,
    pub label: Observable<String>,
    pub disabled: Observable<bool>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            internal_ui_id: next_id(),
            padding: egui::vec2(10.0, 6.0),
            label: Observable::new(label.into()),
            disabled: Observable::new(false),
            interactable: Interactable::new(),
        }
    }

    pub fn with_options(label: impl Into<String>, options: ButtonOptions) -> Self {
        Self {
            internal_ui_id: next_id(),
            padding: egui::vec2(10.0, 6.0),
            label: Observable::new(label.into()),
            disabled: Observable::new(options.disabled),
            interactable: Interactable::new(),
        }
    }

    /// Render the button into a Ui
    pub fn ui(&self, ui: &mut egui::Ui) {
        let button = egui::Button::new(self.label.get()).min_size(egui::vec2(0.0, 0.0));

        let button = button.frame(true);
        ui.style_mut().spacing.button_padding = self.padding;

        ui.add_enabled_ui(!*self.disabled.get(), |ui| {
            self.interactable
                .handle_widget(self, ui, self.internal_ui_id, button);
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

impl InteractableHandlers<Self> for Button {
    fn get_interactable_mut(&mut self) -> &mut Interactable<Self> {
        &mut self.interactable
    }
}
