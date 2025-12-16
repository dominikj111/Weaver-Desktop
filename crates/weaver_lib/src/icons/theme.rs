//! Icon theme implementation following freedesktop.org Icon Theme Specification.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Icon context categories as defined by freedesktop.org spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconContext {
    Actions,
    Animations,
    Apps,
    Categories,
    Devices,
    Emblems,
    Emotes,
    MimeTypes,
    Places,
    Status,
    Stock,
    /// Symbolic icons (usually monochrome, scalable)
    Symbolic,
}

impl IconContext {
    /// Returns the directory name for this context.
    pub fn as_dir_name(&self) -> &'static str {
        match self {
            IconContext::Actions => "actions",
            IconContext::Animations => "animations",
            IconContext::Apps => "apps",
            IconContext::Categories => "categories",
            IconContext::Devices => "devices",
            IconContext::Emblems => "emblems",
            IconContext::Emotes => "emotes",
            IconContext::MimeTypes => "mimetypes",
            IconContext::Places => "places",
            IconContext::Status => "status",
            IconContext::Stock => "stock",
            IconContext::Symbolic => "symbolic",
        }
    }
}

/// Supported icon sizes in Papirus and most icon themes.
const STANDARD_SIZES: &[u32] = &[16, 22, 24, 32, 48, 64];

/// Icon theme loader following freedesktop.org specification.
///
/// Searches for icons in multiple locations with priority:
/// 1. Custom user path (if set)
/// 2. Dev mode embedded path
/// 3. Standard XDG paths on Linux
pub struct IconTheme {
    /// Theme name (e.g., "Papirus", "Papirus-Dark")
    name: String,
    /// Search paths in priority order (first = highest priority)
    search_paths: Vec<PathBuf>,
    /// Cache of resolved icon paths: (name, size, context) -> path
    cache: HashMap<(String, u32, IconContext), PathBuf>,
}

