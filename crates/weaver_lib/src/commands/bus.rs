//! Command bus for dispatching and processing application commands.
//!
//! Provides a bounded, non-blocking queue for commands that can be dispatched
//! from event handlers and processed after rendering.

use std::cell::RefCell;
use std::collections::VecDeque;

/// Bounded command bus for single-threaded event dispatch.
///
/// Commands are queued during event handling (inside `SignalFn` callbacks)
/// and processed after rendering to avoid borrow conflicts.
///
/// # Type Parameters
/// - `C`: The command type
/// - `N`: Maximum queue capacity (compile-time constant for predictable memory)
///
/// # Example
/// ```ignore
/// let bus: CommandBus<AppCommand, 32> = CommandBus::new();
///
/// // In event handler (borrows immutably)
/// bus.dispatch(AppCommand::Navigate(Route::Settings));
///
/// // After render (can mutate state)
/// bus.drain(|cmd| handle_command(cmd));
/// ```
pub struct CommandBus<C, const N: usize = 32> {
    queue: RefCell<VecDeque<C>>,
}

impl<C, const N: usize> Default for CommandBus<C, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C, const N: usize> CommandBus<C, N> {
    /// Create a new command bus with capacity N.
    #[inline]
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(VecDeque::with_capacity(N)),
        }
    }

    /// Dispatch a command to the queue.
    ///
    /// Returns `true` if the command was queued, `false` if the queue is full.
    /// When full, the command is dropped (backpressure).
    ///
    /// This method uses interior mutability (`RefCell`) so it can be called
    /// from immutable contexts like `SignalFn` callbacks.
    #[inline]
    pub fn dispatch(&self, cmd: C) -> bool {
        let mut q = self.queue.borrow_mut();
        if q.len() >= N {
            // Queue full - apply backpressure by dropping
            // In debug builds, this could log a warning
            #[cfg(debug_assertions)]
            eprintln!("[CommandBus] Queue full, dropping command");
            return false;
        }
        q.push_back(cmd);
        true
    }

    /// Process all queued commands.
    ///
    /// Call this after rendering when you have mutable access to application state.
    #[inline]
    pub fn drain(&self, mut handler: impl FnMut(C)) {
        let mut q = self.queue.borrow_mut();
        while let Some(cmd) = q.pop_front() {
            handler(cmd);
        }
    }

    /// Process up to `max` commands, returning the number processed.
    ///
    /// Use this to bound processing time per frame. If this returns `max`,
    /// there may be more commands pending - call `request_repaint()`.
    #[inline]
    pub fn drain_bounded(&self, max: usize, mut handler: impl FnMut(C)) -> usize {
        let mut q = self.queue.borrow_mut();
        let mut count = 0;
        while count < max {
            match q.pop_front() {
                Some(cmd) => {
                    handler(cmd);
                    count += 1;
                }
                None => break,
            }
        }
        count
    }

    /// Returns the number of pending commands.
    #[inline]
    pub fn len(&self) -> usize {
        self.queue.borrow().len()
    }

    /// Returns true if no commands are pending.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.queue.borrow().is_empty()
    }

    /// Returns remaining capacity before backpressure.
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        N.saturating_sub(self.queue.borrow().len())
    }

    /// Clear all pending commands without processing.
    #[inline]
    pub fn clear(&self) {
        self.queue.borrow_mut().clear();
    }

    /// Collect up to `max` commands into a Vec.
    ///
    /// This is useful when you need to process commands with `&mut self`
    /// access to other state, avoiding borrow conflicts.
    ///
    /// # Example
    /// ```ignore
    /// for cmd in bus.collect_bounded(16) {
    ///     self.handle_command(cmd);
    /// }
    /// ```
    #[inline]
    pub fn collect_bounded(&self, max: usize) -> Vec<C> {
        let mut q = self.queue.borrow_mut();
        let count = max.min(q.len());
        q.drain(..count).collect()
    }

    /// Collect all pending commands into a Vec.
    #[inline]
    pub fn collect_all(&self) -> Vec<C> {
        let mut q = self.queue.borrow_mut();
        q.drain(..).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_and_drain() {
        let bus: CommandBus<i32, 8> = CommandBus::new();

        bus.dispatch(1);
        bus.dispatch(2);
        bus.dispatch(3);

        let mut results = Vec::new();
        bus.drain(|cmd| results.push(cmd));

        assert_eq!(results, vec![1, 2, 3]);
        assert!(bus.is_empty());
    }

    #[test]
    fn test_backpressure() {
        let bus: CommandBus<i32, 4> = CommandBus::new();

        assert!(bus.dispatch(1));
        assert!(bus.dispatch(2));
        assert!(bus.dispatch(3));
        assert!(bus.dispatch(4));
        assert!(!bus.dispatch(5)); // Should fail - queue full

        assert_eq!(bus.len(), 4);
    }

    #[test]
    fn test_drain_bounded() {
        let bus: CommandBus<i32, 8> = CommandBus::new();

        for i in 0..6 {
            bus.dispatch(i);
        }

        let mut results = Vec::new();
        let processed = bus.drain_bounded(3, |cmd| results.push(cmd));

        assert_eq!(processed, 3);
        assert_eq!(results, vec![0, 1, 2]);
        assert_eq!(bus.len(), 3); // 3 remaining
    }
}
