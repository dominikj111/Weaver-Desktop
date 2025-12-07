/// Zero-allocation event signal using static function pointers.
/// 
/// Use this when callbacks don't need to capture state.
/// Perfect for command/dispatch patterns where the callback
/// simply notifies and the component handles state updates separately.
///
/// # Example
/// ```
/// fn handle_click(button: &Button) {
///     println!("Clicked: {}", button.label.get());
/// }
///
/// let mut signal = SignalFn::new();
/// signal.set(handle_click);
/// signal.notify(&button);
/// ```
pub struct SignalFn<T> {
    callback: Option<fn(&T)>, // Static function pointer, zero heap allocation
}

impl<T> Default for SignalFn<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SignalFn<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { callback: None }
    }

    /// Set the callback function pointer.
    /// Replaces any existing callback.
    #[inline]
    pub fn set(&mut self, callback: fn(&T)) {
        self.callback = Some(callback);
    }

    /// Clear the callback.
    #[inline]
    pub fn clear(&mut self) {
        self.callback = None;
    }

    /// Returns true if a callback is set.
    #[inline]
    pub fn is_set(&self) -> bool {
        self.callback.is_some()
    }

    /// Notify the callback with the given value.
    /// Does nothing if no callback is set.
    #[inline]
    pub fn notify(&self, value: &T) {
        if let Some(cb) = self.callback {
            cb(value);
        }
    }
}

/// Zero-allocation event signal supporting multiple static function pointers.
/// 
/// Stack-allocated array of function pointers with a fixed capacity.
/// No heap allocation, ideal for resource-constrained environments.
///
/// # Example
/// ```
/// fn log_click(button: &Button) { println!("clicked"); }
/// fn analytics(button: &Button) { /* track event */ }
///
/// let mut signals = SignalFnMulti::<Button, 4>::new();
/// signals.add(log_click);
/// signals.add(analytics);
/// signals.notify(&button);
/// ```
pub struct SignalFnMulti<T, const N: usize> {
    callbacks: [Option<fn(&T)>; N],
    count: usize,
}

impl<T, const N: usize> Default for SignalFnMulti<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> SignalFnMulti<T, N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            callbacks: [None; N],
            count: 0,
        }
    }

    /// Add a callback. Returns the index, or None if full.
    #[inline]
    pub fn add(&mut self, callback: fn(&T)) -> Option<usize> {
        if self.count >= N {
            return None;
        }
        let idx = self.count;
        self.callbacks[idx] = Some(callback);
        self.count += 1;
        Some(idx)
    }

    /// Remove callback at index by setting to None.
    /// Note: Does not compact the array.
    #[inline]
    pub fn remove(&mut self, index: usize) {
        if index < N {
            self.callbacks[index] = None;
        }
    }

    /// Clear all callbacks.
    #[inline]
    pub fn clear(&mut self) {
        self.callbacks = [None; N];
        self.count = 0;
    }

    /// Returns the number of callback slots used (including removed).
    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns true if no callbacks have been added.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns remaining capacity.
    #[inline]
    pub fn remaining(&self) -> usize {
        N - self.count
    }

    /// Notify all set callbacks with the given value.
    #[inline]
    pub fn notify(&self, value: &T) {
        for slot in &self.callbacks[..self.count] {
            if let Some(cb) = slot {
                cb(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_fn_basic() {
        static mut CALLED: bool = false;
        
        fn test_callback(_: &i32) {
            unsafe { CALLED = true; }
        }

        let mut signal = SignalFn::new();
        assert!(!signal.is_set());
        
        signal.set(test_callback);
        assert!(signal.is_set());
        
        signal.notify(&42);
        assert!(unsafe { CALLED });
    }

    #[test]
    fn signal_fn_multi_basic() {
        static mut CALL_COUNT: usize = 0;
        
        fn callback1(_: &i32) {
            unsafe { CALL_COUNT += 1; }
        }
        fn callback2(_: &i32) {
            unsafe { CALL_COUNT += 10; }
        }

        let mut signals = SignalFnMulti::<i32, 4>::new();
        signals.add(callback1);
        signals.add(callback2);
        
        assert_eq!(signals.len(), 2);
        assert_eq!(signals.remaining(), 2);
        
        signals.notify(&0);
        assert_eq!(unsafe { CALL_COUNT }, 11);
    }
}
