//! Application shell - manages persistent UI chrome and view rendering.

mod bottom_bar;
mod top_bar;
mod top_menu;

use crate::{
    Component,
    components::{show_fullscreen_overlay, show_modal, show_overlay},
    widgets::calendar::show_calendar,
};
use bottom_bar::BottomBar;
use egui::{Align2, Direction};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use top_bar::TopBar;
use top_menu::Menu;

/// The application shell that owns all persistent UI elements.
/// Views are rendered in the central panel, with chrome around them.
pub struct Shell {
    top_bar: TopBar,
    top_menu: Menu,
    bottom_bar: BottomBar,
    toasts: Toasts,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            top_bar: TopBar::default(),
            top_menu: Menu::new(),
            bottom_bar: BottomBar::default(),
            toasts: Toasts::new()
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .direction(Direction::TopDown)
                .order(egui::Order::Tooltip),
        }
    }
}

impl Shell {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render the shell with the given view content.
    pub fn ui(&mut self, ctx: &egui::Context, view: impl FnOnce(&mut egui::Ui)) {
        // Top bar - always rendered
        egui::TopBottomPanel::top("shell_top_bar").show(ctx, |ui| {
            self.top_bar.ui(ui);
        });

        // Bottom bar - always rendered
        egui::TopBottomPanel::bottom("shell_bottom_bar").show(ctx, |ui| {
            self.bottom_bar.ui(ui);
        });

        // Central panel - view content
        let central_rect = egui::CentralPanel::default().show(ctx, view).response.rect;

        // Overlays rendered last (on top)
        // TODO: menu, modals, toasts

        self.top_menu.ui(ctx);

        // let screen_rect = ctx.content_rect();
        // egui::Window::new("calendar_popup")
        //     .title_bar(false)
        //     .resizable(false)
        //     .collapsible(false)
        //     .order(egui::Order::Foreground)
        //     .fixed_pos(egui::pos2(screen_rect.center().x - 175.0, 60.0))
        //     .show(ctx, show_calendar);

        // show_modal(ctx, || {
        //     self.toasts.add(Toast {
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

        // Transparent overlay when menu is open (blocks interaction with central panel only)
        show_overlay(ctx, central_rect, &ctx.style(), || {
            // menu_component.hide();
            println!("overlay click")
        });

        // show_fullscreen_overlay(ctx, || println!("fullscreen overlay click"));

        self.toasts.show(ctx);
    }
}
