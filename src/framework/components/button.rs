use std::cell::Cell;

use egui::{Ui, Widget};

use crate::framework::reactive::{observable::Observable, signal_fn::SignalFn};

use crate::services::next_id;

pub trait InteractableHandlers<T>: Sized {
    fn get_interactable_mut(&mut self) -> &mut Interactable<T>;

    /// Set click handler (static function pointer, zero allocation)
    fn on_click(&mut self, callback: fn(&T)) -> &mut Self {
        self.get_interactable_mut().click.set(callback);
        self
    }

    /// Owned chaining variant for vec construction
    fn with_on_click(mut self, callback: fn(&T)) -> Self {
        self.on_click(callback);
        self
    }

    /// Set press handler (static function pointer, zero allocation)
    fn on_press(&mut self, callback: fn(&T)) -> &mut Self {
        self.get_interactable_mut().press.set(callback);
        self
    }

    /// Owned chaining variant for vec construction
    fn with_on_press(mut self, callback: fn(&T)) -> Self {
        self.on_press(callback);
        self
    }

    /// Set release handler (static function pointer, zero allocation)
    fn on_release(&mut self, callback: fn(&T)) -> &mut Self {
        self.get_interactable_mut().release.set(callback);
        self
    }

    /// Owned chaining variant for vec construction
    fn with_on_release(mut self, callback: fn(&T)) -> Self {
        self.on_release(callback);
        self
    }
}

/// Reusable pointer interaction tracker
pub struct Interactable<T> {
    is_pressed: Cell<bool>, // Interior mutability
    pub click: SignalFn<T>,
    pub press: SignalFn<T>,
    pub release: SignalFn<T>,
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
            click: SignalFn::new(),
            press: SignalFn::new(),
            release: SignalFn::new(),
        }
    }
}

pub struct ButtonOptions {
    pub disabled: bool,
}

impl Default for ButtonOptions {
    fn default() -> Self {
        Self { disabled: false }
    }
}

pub struct Button {
    internal_ui_id: usize,
    padding: egui::Vec2,
    interactable: Interactable<Self>,
    pub label: Observable<&'static str>,
    pub disabled: Observable<bool>,
}

impl Button {
    pub fn new(label: &'static str) -> Self {
        Self {
            internal_ui_id: next_id(),
            padding: egui::vec2(10.0, 6.0),
            label: Observable::new(label),
            disabled: Observable::new(false),
            interactable: Interactable::new(),
        }
    }

    pub fn with_options(label: &'static str, options: ButtonOptions) -> Self {
        Self {
            internal_ui_id: next_id(),
            padding: egui::vec2(10.0, 6.0),
            label: Observable::new(label),
            disabled: Observable::new(options.disabled),
            interactable: Interactable::new(),
        }
    }

    /// Render the button into a Ui
    pub fn ui(&self, ui: &mut egui::Ui) {
        let button = egui::Button::new(*self.label.get()).min_size(egui::vec2(0.0, 0.0));

        let button = button.frame(true);
        ui.style_mut().spacing.button_padding = self.padding;

        ui.add_enabled_ui(!*self.disabled.get(), |ui| {
            self.interactable
                .handle_widget(self, ui, self.internal_ui_id, button);
        });
    }
}

impl InteractableHandlers<Self> for Button {
    fn get_interactable_mut(&mut self) -> &mut Interactable<Self> {
        &mut self.interactable
    }
}
