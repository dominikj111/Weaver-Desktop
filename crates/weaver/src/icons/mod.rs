//! Freedesktop.org Icon Theme support for SystemWeaver.
//!
//! Implements the [Icon Theme Specification](https://specifications.freedesktop.org/icon-theme-spec/latest/)
//! to load icons from standard Linux paths or embedded/custom directories.
//!
//! # Usage
//!
//! ```rust,ignore
//! let mut theme = IconTheme::new("Papirus");
//! 
//! // Add dev mode path (embedded icons)
//! theme.add_search_path("./assets/icons/papirus-icon-theme/Papirus");
//! 
//! // Resolve an icon by name
//! if let Some(path) = theme.lookup("folder", 48, IconContext::Places) {
//!     // Load the icon from path
//! }
//! ```

mod theme;

pub use theme::{IconContext, IconTheme};
