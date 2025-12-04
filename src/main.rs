mod components;
mod framework;
mod plugins;
mod services;
mod system_operations;
mod views;

// Define Component trait with minimal required methods
// Create EventBus wrapper around crossbeam
// Implement ComponentManager for dynamic registration
// Refactor existing views (menu.rs, calendar.rs) into Component impls
// Wire up main loop to use the new system

use egui::{Align2, Direction};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};

use components::{show_fullscreen_overlay, show_modal, show_overlay};

use views::{show_bottom_panel, show_calendar, show_top_panel, show_view};

use crate::{framework::component::Component, views::menu::Menu};

struct MyApp {
    name: String,
    age: u32,
    loading: bool,
    role: &'static str,
    toasts: Toasts,
    // menu_open: bool,
    calendar_open: bool,
    menu_component: Menu,
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
                .direction(Direction::TopDown)
                .order(egui::Order::Tooltip),
            // menu_open: true,
            calendar_open: false,
            menu_component: Menu::new(&[]),
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
            // menu_open,
            calendar_open,
            menu_component,
        } = self;

        // Top control panel with date/time center and menu right
        egui::TopBottomPanel::top("top_control_panel").show(ctx, show_top_panel);

        // Floating menu window (appears above overlay but below modals)
        // if *menu_open {
        menu_component.ui(ctx);
        // }

        // Calendar popup window
        if *calendar_open {
            let screen_rect = ctx.content_rect();
            egui::Window::new("calendar_popup")
                .title_bar(false)
                .resizable(false)
                .collapsible(false)
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(screen_rect.center().x - 175.0, 60.0))
                .show(ctx, show_calendar);
        }

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, show_bottom_panel);

        let central_rect = egui::CentralPanel::default()
            .show(ctx, show_view)
            .response
            .rect;

        // Transparent overlay when menu is open (blocks interaction with central panel only)
        // if *menu_open {
        show_overlay(ctx, central_rect, ctx.style(), || {
            menu_component.hide();
        });
        // }

        // show_modal(ctx, || {
        //     toasts.add(Toast {
        //         text: "Hello, World".into(),
        //         kind: ToastKind::Info,
        //         options: ToastOptions::default()
        //             .duration_in_seconds(10.0)
        //             .show_progress(true)
        //             .show_icon(true),
        //         ..Default::default()
        //     });

        //     println!("Modal should close");
        // });

        // show_fullscreen_overlay(ctx, || *menu_open = false);

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
