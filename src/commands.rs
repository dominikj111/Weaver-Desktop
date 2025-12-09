//! Application commands for SystemWeaver.
//!
//! Commands represent all state-changing actions in the application.
//! They can originate from:
//! - UI interactions (button clicks, menu selections)
//! - External sources (network, workmeshd daemon)
//! - Background tasks (progress updates, completion)
//! - Timers and scheduled events

use weaver::TaskId;

/// Application-level commands.
///
/// These flow through the command bus and are processed after rendering.
/// Add new variants as the application grows.
#[derive(Debug, Clone)]
pub enum AppCommand {
    // ─────────────────────────────────────────────────────────────
    // Navigation
    // ─────────────────────────────────────────────────────────────
    /// Navigate to a different view
    Navigate(Route),

    /// Go back to previous view
    NavigateBack,

    // ─────────────────────────────────────────────────────────────
    // UI State
    // ─────────────────────────────────────────────────────────────
    /// Show a toast notification
    ShowToast { message: String, kind: ToastKind },

    /// Toggle a UI element visibility
    TogglePanel(PanelId),

    // ─────────────────────────────────────────────────────────────
    // Task Progress (from background threads)
    // ─────────────────────────────────────────────────────────────
    /// Task started
    TaskStarted {
        task_id: TaskId,
        description: String,
    },

    /// Task progress update (0-100)
    TaskProgress {
        task_id: TaskId,
        progress: u8,
        message: Option<String>,
    },

    /// Task completed successfully
    TaskCompleted { task_id: TaskId, message: String },

    /// Task failed
    TaskFailed { task_id: TaskId, error: String },

    // ─────────────────────────────────────────────────────────────
    // System Operations (will delegate to workmeshd in future)
    // ─────────────────────────────────────────────────────────────
    /// Request package installation
    InstallPackage(String),

    /// Request service control
    ServiceControl { name: String, action: ServiceAction },

    // ─────────────────────────────────────────────────────────────
    // Hardware (will delegate to workmeshd in future)
    // ─────────────────────────────────────────────────────────────
    /// Set GPIO pin state
    SetGpioPin { pin: u8, high: bool },

    // ─────────────────────────────────────────────────────────────
    // Profile Management
    // ─────────────────────────────────────────────────────────────
    /// Load a profile by name
    LoadProfile(String),

    /// Apply the currently loaded profile
    ApplyProfile,
}

/// Application routes/views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Route {
    #[default]
    Home,
    Settings,
    Profiles,
    Hardware,
    System,
}

/// Toast notification severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Identifiers for toggleable panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelId {
    TopMenu,
    Calendar,
    Notifications,
}

/// Service control actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceAction {
    Start,
    Stop,
    Restart,
    Enable,
    Disable,
}
