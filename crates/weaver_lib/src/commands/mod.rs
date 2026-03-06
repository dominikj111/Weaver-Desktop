//! Command dispatch system for application events.
//!
//! This module provides the infrastructure for a unidirectional data flow:
//!
//! ```text
//! ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//! │   WidgetStr    │ ──▶ │ CommandBus  │ ──▶ │  App State  │
//! │   Events    │     │  (queue)    │     │  (mutate)   │
//! └─────────────┘     └─────────────┘     └─────────────┘
//!                           ▲
//!                           │
//!                     ┌─────────────┐
//!                     │  External   │
//!                     │  (network,  │
//!                     │   daemon)   │
//!                     └─────────────┘
//! ```
//!
//! # Architecture
//!
//! - **CommandBus**: Single-threaded queue for UI-originated commands
//! - **ExternalReceiver**: Thread-safe channel for network/daemon commands
//! - Commands are processed after rendering to avoid borrow conflicts
//!
//! # Usage
//!
//! ```ignore
//! use weaver_lib::commands::{CommandBus, external_channel};
//!
//! // Define your command type
//! enum AppCommand {
//!     Navigate(Route),
//!     UpdateSetting(String, String),
//! }
//!
//! // Create the bus (typically as thread_local or in App struct)
//! let bus: CommandBus<AppCommand> = CommandBus::new();
//!
//! // In event handlers (immutable context)
//! bus.dispatch(AppCommand::Navigate(Route::Settings));
//!
//! // After render (mutable context)
//! bus.drain(|cmd| match cmd {
//!     AppCommand::Navigate(route) => state.current_route = route,
//!     AppCommand::UpdateSetting(k, v) => state.settings.insert(k, v),
//! });
//! ```

mod bus;
mod external;
mod task;

pub use bus::CommandBus;
pub use external::{external_channel, ExternalReceiver, ExternalSender};
pub use task::{TaskContext, TaskId, TaskSpawner};
