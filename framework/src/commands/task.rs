//! Async task spawning with progress reporting.
//!
//! Allows spawning long-running operations that communicate back to the UI
//! via the command bus without blocking rendering.

use std::thread;

use super::ExternalSender;

/// Context passed to spawned tasks for communicating back to the main thread.
///
/// This is `Send` and can be moved into spawned threads.
pub struct TaskContext<C> {
    sender: ExternalSender<C>,
    task_id: TaskId,
}

impl<C> TaskContext<C> {
    /// Send a command back to the main thread's command bus.
    ///
    /// Returns `false` if the receiver has been dropped (app shutting down).
    #[inline]
    pub fn send(&self, cmd: C) -> bool {
        self.sender.send(cmd)
    }

    /// Get the unique ID for this task.
    #[inline]
    pub fn task_id(&self) -> TaskId {
        self.task_id
    }
}

impl<C> Clone for TaskContext<C> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            task_id: self.task_id,
        }
    }
}

/// Unique identifier for a spawned task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

impl TaskId {
    /// Get the raw ID value.
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Spawns background tasks that can communicate back to the command bus.
///
/// # Example
/// ```ignore
/// let spawner = TaskSpawner::new(external_sender);
///
/// // Spawn a long-running task
/// let task_id = spawner.spawn(|ctx| {
///     ctx.send(AppCommand::TaskProgress { task_id: ctx.task_id(), progress: 0 });
///     
///     // Do heavy work...
///     let result = expensive_operation();
///     
///     ctx.send(AppCommand::TaskComplete { task_id: ctx.task_id(), result });
/// });
/// ```
pub struct TaskSpawner<C> {
    sender: ExternalSender<C>,
    next_id: std::sync::atomic::AtomicU64,
}

impl<C: Send + 'static> TaskSpawner<C> {
    /// Create a new task spawner that sends commands via the given sender.
    pub fn new(sender: ExternalSender<C>) -> Self {
        Self {
            sender,
            next_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Spawn a task that runs in a background thread.
    ///
    /// The task receives a `TaskContext` for sending commands back to the main thread.
    /// Returns a `TaskId` that can be used to track the task.
    ///
    /// # Panics
    /// The spawned thread will panic if the task function panics.
    pub fn spawn<F>(&self, task: F) -> TaskId
    where
        F: FnOnce(TaskContext<C>) + Send + 'static,
    {
        let task_id = TaskId(
            self.next_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        );
        let ctx = TaskContext {
            sender: self.sender.clone(),
            task_id,
        };

        thread::spawn(move || {
            task(ctx);
        });

        task_id
    }

    /// Spawn a task with a name (for debugging).
    ///
    /// The thread will be named, making it easier to identify in debuggers/profilers.
    pub fn spawn_named<F>(&self, name: &str, task: F) -> TaskId
    where
        F: FnOnce(TaskContext<C>) + Send + 'static,
    {
        let task_id = TaskId(
            self.next_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        );
        let ctx = TaskContext {
            sender: self.sender.clone(),
            task_id,
        };

        thread::Builder::new()
            .name(name.to_string())
            .spawn(move || {
                task(ctx);
            })
            .expect("failed to spawn thread");

        task_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::external_channel;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_spawn_and_communicate() {
        let (sender, receiver) = external_channel::<String>();
        let spawner = TaskSpawner::new(sender);

        let task_id = spawner.spawn(|ctx| {
            ctx.send(format!("started:{}", ctx.task_id().as_u64()));
            std::thread::sleep(Duration::from_millis(10));
            ctx.send(format!("done:{}", ctx.task_id().as_u64()));
        });

        // Wait for task to complete
        std::thread::sleep(Duration::from_millis(50));

        let mut messages = Vec::new();
        receiver.poll(|msg| messages.push(msg));

        assert_eq!(messages.len(), 2);
        assert!(messages[0].starts_with("started:"));
        assert!(messages[1].starts_with("done:"));
        assert_eq!(task_id.as_u64(), 1);
    }

    #[test]
    fn test_task_ids_are_unique() {
        let (sender, _receiver) = external_channel::<()>();
        let spawner = TaskSpawner::new(sender);

        let completed = Arc::new(AtomicBool::new(false));
        let completed_clone = completed.clone();

        let id1 = spawner.spawn(move |_| {
            completed_clone.store(true, Ordering::SeqCst);
        });
        let id2 = spawner.spawn(|_| {});
        let id3 = spawner.spawn(|_| {});

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);

        // Wait for completion
        while !completed.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
