//! Home view - the main application view.

use std::path::PathBuf;

use egui::Ui;
use weaver::{CommandBus, TaskSpawner};

use crate::commands::{AppCommand, Route, ToastKind};
use crate::components::IconButton;

/// Demo icon buttons for showcasing the icon theme.
pub struct DemoIcons {
    // Places
    pub folder: IconButton,
    pub folder_documents: IconButton,
    pub user_home: IconButton,
    // Devices
    pub computer: IconButton,
    pub drive_harddisk: IconButton,
    pub input_keyboard: IconButton,
    // Actions
    pub document_new: IconButton,
    pub document_save: IconButton,
    pub edit_find: IconButton,
    // Browsers
    pub firefox: IconButton,
    pub google_chrome: IconButton,
    // Editors & IDEs
    pub vscode: IconButton,
    pub sublime_text: IconButton,
    pub vim: IconButton,
    pub utilities_terminal: IconButton,
    // Programming Languages
    pub python: IconButton,
    pub java: IconButton,
    // Creative Apps
    pub gimp: IconButton,
    pub blender: IconButton,
    // Office
    pub libreoffice_writer: IconButton,
    pub libreoffice_calc: IconButton,
}

impl DemoIcons {
    pub fn new() -> Self {
        let size = 48.0;
        Self {
            // Places
            folder: IconButton::new("icon_folder", "📁").with_size(size),
            folder_documents: IconButton::new("icon_folder_documents", "📄").with_size(size),
            user_home: IconButton::new("icon_user_home", "🏠").with_size(size),
            // Devices
            computer: IconButton::new("icon_computer", "💻").with_size(size),
            drive_harddisk: IconButton::new("icon_drive_harddisk", "💾").with_size(size),
            input_keyboard: IconButton::new("icon_input_keyboard", "⌨").with_size(size),
            // Actions
            document_new: IconButton::new("icon_document_new", "📝").with_size(size),
            document_save: IconButton::new("icon_document_save", "💾").with_size(size),
            edit_find: IconButton::new("icon_edit_find", "🔍").with_size(size),
            // Browsers
            firefox: IconButton::new("icon_firefox", "🦊").with_size(size),
            google_chrome: IconButton::new("icon_google_chrome", "🌐").with_size(size),
            // Editors & IDEs
            vscode: IconButton::new("icon_vscode", "VS").with_size(size),
            sublime_text: IconButton::new("icon_sublime_text", "ST").with_size(size),
            vim: IconButton::new("icon_vim", "Vi").with_size(size),
            utilities_terminal: IconButton::new("icon_utilities_terminal", ">_").with_size(size),
            // Programming Languages
            python: IconButton::new("icon_python", "🐍").with_size(size),
            java: IconButton::new("icon_java", "☕").with_size(size),
            // Creative Apps
            gimp: IconButton::new("icon_gimp", "🎨").with_size(size),
            blender: IconButton::new("icon_blender", "🪣").with_size(size),
            // Office
            libreoffice_writer: IconButton::new("icon_libreoffice_writer", "W").with_size(size),
            libreoffice_calc: IconButton::new("icon_libreoffice_calc", "C").with_size(size),
        }
    }
}

/// Resolved icon paths for the demo icons.
pub struct DemoIconPaths {
    // Places
    pub folder: Option<PathBuf>,
    pub folder_documents: Option<PathBuf>,
    pub user_home: Option<PathBuf>,
    // Devices
    pub computer: Option<PathBuf>,
    pub drive_harddisk: Option<PathBuf>,
    pub input_keyboard: Option<PathBuf>,
    // Actions
    pub document_new: Option<PathBuf>,
    pub document_save: Option<PathBuf>,
    pub edit_find: Option<PathBuf>,
    // Browsers
    pub firefox: Option<PathBuf>,
    pub google_chrome: Option<PathBuf>,
    // Editors & IDEs
    pub vscode: Option<PathBuf>,
    pub sublime_text: Option<PathBuf>,
    pub vim: Option<PathBuf>,
    pub utilities_terminal: Option<PathBuf>,
    // Programming Languages
    pub python: Option<PathBuf>,
    pub java: Option<PathBuf>,
    // Creative Apps
    pub gimp: Option<PathBuf>,
    pub blender: Option<PathBuf>,
    // Office
    pub libreoffice_writer: Option<PathBuf>,
    pub libreoffice_calc: Option<PathBuf>,
}

