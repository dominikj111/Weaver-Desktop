//! Theme system for Weaver Desktop UI.
//!
//! Provides semantic color and spacing tokens that derive from egui's built-in
//! theming system while extending it for application-specific needs.
//!
//! # Architecture
//!
//! - `Theme`: Complete theme combining egui Visuals + semantic tokens
//! - `ThemeColors`: Named color meanings derived from Visuals
//! - `ThemeSpacing`: Consistent spacing/sizing values
//!
//! # Usage
//!
//! ```rust,ignore
//! // At startup
//! let theme = Theme::weaver_dark();
//! theme.apply(&ctx);
//!
//! // Store in egui data for global access
//! theme.store(&ctx);
//!
//! // Anywhere in UI code
//! let theme = Theme::get(&ctx);
//! let bg = theme.colors.surface;
//! ```
//!
//! # Compatibility
//!
//! The theme system is designed to work with third-party theme crates:
//! ```rust,ignore
//! // Use catppuccin colors but get semantic tokens
//! let theme = Theme::from_visuals(catppuccin_egui::MOCHA.visuals());
//! ```

use egui::{Color32, Context, CornerRadius, Id, Visuals};

/// Semantic color tokens derived from egui's Visuals.
///
/// Use these named colors instead of hardcoding `Color32` values.
/// All colors are automatically derived from the base egui theme,
/// ensuring compatibility with any theme crate.
#[derive(Clone, Debug)]
pub struct ThemeColors {
    // === Surface colors (backgrounds) ===
    /// Primary window/panel background
    pub surface: Color32,
    /// Secondary/elevated surface (cards, dialogs)
    pub surface_elevated: Color32,
    /// Tertiary surface (nested containers)
    pub surface_nested: Color32,
    /// Overlay/modal background (semi-transparent)
    pub surface_overlay: Color32,

    // === Content colors (foreground) ===
    /// Primary text color
    pub text_primary: Color32,
    /// Secondary/muted text
    pub text_secondary: Color32,
    /// Disabled text
    pub text_disabled: Color32,

    // === Interactive element colors ===
    /// Primary accent color (buttons, links, selections)
    pub accent: Color32,
    /// Accent when hovered
    pub accent_hovered: Color32,
    /// Accent when pressed
    pub accent_pressed: Color32,

    // === Semantic/status colors ===
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub info: Color32,

    // === Shell-specific components ===
    /// Top bar background
    pub bar_top_bg: Color32,
    /// Top bar text color
    pub bar_top_text: Color32,
    /// Bottom bar background (with alpha for transparency)
    pub bar_bottom_bg: Color32,
    /// Bottom bar text color
    pub bar_bottom_text: Color32,
    /// Menu button background
    pub menu_button_bg: Color32,
    /// Menu button stroke/border
    pub menu_button_stroke: Color32,
    /// Menu button icon/text color
    pub menu_button_fg: Color32,
}

/// Semantic spacing tokens for consistent layouts.
#[derive(Clone, Debug)]
pub struct ThemeSpacing {
    /// Standard padding inside containers
    pub padding: f32,
    /// Small padding for compact elements
    pub padding_small: f32,
    /// Large padding for sections
    pub padding_large: f32,

    /// Standard gap between items
    pub gap: f32,
    /// Small gap for tight layouts
    pub gap_small: f32,
    /// Large gap for section separators
    pub gap_large: f32,

    /// Standard rounding for buttons/cards
    pub rounding: CornerRadius,
    /// Small rounding (inputs, chips)
    pub rounding_small: CornerRadius,
    /// Large rounding (modals, panels)
    pub rounding_large: CornerRadius,

    /// Standard bar height
    pub bar_height: f32,
    /// Standard icon button size
    pub icon_button_size: f32,
    /// Standard icon size within buttons
    pub icon_size: f32,
}

/// Complete theme combining egui Visuals with semantic tokens.
///
/// This is the main type to use for theming. It wraps egui's `Visuals`
/// and provides additional semantic tokens for application-specific styling.
#[derive(Clone, Debug)]
pub struct Theme {
    /// Base egui visuals - applied to the context
    pub visuals: Visuals,
    /// Semantic color tokens derived from visuals
    pub colors: ThemeColors,
    /// Semantic spacing tokens
    pub spacing: ThemeSpacing,
}

// Storage ID for theme in egui's data
const THEME_ID: &str = "__weaver_theme";

impl Theme {
    /// Create a theme from egui Visuals.
    /// Semantic tokens are automatically derived from the visuals.
    pub fn from_visuals(visuals: Visuals) -> Self {
        let colors = Self::derive_colors(&visuals);
        let spacing = Self::default_spacing();
        Self {
            visuals,
            colors,
            spacing,
        }
    }

    /// Derive semantic colors from egui Visuals.
    fn derive_colors(v: &Visuals) -> ThemeColors {
        let is_dark = v.dark_mode;

        ThemeColors {
            // Surfaces - derive from egui's built-in
            surface: v.panel_fill,
            surface_elevated: v.window_fill,
            surface_nested: v.faint_bg_color,
            surface_overlay: if is_dark {
                Color32::from_black_alpha(200)
            } else {
                Color32::from_white_alpha(230)
            },

            // Text - derive from widget visuals
            text_primary: v.widgets.noninteractive.fg_stroke.color,
            text_secondary: v
                .widgets
                .noninteractive
                .fg_stroke
                .color
                .gamma_multiply(0.7),
            text_disabled: v
                .widgets
                .noninteractive
                .fg_stroke
                .color
                .gamma_multiply(0.4),

            // Accent - derive from selection
            accent: v.selection.bg_fill,
            accent_hovered: v.widgets.hovered.bg_fill,
            accent_pressed: v.widgets.active.bg_fill,

            // Semantic colors (good defaults)
            success: Color32::from_rgb(76, 175, 80),
            warning: Color32::from_rgb(255, 193, 7),
            error: Color32::from_rgb(244, 67, 54),
            info: v.hyperlink_color,

            // Shell-specific defaults
            bar_top_bg: if is_dark {
                Color32::from_gray(30)
            } else {
                Color32::from_gray(245)
            },
            bar_top_text: v.widgets.noninteractive.fg_stroke.color,
            bar_bottom_bg: Color32::from_black_alpha(128),
            bar_bottom_text: Color32::WHITE,
            menu_button_bg: if is_dark {
                Color32::from_gray(50)
            } else {
                Color32::WHITE
            },
            menu_button_stroke: if is_dark {
                Color32::from_gray(80)
            } else {
                Color32::from_gray(200)
            },
            menu_button_fg: v.widgets.noninteractive.fg_stroke.color,
        }
    }

