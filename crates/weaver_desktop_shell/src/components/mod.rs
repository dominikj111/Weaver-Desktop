//! Application shell - manages persistent UI chrome and view rendering.

mod background;
mod bar;
mod icon_button;
mod log_panel;
mod terminal_panel;
mod top_menu;

pub use bar::{Bar, BarPosition, BarStyle, SolidRounded, TransparentOverlay};
pub use icon_button::IconButton;
pub use terminal_panel::TerminalPanel;

use std::fmt::Write;
use std::path::Path;

use background::Background;
use egui::{Align2, Direction, Rect};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use log_panel::LogPanel;
use top_menu::Menu;
use weaver::InteractableHandlers;

/// Thread-local buffer for datetime formatting to avoid per-frame allocations.
thread_local! {
    static DATETIME_BUF: std::cell::RefCell<String> = std::cell::RefCell::new(String::with_capacity(64));
}

// Static function handlers for menu button (zero allocation)
fn on_menu_click(_btn: &IconButton) {
    println!("Menu button clicked");
}

fn on_menu_press(_btn: &IconButton) {
    println!("Menu button pressed");
}

fn on_menu_release(_btn: &IconButton) {
    println!("Menu button released");
}

/// The application shell that owns all persistent UI elements.
/// Views are rendered in the central panel, with chrome around them.
pub struct Shell {
    background: Background,
    top_bar: Bar<SolidRounded>,
    top_menu: Menu,
    bottom_bar: Bar<TransparentOverlay>,
    log_panel: LogPanel,
    terminal_panel: TerminalPanel,
    toasts: Toasts,
    /// Floating menu button rendered as an Area
    menu_button: IconButton,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            background: Background::new(),
            top_bar: Bar::new(
                BarPosition::Top,
                SolidRounded {
                    rounding: 24.0,
                    ..SolidRounded::default()
                },
            ),
            top_menu: Menu::new(),
            bottom_bar: Bar::new(BarPosition::Bottom, TransparentOverlay { alpha: 128 }),
            log_panel: LogPanel::new(),
            terminal_panel: TerminalPanel::new(),
            toasts: Toasts::new()
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .direction(Direction::TopDown)
                .order(egui::Order::Tooltip),
            menu_button: IconButton::new("menu_icon", "☰")
                .with_size(40.0)
                .with_background_color(egui::Color32::WHITE)
                .with_stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
                .with_padding(4.0)
                .with_on_click(on_menu_click)
                .with_on_press(on_menu_press)
                .with_on_release(on_menu_release),
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
    /// - `menu_icon_path`: Optional path to a PNG image for the menu icon.
    pub fn ui(
        &mut self,
        ctx: &egui::Context,
        background_image_path: Option<&Path>,
        menu_icon_path: Option<&Path>,
        view: impl FnOnce(&mut egui::Ui),
    ) {
        let show_background = true;
        let mut central_rect: Rect = Rect::ZERO;

        if show_background {
            // Render background first (behind everything)
            self.background.ui(ctx, background_image_path);
        }

        // Top bar with content
        let screen_rect = ctx.content_rect();
        self.top_bar.ui_floating(
            ctx,
            egui::pos2(10.0, 10.0),
            screen_rect.width() - 60.0,
            |ui| {
                ui.horizontal(|ui| {
                    // Left spacer
                    ui.add_space(ui.available_width() / 2.0 - 100.0);

                    // Center: Date/Time
                    let now = chrono::Local::now();
                    let date_time_str = DATETIME_BUF.with(|buf| {
                        let mut buf = buf.borrow_mut();
                        buf.clear();
                        write!(buf, "{}", now.format("%A, %B %d, %Y  %I:%M %p")).unwrap();
                        buf.clone()
                    });

                    if ui.button(&date_time_str).clicked() {
                        // TODO: toggle calendar popup
                    }
                });
            },
        );

        // Bottom bar with content
        self.bottom_bar.ui(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("v0.1.0");
                });
            });
        });

        // Central panel - view content
        central_rect = egui::CentralPanel::default()
            .frame(if show_background {
                egui::Frame::NONE
            } else {
                egui::Frame::default()
            })
            .show(ctx, view)
            .response
            .rect;

        // Floating menu button - rendered as Area above panels but below blocking overlay
        let screen_rect = ctx.content_rect();
        egui::Area::new(egui::Id::new("floating_menu_button"))
            .fixed_pos(egui::pos2(screen_rect.right() - 60.0, 7.5))
            .order(egui::Order::Middle)
            .interactable(true)
            .show(ctx, |ui| {
                self.menu_button.ui(ui, menu_icon_path);
            });

        // Overlays rendered last (on top)
        // TODO: menu, modals, toasts

        // Log panel - attached above bottom bar
        // self.log_panel.ui(ctx);

        // Terminal panel
        // self.terminal_panel.ui_window(ctx, &mut true);

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