impl IconTheme {
    /// Create a new icon theme with the given name.
    ///
    /// The theme name corresponds to the directory name in icon theme locations
    /// (e.g., "Papirus", "Papirus-Dark", "Papirus-Light").
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            search_paths: Vec::new(),
            cache: HashMap::new(),
        }
    }

    /// Add a search path for icon themes.
    ///
    /// Paths are searched in the order they are added (first added = highest priority).
    /// The path should point to the theme directory itself (e.g., `/usr/share/icons/Papirus`).
    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    /// Add standard XDG icon paths for Linux.
    ///
    /// Adds paths in priority order:
    /// 1. `~/.local/share/icons/{theme}`
    /// 2. `~/.icons/{theme}`
    /// 3. `/usr/local/share/icons/{theme}`
    /// 4. `/usr/share/icons/{theme}`
    pub fn add_xdg_paths(&mut self) {
        // User-local paths (highest priority)
        if let Some(home) = std::env::var_os("HOME") {
            let home = PathBuf::from(home);
            
            // XDG_DATA_HOME or default
            let data_home = std::env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| home.join(".local/share"));
            
            self.search_paths.push(data_home.join("icons").join(&self.name));
            self.search_paths.push(home.join(".icons").join(&self.name));
        }

        // System paths
        self.search_paths.push(PathBuf::from("/usr/local/share/icons").join(&self.name));
        self.search_paths.push(PathBuf::from("/usr/share/icons").join(&self.name));
    }

    /// Get the theme name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the current search paths.
    pub fn search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }

    /// Clear the icon path cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Look up an icon by name, size, and context.
    ///
    /// Returns the path to the icon file if found, or None if not found.
    /// Results are cached for performance.
    ///
    /// # Arguments
    /// - `name`: Icon name without extension (e.g., "folder", "document-open")
    /// - `size`: Desired icon size in pixels (will find closest match)
    /// - `context`: Icon context category
    pub fn lookup(&mut self, name: &str, size: u32, context: IconContext) -> Option<PathBuf> {
        let cache_key = (name.to_string(), size, context);
        
        // Check cache first
        if let Some(path) = self.cache.get(&cache_key) {
            return Some(path.clone());
        }

        // Find the best matching size
        let best_size = find_closest_size(size);
        
        // Search in all paths
        for base_path in &self.search_paths {
            if let Some(path) = self.find_icon_in_theme(base_path, name, best_size, context) {
                self.cache.insert(cache_key, path.clone());
                return Some(path);
            }
        }

        // Try fallback sizes if exact size not found
        for &fallback_size in STANDARD_SIZES {
            if fallback_size == best_size {
                continue;
            }
            for base_path in &self.search_paths {
                if let Some(path) = self.find_icon_in_theme(base_path, name, fallback_size, context) {
                    self.cache.insert(cache_key, path.clone());
                    return Some(path);
                }
            }
        }

        None
    }

    /// Look up an icon, trying multiple contexts in order.
    ///
    /// Useful when you're not sure which context an icon belongs to.
    pub fn lookup_any_context(&mut self, name: &str, size: u32, contexts: &[IconContext]) -> Option<PathBuf> {
        for &context in contexts {
            if let Some(path) = self.lookup(name, size, context) {
                return Some(path);
            }
        }
        None
    }

    /// Find an icon file in a specific theme directory.
    fn find_icon_in_theme(
        &self,
        theme_path: &Path,
        name: &str,
        size: u32,
        context: IconContext,
    ) -> Option<PathBuf> {
        let size_dir = format!("{}x{}", size, size);
        let context_dir = context.as_dir_name();

        // Try SVG first (scalable, preferred)
        let svg_path = theme_path
            .join(&size_dir)
            .join(context_dir)
            .join(format!("{}.svg", name));
        if svg_path.exists() {
            return Some(svg_path);
        }

        // Try PNG
        let png_path = theme_path
            .join(&size_dir)
            .join(context_dir)
            .join(format!("{}.png", name));
        if png_path.exists() {
            return Some(png_path);
        }

        // Try symbolic directory for symbolic icons
        if context == IconContext::Symbolic {
            let symbolic_path = theme_path
                .join("symbolic")
                .join(context_dir)
                .join(format!("{}-symbolic.svg", name));
            if symbolic_path.exists() {
                return Some(symbolic_path);
            }
        }

        None
    }

    /// List all available icons in a given context and size.
    ///
    /// Returns icon names (without extension).
    pub fn list_icons(&self, size: u32, context: IconContext) -> Vec<String> {
        let mut icons = Vec::new();
        let size_dir = format!("{}x{}", size, size);
        let context_dir = context.as_dir_name();

        for base_path in &self.search_paths {
            let dir_path = base_path.join(&size_dir).join(context_dir);
            if let Ok(entries) = std::fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.path().file_stem() {
                        let name = name.to_string_lossy().to_string();
                        if !icons.contains(&name) {
                            icons.push(name);
                        }
                    }
                }
            }
        }

        icons.sort();
        icons
    }
}

/// Find the closest standard icon size.
fn find_closest_size(requested: u32) -> u32 {
    STANDARD_SIZES
        .iter()
        .copied()
        .min_by_key(|&s| (s as i32 - requested as i32).abs() as u32)
        .unwrap_or(48)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_size() {
        assert_eq!(find_closest_size(16), 16);
        assert_eq!(find_closest_size(20), 22);
        assert_eq!(find_closest_size(30), 32);
        assert_eq!(find_closest_size(50), 48);
        assert_eq!(find_closest_size(100), 64);
    }

    #[test]
    fn test_icon_context_dir_names() {
        assert_eq!(IconContext::Actions.as_dir_name(), "actions");
        assert_eq!(IconContext::Places.as_dir_name(), "places");
        assert_eq!(IconContext::Apps.as_dir_name(), "apps");
    }
}