    /// Default spacing values.
    fn default_spacing() -> ThemeSpacing {
        ThemeSpacing {
            padding: 8.0,
            padding_small: 4.0,
            padding_large: 16.0,
            gap: 8.0,
            gap_small: 4.0,
            gap_large: 16.0,
            rounding: CornerRadius::same(8),
            rounding_small: CornerRadius::same(4),
            rounding_large: CornerRadius::same(16),
            bar_height: 40.0,
            icon_button_size: 40.0,
            icon_size: 24.0,
        }
    }

    /// Apply the visuals to egui context.
    /// Call this once at startup or when theme changes.
    pub fn apply(&self, ctx: &Context) {
        ctx.set_visuals(self.visuals.clone());
    }

    /// Store the theme in egui's data for global access.
    /// Call this after `apply()` at startup.
    pub fn store(&self, ctx: &Context) {
        ctx.data_mut(|d| d.insert_temp(Id::new(THEME_ID), self.clone()));
    }

    /// Apply and store the theme in one call.
    pub fn install(&self, ctx: &Context) {
        self.apply(ctx);
        self.store(ctx);
    }

    /// Retrieve the theme from egui's data.
    /// Returns the stored theme or a default dark theme if none was stored.
    pub fn get(ctx: &Context) -> Self {
        ctx.data(|d| d.get_temp::<Self>(Id::new(THEME_ID)))
            .unwrap_or_else(Self::dark)
    }

    // === Preset themes ===

    /// Standard egui dark theme with derived tokens.
    pub fn dark() -> Self {
        Self::from_visuals(Visuals::dark())
    }

    /// Standard egui light theme with derived tokens.
    pub fn light() -> Self {
        Self::from_visuals(Visuals::light())
    }

    /// Weaver Desktop branded dark theme.
    pub fn weaver_dark() -> Self {
        let mut visuals = Visuals::dark();

        // Custom background colors
        visuals.panel_fill = Color32::from_rgb(24, 24, 28);
        visuals.window_fill = Color32::from_rgb(32, 32, 38);
        visuals.faint_bg_color = Color32::from_rgb(40, 40, 48);
        visuals.extreme_bg_color = Color32::from_rgb(16, 16, 20);

        // Accent color (blue-ish)
        visuals.selection.bg_fill = Color32::from_rgb(80, 140, 200);
        visuals.hyperlink_color = Color32::from_rgb(100, 180, 255);

        // WidgetStr styling
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(45, 45, 55);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(55, 55, 65);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(70, 70, 85);
        visuals.widgets.active.bg_fill = Color32::from_rgb(90, 90, 110);

        let mut theme = Self::from_visuals(visuals);

        // Override shell-specific colors for Weaver look
        theme.colors.bar_top_bg = Color32::from_rgb(28, 28, 35);
        theme.colors.bar_top_text = Color32::from_gray(220);
        theme.colors.bar_bottom_bg = Color32::from_rgba_unmultiplied(20, 20, 25, 180);
        theme.colors.bar_bottom_text = Color32::from_gray(200);
        theme.colors.menu_button_bg = Color32::from_rgb(45, 45, 55);
        theme.colors.menu_button_stroke = Color32::from_rgb(70, 70, 85);
        theme.colors.menu_button_fg = Color32::from_gray(220);

        // Custom spacing for Weaver
        theme.spacing.rounding_large = CornerRadius::same(24);

        theme
    }

    /// Weaver Desktop branded light theme.
    pub fn weaver_light() -> Self {
        let mut visuals = Visuals::light();

        // Custom background colors
        visuals.panel_fill = Color32::from_rgb(250, 250, 252);
        visuals.window_fill = Color32::from_rgb(255, 255, 255);
        visuals.faint_bg_color = Color32::from_rgb(245, 245, 248);
        visuals.extreme_bg_color = Color32::from_rgb(235, 235, 240);

        // Accent color
        visuals.selection.bg_fill = Color32::from_rgb(60, 120, 200);
        visuals.hyperlink_color = Color32::from_rgb(40, 100, 180);

        let mut theme = Self::from_visuals(visuals);

        // Override shell-specific colors
        theme.colors.bar_top_bg = Color32::from_rgb(255, 255, 255);
        theme.colors.bar_top_text = Color32::from_gray(40);
        theme.colors.bar_bottom_bg = Color32::from_rgba_unmultiplied(255, 255, 255, 230);
        theme.colors.bar_bottom_text = Color32::from_gray(60);
        theme.colors.menu_button_bg = Color32::WHITE;
        theme.colors.menu_button_stroke = Color32::from_gray(200);
        theme.colors.menu_button_fg = Color32::from_gray(40);

        theme.spacing.rounding_large = CornerRadius::same(24);

        theme
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
