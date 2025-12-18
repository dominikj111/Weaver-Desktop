//! Application shell - manages persistent UI chrome and view rendering.

mod background;
mod bar;
mod desktop_shell;
mod icon_button;
mod image_surface;
mod log_panel;
mod modal;
mod terminal_panel;
mod top_menu;
mod widget;

pub use bar::{Bar, BarPosition, BarStyle, SolidRounded, TransparentOverlay};
pub use desktop_shell::{
    DesktopShell, DesktopIcon, DesktopImageWidget, IconGridWidget,
    ClockWidget, DateWidget, MenuButton, StatusText, VersionLabel,
};
pub use icon_button::IconButton;
pub use image_surface::{ImageSource, ImageSurface, ScaleMode};
pub use modal::{Modal, ModalResult};
pub use terminal_panel::TerminalPanel;
pub use widget::{Align, Axis, Justify, Label, Size, Spacer, Spacing, Widget, WidgetContent};

use std::fmt::Write;
use std::path::Path;

// Keep Background for reference - now using ImageSurface instead
#[allow(dead_code)]
mod background_legacy {
    pub use super::background::Background;
}
use egui::{Align2, Direction, Rect};
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use log_panel::LogPanel;
use top_menu::Menu;
use weaver_lib::{InteractableHandlers, Theme};

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
    /// Desktop background surface - renders behind all UI
    background: ImageSurface,
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
        // Use the default weaver dark theme
        Self::with_theme(&Theme::weaver_dark())
    }
}

impl Shell {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a shell with styling from the given theme.
    pub fn with_theme(theme: &Theme) -> Self {
        let colors = &theme.colors;
        let spacing = &theme.spacing;

        Self {
            background: ImageSurface::with_id("shell_background"),
            top_bar: Bar::new(
                BarPosition::Top,
                SolidRounded {
                    color: colors.bar_top_bg,
                    rounding: spacing.rounding_large.nw as f32,
                    margin_x: spacing.padding_large,
                    margin_y: spacing.padding,
                },
            ),
            top_menu: Menu::new(),
            bottom_bar: Bar::new(
                BarPosition::Bottom,
                TransparentOverlay {
                    alpha: colors.bar_bottom_bg.a(),
                },
            ),
            log_panel: LogPanel::new(),
            terminal_panel: TerminalPanel::new(),
            toasts: Toasts::new()
                .anchor(Align2::LEFT_TOP, (10.0, 10.0))
                .direction(Direction::TopDown)
                .order(egui::Order::Tooltip),
            menu_button: IconButton::new("menu_icon", "☰")
                .with_size(spacing.icon_button_size)
                .with_background_color(colors.menu_button_bg)
                .with_stroke(egui::Stroke::new(2.0, colors.menu_button_stroke))
                .with_padding(spacing.padding_small)
                .with_on_click(on_menu_click)
                .with_on_press(on_menu_press)
                .with_on_release(on_menu_release),
        }
    }

    /// Update shell component colors from a theme.
    /// Call this when the theme changes at runtime.
    pub fn apply_theme(&mut self, theme: &Theme) {
        let colors = &theme.colors;
        let spacing = &theme.spacing;

        // Update top bar style
        let top_style = self.top_bar.style_mut();
        top_style.color = colors.bar_top_bg;
        top_style.rounding = spacing.rounding_large.nw as f32;

        // Update bottom bar style
        let bottom_style = self.bottom_bar.style_mut();
        bottom_style.alpha = colors.bar_bottom_bg.a();

        // Update menu button
        self.menu_button.set_background_color(colors.menu_button_bg);
        self.menu_button
            .set_stroke(egui::Stroke::new(2.0, colors.menu_button_stroke));
        self.menu_button.set_size(spacing.icon_button_size);
        self.menu_button.set_padding(spacing.padding_small);
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
            // Update source if path provided
            if let Some(path) = background_image_path {
                self.background.set_source(ImageSource::Image(path.to_path_buf()));
            }
            // Paint directly to background layer (like old Background component)
            let screen_rect = ctx.input(|i| i.screen_rect());
            self.background.paint_background(ctx, screen_rect);
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
