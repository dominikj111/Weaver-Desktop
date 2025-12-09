//! Terminal panel component - embedded terminal emulator using alacritty backend.

use egui_term::{PtyEvent, TerminalBackend, TerminalView};
use std::sync::mpsc::{Receiver, Sender};

/// Terminal panel that provides an embedded terminal emulator.
/// Uses alacritty_terminal backend via egui_term.
pub struct TerminalPanel {
    backend: Option<TerminalBackend>,
    pty_receiver: Option<Receiver<(u64, PtyEvent)>>,
    pty_sender: Option<Sender<(u64, PtyEvent)>>,
    shell: String,
    initialized: bool,
    exited: bool,
}

impl Default for TerminalPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalPanel {
    pub fn new() -> Self {
        // Detect system shell
        #[cfg(unix)]
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        #[cfg(windows)]
        let shell = "cmd.exe".to_string();

        Self {
            backend: None,
            pty_receiver: None,
            pty_sender: None,
            shell,
            initialized: false,
            exited: false,
        }
    }

    /// Create with a specific shell (e.g., "/bin/zsh", "/bin/bash")
    pub fn with_shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = shell.into();
        self
    }

    /// Initialize the terminal backend. Must be called with egui Context.
    /// This is deferred because we need the Context for repaint requests.
    fn initialize(&mut self, ctx: &egui::Context) {
        if self.initialized {
            return;
        }

        let (sender, receiver) = std::sync::mpsc::channel();

        match TerminalBackend::new(
            0, // terminal id
            ctx.clone(),
            sender.clone(),
            egui_term::BackendSettings {
                shell: self.shell.clone(),
                ..Default::default()
            },
        ) {
            Ok(backend) => {
                self.backend = Some(backend);
                self.pty_receiver = Some(receiver);
                self.pty_sender = Some(sender);
                self.initialized = true;
            }
            Err(e) => {
                eprintln!("Failed to initialize terminal: {}", e);
            }
        }
    }

    /// Check if the terminal process has exited.
    pub fn has_exited(&self) -> bool {
        self.exited
    }

    /// Render the terminal panel.
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Lazy initialization
        if !self.initialized {
            self.initialize(ctx);
        }

        // Check for PTY events (exit, etc.)
        if let Some(ref receiver) = self.pty_receiver {
            if let Ok((_, PtyEvent::Exit)) = receiver.try_recv() {
                self.exited = true;
                return;
            }
        }

        // Render terminal if backend is available
        if let Some(ref mut backend) = self.backend {
            let terminal = TerminalView::new(ui, backend)
                .set_focus(true)
                .set_size(egui::vec2(ui.available_width(), ui.available_height()));

            ui.add(terminal);
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Terminal not available");
            });
        }
    }

    /// Render as a floating window (for overlay/popup use).
    pub fn ui_window(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Terminal")
            .open(open)
            .resizable(true)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                self.ui(ctx, ui);
            });
    }
}
