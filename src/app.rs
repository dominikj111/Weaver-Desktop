use std::path::PathBuf;

use weaver::{
    CommandBus, ExternalReceiver, IconContext, IconTheme, TaskSpawner, Theme, external_channel,
};
use weaver_desktop_shell::Shell;
use weaver_desktop_shell::commands::{AppCommand, Route, ToastKind};
use weaver_desktop_shell::views::{DemoIconPaths, DemoIcons, show_home};

/// Default path to the background image assets directory.
const DEFAULT_ASSETS_PATH: &str = "assets";
/// Default background image filename.
const DEFAULT_BACKGROUND_IMAGE: &str = "stock-adobe-weaver-birds-1836533864.png";
/// Default menu icon image filename.
const DEFAULT_MENU_ICON_IMAGE: &str = "weaven.png";
/// Default path to the Papirus icon theme (dev mode).
const DEFAULT_ICON_THEME_PATH: &str = "assets/icons/papirus-icon-theme/Papirus";

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
    /// Current theme for the application
    theme: Theme,
    shell: Shell,
    state: AppState,
    command_bus: CommandBus<AppCommand>,
    external_receiver: ExternalReceiver<AppCommand>,
    task_spawner: TaskSpawner<AppCommand>,
    /// Icon theme for loading icons by name
    icon_theme: IconTheme,
    /// Demo icon buttons for various categories
    demo_icons: DemoIcons,
}

impl App {
    /// Create the application with the given egui context.
    /// This initializes the theme and applies it to egui.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (external_sender, external_receiver) = external_channel();
        let task_spawner = TaskSpawner::new(external_sender);

        // Set up icon theme with dev mode path
        let mut icon_theme = IconTheme::new("Papirus");
        icon_theme.add_search_path(DEFAULT_ICON_THEME_PATH);

        // Initialize and apply the theme
        let theme = Theme::weaver_dark();
        theme.install(&cc.egui_ctx);

        // Create shell with theme colors
        let shell = Shell::with_theme(&theme);

        Self {
            theme,
            shell,
            state: AppState::default(),
            command_bus: CommandBus::new(),
            external_receiver,
            task_spawner,
            icon_theme,
            demo_icons: DemoIcons::new(),
        }
    }

    /// Switch to a new theme at runtime.
    pub fn set_theme(&mut self, ctx: &egui::Context, theme: Theme) {
        theme.install(ctx);
        self.shell.apply_theme(&theme);
        self.theme = theme;
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

        // 3. Resolve icon paths for demo buttons
        let icon_paths = DemoIconPaths {
            // Places
            folder: self.icon_theme.lookup("folder", 48, IconContext::Places),
            folder_documents: self
                .icon_theme
                .lookup("folder-documents", 48, IconContext::Places),
            user_home: self.icon_theme.lookup("user-home", 48, IconContext::Places),
            // Devices
            computer: self.icon_theme.lookup("computer", 48, IconContext::Devices),
            drive_harddisk: self
                .icon_theme
                .lookup("drive-harddisk", 48, IconContext::Devices),
            input_keyboard: self
                .icon_theme
                .lookup("input-keyboard", 48, IconContext::Devices),
            // Actions
            document_new: self
                .icon_theme
                .lookup("document-new", 48, IconContext::Actions),
            document_save: self
                .icon_theme
                .lookup("document-save", 48, IconContext::Actions),
            edit_find: self
                .icon_theme
                .lookup("edit-find", 48, IconContext::Actions),
            // Browsers
            firefox: self.icon_theme.lookup("firefox", 48, IconContext::Apps),
            google_chrome: self
                .icon_theme
                .lookup("google-chrome", 48, IconContext::Apps),
            // Editors & IDEs
            vscode: self.icon_theme.lookup("vscode", 48, IconContext::Apps),
            sublime_text: self
                .icon_theme
                .lookup("sublime-text", 48, IconContext::Apps),
            vim: self.icon_theme.lookup("vim", 48, IconContext::Apps),
            utilities_terminal: self
                .icon_theme
                .lookup("utilities-terminal", 48, IconContext::Apps),
            // Programming Languages
            python: self.icon_theme.lookup("python", 48, IconContext::Apps),
            java: self.icon_theme.lookup("java", 48, IconContext::Apps),
            // Creative Apps
            gimp: self.icon_theme.lookup("gimp", 48, IconContext::Apps),
            blender: self.icon_theme.lookup("blender", 48, IconContext::Apps),
            // Office
            libreoffice_writer: self
                .icon_theme
                .lookup("libreoffice-writer", 48, IconContext::Apps),
            libreoffice_calc: self
                .icon_theme
                .lookup("libreoffice-calc", 48, IconContext::Apps),
        };

        // 4. Render UI with updated state (new events dispatch to command_bus for next frame)
        let demo_icons = &mut self.demo_icons;
        self.shell.ui(
            ctx,
            self.state.background_image_path.as_deref(),
            self.state.menu_icon_path.as_deref(),
            |ui| {
                show_home(
                    ui,
                    &self.command_bus,
                    &self.task_spawner,
                    demo_icons,
                    &icon_paths,
                );
            },
        );
    }
}
