use crate::framework::reactive::interactable::{Interactable, InteractableHandlers};
use crate::framework::reactive::observable::Observable;

use crate::services::next_id;

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
