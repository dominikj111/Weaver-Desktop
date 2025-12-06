use super::listener::Listener;

pub struct Signal<T> {
    value: T,
    pub change: Listener<T>,
}

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            change: Listener::new(),
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        self.change.notify(&self.value);
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}
