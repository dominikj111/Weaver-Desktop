use super::listener::Listener;

pub struct Observable<T: PartialEq> {
    value: T,
    pub change: Listener<T>,
}

impl<T: PartialEq> Observable<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            change: Listener::new(),
        }
    }

    pub fn set(&mut self, value: T) {
        if self.value != value {
            self.value = value;
            self.change.notify(&self.value);
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}
