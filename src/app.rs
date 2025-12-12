use std::path::PathBuf;

use weaver::{CommandBus, ExternalReceiver, TaskSpawner, external_channel};
use weaver_desktop_shell::Shell;
use weaver_desktop_shell::commands::{AppCommand, Route, ToastKind};
use weaver_desktop_shell::views::show_home;

/// Default path to the background image assets directory.
const DEFAULT_ASSETS_PATH: &str = "/Volumes/WORKING/Development/repositories/SystemWeaver/assets";
/// Default background image filename.
const DEFAULT_BACKGROUND_IMAGE: &str = "stock-adobe-weaver-birds-1836533864.png";
/// Default menu icon image filename.
const DEFAULT_MENU_ICON_IMAGE: &str = "weaven.png";

/// Application state that can be mutated by commands.
struct AppState {
    current_route: Route,
    /// Path to the background image (None = no background)
    background_image_path: Option<PathBuf>,
    /// Path to the menu icon image (None = use fallback character)
    menu_icon_path: Option<PathBuf>,
    // Add more state fields as needed
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_route: Route::default(),
            background_image_path: Some(
                PathBuf::from(DEFAULT_ASSETS_PATH).join(DEFAULT_BACKGROUND_IMAGE),
            ),
            menu_icon_path: Some(PathBuf::from(DEFAULT_ASSETS_PATH).join(DEFAULT_MENU_ICON_IMAGE)),
        }
    }
}

pub struct App {
    shell: Shell,
    state: AppState,
    command_bus: CommandBus<AppCommand>,
    external_receiver: ExternalReceiver<AppCommand>,
    task_spawner: TaskSpawner<AppCommand>,
}

impl App {
    pub fn new() -> Self {
        let (external_sender, external_receiver) = external_channel();
        let task_spawner = TaskSpawner::new(external_sender);
        Self {
            shell: Shell::new(),
            state: AppState::default(),
            command_bus: CommandBus::new(),
            external_receiver,
            task_spawner,
        }
    }

    /// Process a single command, mutating application state.
    fn handle_command(&mut self, cmd: AppCommand) {
        match cmd {
            AppCommand::Navigate(route) => {
                self.state.current_route = route;
                println!("{:?}", route);
            }
            AppCommand::NavigateBack => {
                // TODO: implement navigation history
                self.state.current_route = Route::Home;
            }
            AppCommand::ShowToast { message, kind } => {
                // TODO: integrate with shell toasts
                let kind_str = match kind {
                    ToastKind::Info => "INFO",
                    ToastKind::Success => "SUCCESS",
                    ToastKind::Warning => "WARNING",
                    ToastKind::Error => "ERROR",
                };
                println!("[{kind_str}] {message}");
            }
            AppCommand::TogglePanel(panel) => {
                // TODO: implement panel visibility state
                println!("Toggle panel: {:?}", panel);
            }
            AppCommand::TaskStarted {
                task_id,
                description,
            } => {
                // TODO: track active tasks in state
                println!("[Task {}] Started: {}", task_id.as_u64(), description);
            }
            AppCommand::TaskProgress {
                task_id,
                progress,
                message,
            } => {
                // TODO: update task progress in state
                let msg = message.as_deref().unwrap_or("");
                println!(
                    "[Task {}] Progress: {}% {}",
                    task_id.as_u64(),
                    progress,
                    msg
                );
            }
            AppCommand::TaskCompleted { task_id, message } => {
                // TODO: remove from active tasks, maybe show toast
                println!("[Task {}] Completed: {}", task_id.as_u64(), message);
            }
            AppCommand::TaskFailed { task_id, error } => {
                // TODO: remove from active tasks, show error toast
                eprintln!("[Task {}] Failed: {}", task_id.as_u64(), error);
            }
            AppCommand::InstallPackage(pkg) => {
                // TODO: delegate to workmeshd
                println!("Install package: {pkg}");
            }
            AppCommand::ServiceControl { name, action } => {
                // TODO: delegate to workmeshd
                println!("Service {name}: {:?}", action);
            }
            AppCommand::SetGpioPin { pin, high } => {
                // TODO: delegate to workmeshd
                println!("GPIO pin {pin}: {}", if high { "HIGH" } else { "LOW" });
            }
            AppCommand::LoadProfile(name) => {
                // TODO: implement profile loading
                println!("Load profile: {name}");
            }
            AppCommand::ApplyProfile => {
                // TODO: implement profile application
                println!("Apply profile");
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Poll external events (network, daemon) - non-blocking
        self.external_receiver.poll(|cmd| {
            self.command_bus.dispatch(cmd);
        });

        // 2. Process ALL pending commands BEFORE rendering (state is fresh for UI)
        for cmd in self.command_bus.collect_all() {
            self.handle_command(cmd);
        }

        // 3. Render UI with updated state (new events dispatch to command_bus for next frame)
        self.shell.ui(
            ctx,
            self.state.background_image_path.as_deref(),
            self.state.menu_icon_path.as_deref(),
            |ui| {
                show_home(ui, &self.command_bus, &self.task_spawner);
            },
        );
    }
}
