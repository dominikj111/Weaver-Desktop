//! Home view - the main application view.

use egui::Ui;
use weaver::{CommandBus, TaskSpawner};

use crate::commands::{AppCommand, Route, ToastKind};

pub fn show_home(ui: &mut Ui, bus: &CommandBus<AppCommand>, spawner: &TaskSpawner<AppCommand>) {
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
