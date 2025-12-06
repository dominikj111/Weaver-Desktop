use std::collections::HashMap;

/// Generic listener registry for any event payload type.
/// Use this to add pub/sub capability to any struct.
pub struct Listener<T> {
    callbacks: HashMap<usize, Box<dyn Fn(&T)>>,
    next_id: usize,
}

impl<T> Default for Listener<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Listener<T> {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            next_id: 0,
        }
    }

    /// Register a callback. Returns a stable ID for removal.
    pub fn subscribe<F>(&mut self, callback: F) -> usize
    where
        F: Fn(&T) + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.callbacks.insert(id, Box::new(callback));
        id
    }

    /// Remove a listener by its ID.
    pub fn unsubscribe(&mut self, id: usize) {
        self.callbacks.remove(&id);
    }

    /// Notify all listeners with the given value.
    pub fn notify(&self, value: &T) {
        for callback in self.callbacks.values() {
            callback(value);
        }
    }

    /// Clear all listeners.
    pub fn clear(&mut self) {
        self.callbacks.clear();
    }

    /// Returns the number of active listeners.
    pub fn len(&self) -> usize {
        self.callbacks.len()
    }

    /// Returns true if there are no listeners.
    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }
}
