use chrono::Datelike;
use egui::{Align2, ComboBox, Direction, Id, Modal};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

fn days_in_month(year: i32, month: u32) -> u32 {
    chrono::NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}

struct MyApp {
    name: String,
    age: u32,
    loading: bool,
    role: &'static str,
    toasts: Toasts,
    menu_open: bool,
    calendar_open: bool,
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
            calendar_open: false,
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
            calendar_open,
        } = self;

        // Top control panel with date/time center and menu right
        egui::TopBottomPanel::top("top_control_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Left spacer
                    ui.add_space(ui.available_width() / 2.0 - 100.0);

                    // Center: Date/Time
                    let now = chrono::Local::now();
                    let date_time_str = now.format("%A, %B %d, %Y  %I:%M %p").to_string();
                    if ui.button(&date_time_str).clicked() {
                        *calendar_open = !*calendar_open;
                    }

                    // Right: Menu icon
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        if ui
                            .add(egui::Button::new(egui::RichText::new("☰").size(24.0)))
                            .clicked()
                        {
                            *menu_open = !*menu_open;
                        }
                    });
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
                .fixed_pos(egui::pos2(screen_rect.right() - 520.0, 50.0))
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

        // Calendar popup window
        if *calendar_open {
            let screen_rect = ctx.screen_rect();
            egui::Window::new("calendar_popup")
                .title_bar(false)
                .resizable(false)
                .collapsible(false)
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(screen_rect.center().x - 175.0, 60.0))
                .show(ctx, |ui| {
                    ui.set_width(350.0);
                    ui.add_space(10.0);

                    let now = chrono::Local::now();

                    // Month and Year header
                    ui.vertical_centered(|ui| {
                        ui.heading(now.format("%B %Y").to_string());
                    });
                    ui.add_space(5.0);

                    // Day headers
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                        for day in ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] {
                            ui.label(egui::RichText::new(day).strong().size(12.0));
                            ui.add_space(15.0);
                        }
                    });

                    ui.add_space(5.0);

                    // Calendar grid
                    let first_day = now.with_day(1).unwrap();
                    let first_weekday = first_day.weekday().num_days_from_sunday() as usize;
                    let days_in_month = days_in_month(now.year(), now.month()) as usize;
                    let today = now.day() as usize;

                    let mut day = 1;
                    let total_cells = ((first_weekday + days_in_month + 6) / 7) * 7;

                    for week in 0..(total_cells / 7) {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                            for weekday in 0..7 {
                                let cell_index = week * 7 + weekday;
                                if cell_index < first_weekday || day > days_in_month {
                                    ui.add_space(35.0);
                                } else {
                                    let is_today = day == today;
                                    let day_text = if is_today {
                                        egui::RichText::new(format!("{}", day))
                                            .strong()
                                            .color(egui::Color32::from_rgb(0, 120, 215))
                                    } else {
                                        egui::RichText::new(format!("{}", day))
                                    };
                                    ui.label(day_text);
                                    ui.add_space(if day < 10 { 23.0 } else { 17.0 });
                                    day += 1;
                                }
                            }
                        });
                    }

                    ui.add_space(10.0);
                });
        }

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎨 Appearance Style Selector");
            });
        });

        let central_rect = egui::CentralPanel::default()
            .show(ctx, |ui| {
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
            })
            .response
            .rect;

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
                    painter.rect_filled(central_rect, 0.0, overlay_color);

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
