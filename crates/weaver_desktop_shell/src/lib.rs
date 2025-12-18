pub mod commands;
pub mod components;
pub mod views;

// Legacy shell (preserved for reference)
pub use components::Shell;

// New widget-based shell and types
pub use components::{
    DesktopShell, DesktopIcon, DesktopImageWidget, IconGridWidget,
    Widget, WidgetContent, Size, Align, Justify, Spacing, Axis, Label, Spacer,
    ImageSource, ImageSurface, ScaleMode,
};
