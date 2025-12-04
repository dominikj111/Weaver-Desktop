use super::listeners::Listeners;

pub struct Observable<T: PartialEq> {
    value: T,
    change_listeners: Listeners<T>,
}

impl<T: PartialEq> Observable<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            change_listeners: Listeners::new(),
        }
    }

    pub fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
            self.notify_change();
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    /// Register a callback to be called whenever the value changes.
    /// Returns a stable ID that can be used to remove the listener.
    pub fn on_change<F>(&mut self, callback: F) -> usize
    where
        F: Fn(&T) + 'static,
    {
        self.change_listeners.subscribe(callback)
    }

    /// Remove a listener by its ID.
    pub fn remove_listener(&mut self, id: usize) {
        self.change_listeners.unsubscribe(id);
    }

    /// Clear all listeners.
    pub fn clear_listeners(&mut self) {
        self.change_listeners.clear();
    }

    fn notify_change(&self) {
        self.change_listeners.notify(&self.value);
    }
}
