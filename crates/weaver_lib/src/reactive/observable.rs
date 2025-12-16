//! Observable values with change notification.

use super::SignalFn;

/// Observable value with change notification.
/// Uses zero-allocation function pointer for the change callback.
/// Only notifies when the value actually changes (requires PartialEq).
pub struct Observable<T: PartialEq> {
    value: T,
    pub change: SignalFn<T>,
}

impl<T: PartialEq> Observable<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            change: SignalFn::new(),
        }
    }

    /// Set a new value. Notifies listeners only if value changed.
    pub fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
            self.change.notify(&self.value);
        }
    }

    /// Get a reference to the current value.
    #[inline]
    pub fn get(&self) -> &T {
        &self.value
    }
}

/// Signal value that always notifies on set, regardless of equality.
/// Uses zero-allocation function pointer for the change callback.
pub struct Signal<T> {
    value: T,
    pub change: SignalFn<T>,
}

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            change: SignalFn::new(),
        }
    }

    /// Set a new value. Always notifies listeners.
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.change.notify(&self.value);
    }

    /// Get a reference to the current value.
    #[inline]
    pub fn get(&self) -> &T {
        &self.value
    }
}
