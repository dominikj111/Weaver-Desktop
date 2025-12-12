//! Application shell - manages persistent UI chrome and view rendering.

mod background;
mod bottom_bar;
mod log_panel;
mod terminal_panel;
mod top_bar;
mod top_menu;

pub use terminal_panel::TerminalPanel;

use std::path::Path;

use crate::{
    Component,
    components::{show_fullscreen_overlay, show_modal, show_overlay},
    widgets::calendar::show_calendar,
};
use background::Background;
use bottom_bar::BottomBar;
use egui::{Align2, Direction, Rect};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use log_panel::LogPanel;
use top_bar::TopBar;
use top_menu::Menu;

/// The application shell that owns all persistent UI elements.
/// Views are rendered in the central panel, with chrome around them.
pub struct Shell {
    background: Background,
    top_bar: TopBar,
    top_menu: Menu,
    bottom_bar: BottomBar,
    log_panel: LogPanel,
    terminal_panel: TerminalPanel,
    toasts: Toasts,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            background: Background::new(),
            top_bar: TopBar::default(),
            top_menu: Menu::new(),
            bottom_bar: BottomBar::default(),
            log_panel: LogPanel::new(),
            terminal_panel: TerminalPanel::new(),
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
    ///
    /// - `background_image_path`: Optional path to a background image to render behind all UI.
    pub fn ui(
        &mut self,
        ctx: &egui::Context,
        background_image_path: Option<&Path>,
        view: impl FnOnce(&mut egui::Ui),
    ) {
        let show_background = true;
        let mut central_rect: Rect = Rect::ZERO;

        if show_background {
            // Render background first (behind everything)
            self.background.ui(ctx, background_image_path);

            // Both image and fallback render to background layer, so panels need transparent frames
            // Top bar - semi-transparent dark overlay
            egui::TopBottomPanel::top("shell_top_bar")
                .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(128)))
                .show(ctx, |ui| {
                    self.top_bar.ui(ui);
                });

            // Bottom bar - semi-transparent dark overlay
            egui::TopBottomPanel::bottom("shell_bottom_bar")
                .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(128)))
                .show(ctx, |ui| {
                    self.bottom_bar.ui(ui);
                });

            // Central panel - transparent to show background
            central_rect = egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, view)
                .response
                .rect;
        } else {
            // Top bar - always rendered
            egui::TopBottomPanel::top("shell_top_bar").show(ctx, |ui| {
                self.top_bar.ui(ui);
            });

            // Bottom bar - always rendered
            egui::TopBottomPanel::bottom("shell_bottom_bar").show(ctx, |ui| {
                self.bottom_bar.ui(ui);
            });

            // Central panel - view content
            central_rect = egui::CentralPanel::default().show(ctx, view).response.rect;
        }

        // Overlays rendered last (on top)
        // TODO: menu, modals, toasts

        // Log panel - attached above bottom bar
        // self.log_panel.ui(ctx);

        // Terminal panel
        self.terminal_panel.ui_window(ctx, &mut true);

        // self.top_menu.ui(ctx);

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
        // show_overlay(ctx, central_rect, &ctx.style(), || {
        //     // menu_component.hide();
        //     println!("overlay click")
        // });

        // show_fullscreen_overlay(ctx, || println!("fullscreen overlay click"));

        self.toasts.show(ctx);
    }
}
