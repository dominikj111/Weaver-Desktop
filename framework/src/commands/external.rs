//! External event receiver for network and daemon communication.
//!
//! Provides a thread-safe channel for receiving commands from external sources
//! (network, workmeshd daemon, other processes) and forwarding them to the
//! application's command bus.

use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

/// Sender handle for external event sources.
///
/// This is `Send + Sync` and can be passed to other threads or processes.
/// Clone it for each event source that needs to send commands.
///
/// # Example
/// ```ignore
/// // In network handler thread
/// let sender = external_sender.clone();
/// std::thread::spawn(move || {
///     loop {
///         let cmd = receive_network_command();
///         sender.send(cmd);
///     }
/// });
/// ```
pub struct ExternalSender<C> {
    tx: Sender<C>,
}

impl<C> Clone for ExternalSender<C> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<C> ExternalSender<C> {
    /// Send a command from an external source.
    ///
    /// Returns `true` if sent successfully, `false` if the receiver was dropped.
    #[inline]
    pub fn send(&self, cmd: C) -> bool {
        self.tx.send(cmd).is_ok()
    }
}

/// Receiver for external events, polled from the main thread.
///
/// This lives in the main application and is polled each frame to
/// forward external commands to the command bus.
///
/// # Example
/// ```ignore
/// // In App::update()
/// external_receiver.poll(|cmd| {
///     command_bus.dispatch(cmd);
/// });
/// ```
pub struct ExternalReceiver<C> {
    rx: Receiver<C>,
}

impl<C> ExternalReceiver<C> {
    /// Non-blocking poll of all pending external events.
    ///
    /// Calls the handler for each received command. This never blocks -
    /// it returns immediately when no more events are available.
    #[inline]
    pub fn poll(&self, mut handler: impl FnMut(C)) {
        loop {
            match self.rx.try_recv() {
                Ok(cmd) => handler(cmd),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
    }

    /// Non-blocking poll with a maximum count.
    ///
    /// Returns the number of events processed.
    #[inline]
    pub fn poll_bounded(&self, max: usize, mut handler: impl FnMut(C)) -> usize {
        let mut count = 0;
        while count < max {
            match self.rx.try_recv() {
                Ok(cmd) => {
                    handler(cmd);
                    count += 1;
                }
                Err(_) => break,
            }
        }
        count
    }

    /// Check if the sender side has been dropped.
    ///
    /// If true, no more external events will ever arrive.
    pub fn is_disconnected(&self) -> bool {
        // Try to peek - Disconnected means sender dropped
        matches!(self.rx.try_recv(), Err(TryRecvError::Disconnected))
    }
}

/// Create a linked sender/receiver pair for external events.
///
/// # Example
/// ```ignore
/// let (sender, receiver) = external_channel::<AppCommand>();
///
/// // Give sender to network thread
/// let net_sender = sender.clone();
/// std::thread::spawn(move || {
///     // ... send commands via net_sender
/// });
///
/// // Poll receiver in main loop
/// receiver.poll(|cmd| bus.dispatch(cmd));
/// ```
pub fn external_channel<C>() -> (ExternalSender<C>, ExternalReceiver<C>) {
    let (tx, rx) = mpsc::channel();
    (ExternalSender { tx }, ExternalReceiver { rx })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_and_poll() {
        let (sender, receiver) = external_channel::<i32>();

        sender.send(1);
        sender.send(2);
        sender.send(3);

        let mut results = Vec::new();
        receiver.poll(|cmd| results.push(cmd));

        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_poll_bounded() {
        let (sender, receiver) = external_channel::<i32>();

        for i in 0..10 {
            sender.send(i);
        }

        let mut results = Vec::new();
        let count = receiver.poll_bounded(5, |cmd| results.push(cmd));

        assert_eq!(count, 5);
        assert_eq!(results, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_clone_sender() {
        let (sender, receiver) = external_channel::<i32>();
        let sender2 = sender.clone();

        sender.send(1);
        sender2.send(2);

        let mut results = Vec::new();
        receiver.poll(|cmd| results.push(cmd));

        assert_eq!(results, vec![1, 2]);
    }
}
