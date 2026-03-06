//! Component trait for top-level UI elements that render with full Context access.

use egui::Context;

/// Trait for components that manage their own window/area and need full Context access.
///
/// Use this for top-level UI elements like menus, dialogs, and panels.
/// For atomic widgets that render into a `&mut Ui`, implement `egui::WidgetStr` instead.
pub trait Component {
    /// Called once when the component is first initialized.
    fn on_init(&mut self) {}

    /// Called when the component becomes visible.
    fn on_visible(&mut self) {}

    /// Called when the component is hidden.
    fn on_hidden(&mut self) {}

    /// Render the component. Called every frame.
    fn ui(&mut self, ctx: &Context);
}
