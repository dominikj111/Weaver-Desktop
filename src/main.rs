use egui::{Align2, ComboBox, Direction, Id, Modal};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

struct MyApp {
    name: String,
    age: u32,
    loading: bool,
    role: &'static str,
    toasts: Toasts,
    menu_open: bool,
}

impl MyApp {
    const ROLES: [&'static str; 2] = ["user", "admin"];
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            loading: false,
            role: Self::ROLES[0],
            toasts: Toasts::new()
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .direction(Direction::TopDown),
            menu_open: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            name,
            age,
            loading,
            role,
            toasts,
            menu_open,
        } = self;

        // Top control icon button
        egui::TopBottomPanel::top("top_control_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui.add(egui::Button::new(
                        egui::RichText::new("☰").size(24.0)
                    )).clicked() {
                        *menu_open = !*menu_open;
                    }
                });
            });

        // Floating menu window (appears above overlay but below modals)
        if *menu_open {
            let screen_rect = ctx.screen_rect();
            egui::Window::new("control_menu")
                .title_bar(false)
                .resizable(false)
                .collapsible(false)
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(
                    screen_rect.right() - 520.0,
                    50.0,
                ))
                .show(ctx, |ui| {
                    ui.set_width(500.0);
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(20.0, 10.0);
                        ui.add_space(10.0);
                        
                        // Menu icons in a single row
                        if ui.button("🏠\nHome").clicked() {
                            println!("Home clicked");
                            *menu_open = false;
                        }
                        if ui.button("📊\nDashboard").clicked() {
                            println!("Dashboard clicked");
                            *menu_open = false;
                        }
                        if ui.button("👤\nProfile").clicked() {
                            println!("Profile clicked");
                            *menu_open = false;
                        }
                        if ui.button("⚙\nSettings").clicked() {
                            println!("Settings clicked");
                            *menu_open = false;
                        }
                        if ui.button("📁\nFiles").clicked() {
                            println!("Files clicked");
                            *menu_open = false;
                        }
                        if ui.button("📈\nAnalytics").clicked() {
                            println!("Analytics clicked");
                            *menu_open = false;
                        }
                        ui.add_space(10.0);
                    });
                    ui.add_space(10.0);
                });
        }

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎨 Appearance Style Selector");
            });
        });

        let central_rect = egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.separator();

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(name).labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(age, 0..=120).text("age"));

            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    *age += 1;
                    // Simulate loading state
                    *loading = true;
                }

                // Toggle loading for demo purposes
                if ui.button("Toggle Loading").clicked() {
                    *loading = !*loading;
                }
            });

            // Display spinner when loading
            if *loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Processing...");
                });
            }

            ui.label(format!("Hello '{}', age {}", name, age));
            ui.separator();

        //     let modal = Modal::new(Id::new("Modal A")).show(ui.ctx(), |ui| {
        //         ui.set_width(250.0);

        //         ui.heading("Edit User");

        //         ui.label("Name:");
        //         ui.text_edit_singleline(name);

        //         ComboBox::new("role", "Role")
        //             .selected_text(*role)
        //             .show_ui(ui, |ui| {
        //                 for r in Self::ROLES {
        //                     ui.selectable_value(role, r, r);
        //                 }
        //             });

        //         ui.separator();

        //         egui::Sides::new().show(
        //             ui,
        //             |_ui| {},
        //             |ui| {
        //                 if ui.button("Save").clicked() {
        //                     toasts.add(Toast {
        //                         text: "Hello, World".into(),
        //                         kind: ToastKind::Info,
        //                         options: ToastOptions::default()
        //                             .duration_in_seconds(10.0)
        //                             .show_progress(true)
        //                             .show_icon(true),
        //                         ..Default::default()
        //                     });
        //                     // *save_modal_open = true;
        //                     println!("Save");
        //                 }
        //                 if ui.button("Cancel").clicked() {
        //                     // You can call `ui.close()` to close the modal.
        //                     // (This causes the current modals `should_close` to return true)
        //                     ui.close();
        //                 }
        //             },
        //         );
        //     });

        //     if modal.should_close() {
        //         // *user_modal_open = false;
        //         println!("Close");
        //     }
        }).response.rect;

        // Transparent overlay when menu is open (blocks interaction with central panel only)
        if *menu_open {
            egui::Area::new(egui::Id::new("menu_overlay"))
                .fixed_pos(central_rect.min)
                .order(egui::Order::Middle)
                .interactable(true)
                .show(ctx, |ui| {
                    let painter = ui.painter();
                    // Use the current theme's background color with high transparency
                    let bg_color = ctx.style().visuals.panel_fill;
                    let overlay_color = egui::Color32::from_rgba_unmultiplied(
                        bg_color.r(),
                        bg_color.g(),
                        bg_color.b(),
                        100, // High alpha for semi-transparent effect
                    );
                    painter.rect_filled(
                        central_rect,
                        0.0,
                        overlay_color,
                    );
                    
                    // Invisible button to capture all clicks on central panel
                    let response = ui.allocate_rect(central_rect, egui::Sense::click());
                    if response.clicked() {
                        *menu_open = false;
                    }
                });
        }

        // Render toasts - now using patched version with Order::Tooltip
        self.toasts.show(ctx);
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "My egui App",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
