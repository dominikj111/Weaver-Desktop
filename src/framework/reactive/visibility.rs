pub type Visibility = super::observable::Observable<bool>;

impl Visibility {
    pub fn show(&mut self) {
        self.set(true);
    }

    pub fn hide(&mut self) {
        self.set(false);
    }

    pub fn toggle(&mut self) {
        self.set(!*self.get());
    }

    pub fn is_visible(&self) -> bool {
        *self.get()
    }
}
