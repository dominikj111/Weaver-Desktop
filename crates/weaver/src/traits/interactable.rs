//! Trait for types that support pointer interaction handlers.

use crate::components::Interactable;

/// Trait for types that support pointer interaction handlers.
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
