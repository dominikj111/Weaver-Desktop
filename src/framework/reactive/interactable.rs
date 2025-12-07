use std::cell::Cell;

use egui::{Ui, Widget};

use crate::framework::reactive::signal_fn::SignalFn;

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
