//! Pointer interaction tracking for widgets.

use std::cell::Cell;

use egui::{Ui, Widget};

use crate::reactive::SignalFn;

/// Reusable pointer interaction tracker with press/release/click signals.
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

    /// Handle pointer interactions on a response.
    /// Uses interior mutability via Cell, so takes &self.
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

    /// Convenience method to add a widget and handle its interactions.
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
