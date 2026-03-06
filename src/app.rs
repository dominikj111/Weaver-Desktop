use std::path::PathBuf;

use weaver_desktop_shell::commands::{AppCommand, Route, ToastKind};
use weaver_desktop_shell::{
    DesktopIcon, DesktopImageWidget, DesktopShell, IconGridWidget, ImageSource, ScaleMode, Size,
    Spacing, WidgetStr,
};
use weaver_lib::{
    CommandBus, ExternalReceiver, IconContext, IconTheme, TaskSpawner, Theme, external_channel,
};

/// Default path to the background image assets directory.
const DEFAULT_ASSETS_PATH: &str = "assets";
/// Default background image filename.
const DEFAULT_BACKGROUND_IMAGE: &str = "xp_wallpaper.png";
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
    shell: DesktopShell,
    state: AppState,
    command_bus: CommandBus<AppCommand>,
    external_receiver: ExternalReceiver<AppCommand>,
    task_spawner: TaskSpawner<AppCommand>,
    /// Icon theme for loading icons by name
    icon_theme: IconTheme,
}

impl App {
    /// Create the application with the given egui context.
    /// This initializes the theme and applies it to egui.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // DPI - scaling (when working on custom DRM backend)
        // let current_scale = cc.egui_ctx.pixels_per_point();
        // println!("Current scale: {}", current_scale);
        // cc.egui_ctx.set_pixels_per_point(1.0);

        let (external_sender, external_receiver) = external_channel();
        let task_spawner = TaskSpawner::new(external_sender);

        // Set up icon theme with dev mode path
        let mut icon_theme = IconTheme::new("Papirus");
        icon_theme.add_search_path(DEFAULT_ICON_THEME_PATH);

        // Initialize and apply the theme
        let theme = Theme::weaver_dark();
        theme.install(&cc.egui_ctx);

        // Build desktop content widgets
        let content_widgets = Self::build_desktop_content(&mut icon_theme);

        // Create shell with desktop widgets
        let mut shell = DesktopShell::with_content(content_widgets);

        // Set background
        let bg_path = PathBuf::from(DEFAULT_ASSETS_PATH).join(DEFAULT_BACKGROUND_IMAGE);
        if bg_path.exists() {
            shell.set_background_image(&bg_path);
        }

        Self {
            theme,
            shell,
            state: AppState::default(),
            command_bus: CommandBus::new(),
            external_receiver,
            task_spawner,
            icon_theme,
        }
    }

    /// Build the desktop content widgets (icon grids, images, etc.)
    fn build_desktop_content(icon_theme: &mut IconTheme) -> Vec<WidgetStr> {
        // Places icon grid
        let places_icons = vec![
            DesktopIcon::new("Home", "places.home").with_icon(
                icon_theme
                    .lookup("user-home", 48, IconContext::Places)
                    .unwrap_or_default(),
            ),
            DesktopIcon::new("Documents", "places.documents").with_icon(
                icon_theme
                    .lookup("folder-documents", 48, IconContext::Places)
                    .unwrap_or_default(),
            ),
            DesktopIcon::new("Downloads", "places.downloads").with_icon(
                icon_theme
                    .lookup("folder-download", 48, IconContext::Places)
                    .unwrap_or_default(),
            ),
        ];

        let places_widget = WidgetStr::leaf(
            "places-grid",
            IconGridWidget::new()
                .with_icons(places_icons)
                .icon_size(48.0)
                .spacing(12.0)
                .columns(3),
        )
        .width(Size::Fixed(220.0))
        .height(Size::Fixed(200.0))
        .background(weaver_desktop_shell::ImageSurface::with_source(
            ImageSource::Color(egui::Color32::from_rgba_unmultiplied(30, 30, 30, 180)),
        ))
        .border_radius(12.0)
        .padding(Spacing::all(12.0));

        // Devices icon grid
        let devices_icons = vec![
            DesktopIcon::new("Computer", "devices.computer").with_icon(
                icon_theme
                    .lookup("computer", 48, IconContext::Devices)
                    .unwrap_or_default(),
            ),
            DesktopIcon::new("Disk", "devices.disk").with_icon(
                icon_theme
                    .lookup("drive-harddisk", 48, IconContext::Devices)
                    .unwrap_or_default(),
            ),
            DesktopIcon::new("Keyboard", "devices.keyboard").with_icon(
                icon_theme
                    .lookup("input-keyboard", 48, IconContext::Devices)
                    .unwrap_or_default(),
            ),
        ];

        let devices_widget = WidgetStr::leaf(
            "devices-grid",
            IconGridWidget::new()
                .with_icons(devices_icons)
                .icon_size(48.0)
                .spacing(12.0)
                .columns(3),
        )
        .width(Size::Fixed(220.0))
        .height(Size::Fixed(200.0))
        .background(weaver_desktop_shell::ImageSurface::with_source(
            ImageSource::Color(egui::Color32::from_rgba_unmultiplied(30, 30, 30, 180)),
        ))
        .border_radius(12.0)
        .padding(Spacing::all(12.0));

        // Image widget (photo frame)
        let image_widget = WidgetStr::leaf(
            "photo-frame",
            DesktopImageWidget::new()
                .with_image(PathBuf::from(DEFAULT_ASSETS_PATH).join(DEFAULT_BACKGROUND_IMAGE))
                .title("Weaver Birds")
                .scale_mode(ScaleMode::Cover),
        )
        .width(Size::Fixed(200.0))
        .height(Size::Fixed(150.0));

        vec![places_widget, devices_widget, image_widget]
    }

    /// Switch to a new theme at runtime.
    pub fn set_theme(&mut self, ctx: &egui::Context, theme: Theme) {
        theme.install(ctx);
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

        // 3. Render UI with updated state
        self.shell.ui(ctx, |_ui| {
            // View content is now provided via desktop widgets
        });
    }
}