pub fn show_home(
    ui: &mut Ui,
    bus: &CommandBus<AppCommand>,
    spawner: &TaskSpawner<AppCommand>,
    demo_icons: &mut DemoIcons,
    icon_paths: &DemoIconPaths,
) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading("Welcome to SystemWeaver");
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            if ui.button("Settings").clicked() {
                bus.dispatch(AppCommand::Navigate(Route::Settings));
            }
            if ui.button("Profiles").clicked() {
                bus.dispatch(AppCommand::Navigate(Route::Profiles));
            }
            if ui.button("Hardware").clicked() {
                bus.dispatch(AppCommand::Navigate(Route::Hardware));
            }
        });

        ui.add_space(10.0);

        if ui.button("Show Toast").clicked() {
            bus.dispatch(AppCommand::ShowToast {
                message: "Hello from SystemWeaver!".to_string(),
                kind: ToastKind::Info,
            });
        }

        ui.add_space(10.0);

        // Demo icon grid using Papirus icon theme
        ui.add_space(20.0);
        
        // Places category
        ui.label("Places:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.folder, icon_paths.folder.as_deref(), "folder", bus);
            render_icon_with_label(ui, &mut demo_icons.folder_documents, icon_paths.folder_documents.as_deref(), "folder-documents", bus);
            render_icon_with_label(ui, &mut demo_icons.user_home, icon_paths.user_home.as_deref(), "user-home", bus);
        });
        
        ui.add_space(10.0);
        
        // Devices category
        ui.label("Devices:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.computer, icon_paths.computer.as_deref(), "computer", bus);
            render_icon_with_label(ui, &mut demo_icons.drive_harddisk, icon_paths.drive_harddisk.as_deref(), "drive-harddisk", bus);
            render_icon_with_label(ui, &mut demo_icons.input_keyboard, icon_paths.input_keyboard.as_deref(), "input-keyboard", bus);
        });
        
        ui.add_space(10.0);
        
        // Actions category
        ui.label("Actions:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.document_new, icon_paths.document_new.as_deref(), "document-new", bus);
            render_icon_with_label(ui, &mut demo_icons.document_save, icon_paths.document_save.as_deref(), "document-save", bus);
            render_icon_with_label(ui, &mut demo_icons.edit_find, icon_paths.edit_find.as_deref(), "edit-find", bus);
        });
        
        ui.add_space(10.0);
        
        // Browsers
        ui.label("Browsers:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.firefox, icon_paths.firefox.as_deref(), "firefox", bus);
            render_icon_with_label(ui, &mut demo_icons.google_chrome, icon_paths.google_chrome.as_deref(), "google-chrome", bus);
        });
        
        ui.add_space(10.0);
        
        // Editors & IDEs
        ui.label("Editors & IDEs:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.vscode, icon_paths.vscode.as_deref(), "vscode", bus);
            render_icon_with_label(ui, &mut demo_icons.sublime_text, icon_paths.sublime_text.as_deref(), "sublime-text", bus);
            render_icon_with_label(ui, &mut demo_icons.vim, icon_paths.vim.as_deref(), "vim", bus);
            render_icon_with_label(ui, &mut demo_icons.utilities_terminal, icon_paths.utilities_terminal.as_deref(), "terminal", bus);
        });
        
        ui.add_space(10.0);
        
        // Programming Languages
        ui.label("Languages:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.python, icon_paths.python.as_deref(), "python", bus);
            render_icon_with_label(ui, &mut demo_icons.java, icon_paths.java.as_deref(), "java", bus);
        });
        
        ui.add_space(10.0);
        
        // Creative Apps
        ui.label("Creative:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.gimp, icon_paths.gimp.as_deref(), "gimp", bus);
            render_icon_with_label(ui, &mut demo_icons.blender, icon_paths.blender.as_deref(), "blender", bus);
        });
        
        ui.add_space(10.0);
        
        // Office
        ui.label("Office:");
        ui.horizontal(|ui| {
            render_icon_with_label(ui, &mut demo_icons.libreoffice_writer, icon_paths.libreoffice_writer.as_deref(), "writer", bus);
            render_icon_with_label(ui, &mut demo_icons.libreoffice_calc, icon_paths.libreoffice_calc.as_deref(), "calc", bus);
        });
        
        ui.add_space(20.0);
        
        if ui.button("Run Background Task").clicked() {
            // Spawn a simulated long-running task
            spawner.spawn(|ctx| {
                use std::thread::sleep;
                use std::time::Duration;

                ctx.send(AppCommand::TaskStarted {
                    task_id: ctx.task_id(),
                    description: "Simulated heavy operation".to_string(),
                });

                // Simulate work with progress updates
                for i in 0..=10 {
                    sleep(Duration::from_millis(200));
                    ctx.send(AppCommand::TaskProgress {
                        task_id: ctx.task_id(),
                        progress: i * 10,
                        message: Some(format!("Step {}/10", i)),
                    });
                }

                ctx.send(AppCommand::TaskCompleted {
                    task_id: ctx.task_id(),
                    message: "Operation finished successfully".to_string(),
                });
            });
        }
    });
}

/// Helper to render an icon button with a label below it.
fn render_icon_with_label(
    ui: &mut Ui,
    button: &mut IconButton,
    icon_path: Option<&std::path::Path>,
    name: &str,
    bus: &CommandBus<AppCommand>,
) {
    ui.vertical(|ui| {
        ui.set_width(80.0);
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            if button.ui(ui, icon_path).clicked() {
                bus.dispatch(AppCommand::ShowToast {
                    message: format!("Clicked: {}", name),
                    kind: ToastKind::Info,
                });
            }
            ui.label(name);
        });
    });
}
