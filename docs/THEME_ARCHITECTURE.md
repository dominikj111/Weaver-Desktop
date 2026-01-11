# Theme Architecture Proposal

**Status**: Proposed  
**Created**: December 31, 2025  
**Related**: [PROPOSAL.md](PROPOSAL.md), [UI_FABRIC_PROPOSAL.md](UI_FABRIC_PROPOSAL.md)

---

## Executive Summary

Weaver Desktop's vision includes mimicking any OS visual style—Windows XP, Windows 7, GNOME Shell, macOS, and custom interfaces like kiosk systems or order terminals. This proposal defines a three-layer architecture that separates **functional components** (what they do), **visual renderers** (how they look), and **theme definitions** (layout configuration), enabling unlimited visual flexibility while maintaining a unified codebase.

**Key insight**: A calendar widget, app launcher, or workspace switcher performs the same function regardless of whether it looks like Windows XP or GNOME Shell. We separate the logic from the presentation.

---

## Vision: One System, Infinite Appearances

The same Weaver Desktop binary should transform into:

| Theme              | Visual Appearance                                                                   |
| ------------------ | ----------------------------------------------------------------------------------- |
| **Windows XP**     | Blue taskbar gradient, green Start button, system tray clock, no workspace switcher |
| **Windows 7**      | Glass taskbar, rounded Start orb, peek preview, Aero styling                        |
| **GNOME Shell**    | Top panel, Activities corner, workspace switcher, app grid overlay                  |
| **macOS**          | Dock at bottom, menu bar at top, app icons with bounce animation                    |
| **Zorin OS**       | Hybrid Windows/GNOME aesthetic, configurable layout                                 |
| **Kiosk System**   | Single-purpose locked interface, minimal chrome, large touch targets                |
| **Order Terminal** | Restaurant POS-style grid, department categories, numpad focus                      |

All these share common **functional components**: app launcher, clock, workspace manager, system status—only the **visual presentation** differs.

---

## Current Problem

The existing architecture has theme-specific widgets as concrete structs:

```rust
pub struct XpStartButton { /* Windows XP specific */ }
pub struct XpTaskbar { /* Windows XP specific */ }
pub struct XpClock { /* Windows XP specific */ }
```

**Issues with this approach:**

1. ❌ **Theme lock-in**: Adding Windows 7 requires `Win7StartButton`, `Win7Taskbar`, etc.
2. ❌ **Logic duplication**: App launcher behavior is duplicated across theme implementations
3. ❌ **Binary bloat**: Every theme compiled into binary, even if unused
4. ❌ **No extensibility**: Users can't create custom themes without forking
5. ❌ **Testing complexity**: Can't test app launcher logic independently of XP rendering

---

## Proposed Architecture: Three-Layer Separation

```
┌──────────────────────────────────────────────────────────────┐
│  LAYER 1: Functional Components (theme-agnostic)             │
│  ┌────────────┐  ┌───────┐  ┌─────────────────┐            │
│  │AppLauncher │  │ Clock │  │WorkspaceSwitcher│  ...        │
│  └────────────┘  └───────┘  └─────────────────┘            │
│  • Business logic only                                       │
│  • No rendering code                                         │
│  • Theme-independent state management                        │
└──────────────────────────────────────────────────────────────┘
                            ↓ uses
┌──────────────────────────────────────────────────────────────┐
│  LAYER 2: Theme Renderers (visual implementations)           │
│  ┌──────────────────┐  ┌──────────────────┐                │
│  │WindowsXPRenderer │  │GnomeShellRenderer│  ...            │
│  └──────────────────┘  └──────────────────┘                │
│  • Trait-based implementations                               │
│  • Component-specific rendering logic                        │
│  • Asset management (images, gradients, fonts)               │
└──────────────────────────────────────────────────────────────┘
                            ↓ defined by
┌──────────────────────────────────────────────────────────────┐
│  LAYER 3: Theme Definitions (configuration)                  │
│  ┌──────────────┐  ┌───────────────┐                        │
│  │windowsxp.toml│  │gnomeshell.toml│  ...                   │
│  └──────────────┘  └───────────────┘                        │
│  • Component layout and placement                            │
│  • Which components are visible                              │
│  • Renderer selection per component                          │
│  • Theme metadata and assets                                 │
└──────────────────────────────────────────────────────────────┘
```

---

## Layer 1: Functional Components

Components contain **business logic only**, no rendering code.

### Example: AppLauncher Component

```rust
/// Functional app launcher - manages menu state and app list.
/// Theme-agnostic: works the same in all visual styles.
pub struct AppLauncher {
    /// Current menu state
    state: MenuState,
    /// Available applications
    apps: Vec<AppEntry>,
    /// Search filter (if menu supports search)
    filter: String,
}

impl AppLauncher {
    pub fn toggle(&mut self) {
        self.state = match self.state {
            MenuState::Closed => MenuState::Open,
            MenuState::Open => MenuState::Closed,
        };
    }
    
    pub fn is_open(&self) -> bool {
        matches!(self.state, MenuState::Open)
    }
    
    pub fn get_apps(&self) -> &[AppEntry] {
        if self.filter.is_empty() {
            &self.apps
        } else {
            // Filtered list
        }
    }
    
    pub fn launch_app(&mut self, app_id: &str) {
        // Delegate to workmeshd or launch directly
        self.state = MenuState::Closed;
    }
}
```

### Other Functional Components

- **Clock**: Time source, format settings, calendar popup state
- **WorkspaceSwitcher**: Workspace list, current workspace, switching logic
- **SystemTray**: Tray apps, notification count, status indicators
- **WindowList**: Open windows, switching, minimize/maximize logic
- **QuickLaunch**: Pinned apps, launch logic
- **SearchBar**: Query state, search results, filtering

---

## Layer 2: Theme Renderers

Renderers implement **visual presentation** via traits.

### Renderer Trait Pattern

```rust
/// Trait for rendering app launcher component.
pub trait AppLauncherRenderer: Send + Sync {
    /// Render the launcher button/trigger.
    /// Returns response for interaction handling.
    fn render_button(
        &mut self,
        ui: &mut Ui,
        launcher: &AppLauncher,
        theme: &Theme,
    ) -> Response;
    
    /// Render the app menu (when open).
    fn render_menu(
        &mut self,
        ui: &mut Ui,
        launcher: &mut AppLauncher,
        theme: &Theme,
    );
    
    /// Preferred size for the button.
    fn button_size(&self) -> Vec2;
}
```

### Example: Windows XP Renderer

```rust
pub struct WindowsXPAppLauncherRenderer {
    /// Cached Start button texture
    start_button_texture: Option<TextureHandle>,
    /// Button dimensions (scaled to taskbar height)
    button_size: Vec2,
    /// Load attempt tracker
    texture_loaded: bool,
}

impl AppLauncherRenderer for WindowsXPAppLauncherRenderer {
    fn render_button(
        &mut self,
        ui: &mut Ui,
        launcher: &AppLauncher,
        theme: &Theme,
    ) -> Response {
        // Load texture on first render
        if !self.texture_loaded {
            self.load_start_button_texture(ui.ctx());
        }
        
        let (rect, response) = ui.allocate_exact_size(
            self.button_size,
            Sense::click(),
        );
        
        if ui.is_rect_visible(rect) {
            if let Some(ref texture) = self.start_button_texture {
                // Draw Windows XP Start button image
                let tint = if response.hovered() {
                    Color32::from_rgb(255, 255, 240) // Slight brightness
                } else if response.is_pointer_button_down_on() {
                    Color32::from_rgb(220, 220, 220) // Slight darken
                } else {
                    Color32::WHITE
                };
                
                ui.painter().image(texture.id(), rect, Rect::from_min_max(
                    pos2(0.0, 0.0),
                    pos2(1.0, 1.0),
                ), tint);
            } else {
                // Fallback: green button with "start" text
                self.render_fallback_button(ui, rect, &response);
            }
        }
        
        response
    }
    
    fn render_menu(
        &mut self,
        ui: &mut Ui,
        launcher: &mut AppLauncher,
        theme: &Theme,
    ) {
        // Classic Windows XP Start Menu
        // Two-column layout: left (pinned/common), right (all programs)
        let menu_rect = self.calculate_menu_position(ui);
        
        egui::Window::new("start_menu")
            .fixed_pos(menu_rect.min)
            .fixed_size(menu_rect.size())
            .title_bar(false)
            .frame(self.xp_menu_frame())
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    // Left column: user info + pinned apps
                    ui.vertical(|ui| {
                        self.render_user_panel(ui, theme);
                        self.render_pinned_apps(ui, launcher);
                    });
                    
                    // Right column: all programs
                    ui.vertical(|ui| {
                        self.render_app_list(ui, launcher);
                    });
                });
                
                // Bottom: log off, shutdown buttons
                self.render_system_buttons(ui, launcher);
            });
    }
    
    fn button_size(&self) -> Vec2 {
        Vec2::new(97.0, 30.0) // XP Start button dimensions
    }
}
```

### Example: GNOME Shell Renderer

```rust
pub struct GnomeShellAppLauncherRenderer {
    app_grid_scroll: f32,
}

impl AppLauncherRenderer for GnomeShellAppLauncherRenderer {
    fn render_button(
        &mut self,
        ui: &mut Ui,
        launcher: &AppLauncher,
        theme: &Theme,
    ) -> Response {
        // GNOME has "Activities" text in top-left corner
        let text = "Activities";
        let galley = ui.painter().layout_no_wrap(
            text.to_string(),
            FontId::proportional(14.0),
            theme.colors.text_primary,
        );
        
        let size = galley.size() + vec2(16.0, 8.0); // Padding
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        
        if ui.is_rect_visible(rect) {
            let bg_color = if launcher.is_open() {
                theme.colors.accent // Highlighted when active
            } else if response.hovered() {
                theme.colors.accent_hovered.linear_multiply(0.3)
            } else {
                Color32::TRANSPARENT
            };
            
            if bg_color != Color32::TRANSPARENT {
                ui.painter().rect_filled(rect, 3.0, bg_color);
            }
            
            ui.painter().galley(
                rect.center() - galley.size() / 2.0,
                galley,
            );
        }
        
        response
    }
    
    fn render_menu(
        &mut self,
        ui: &mut Ui,
        launcher: &mut AppLauncher,
        theme: &Theme,
    ) {
        // GNOME shows fullscreen app grid overlay
        let screen_rect = ui.ctx().screen_rect();
        
        egui::Area::new("gnome_app_grid")
            .fixed_pos(screen_rect.min)
            .show(ui.ctx(), |ui| {
                // Semi-transparent dark overlay
                ui.painter().rect_filled(
                    screen_rect,
                    0.0,
                    Color32::from_black_alpha(200),
                );
                
                // Centered app grid
                ui.centered_and_justified(|ui| {
                    egui::ScrollArea::vertical()
                        .id_source("app_grid_scroll")
                        .show(ui, |ui| {
                            self.render_app_grid(ui, launcher, theme);
                        });
                });
                
                // Search bar at top
                self.render_search_bar(ui, launcher, theme);
            });
    }
    
    fn button_size(&self) -> Vec2 {
        Vec2::new(80.0, 28.0) // "Activities" button
    }
}
```

---

## Layer 3: Theme Definitions

Theme configuration files define **layout and component selection**.

### Theme Definition Format (TOML)

```toml
# themes/windowsxp/theme.toml

[meta]
id = "windowsxp"
name = "Windows XP"
author = "Weaver Team"
version = "1.0.0"
description = "Classic Windows XP Luna theme with blue taskbar"

[layout]
# Where is the primary bar?
primary_bar = "bottom"  # bottom | top | left | right

# Bar dimensions
bar_height = 30  # pixels (for bottom/top)
bar_width = 48   # pixels (for left/right)

# Desktop background
wallpaper = "bliss.jpg"
background_color = "#5A7FBF"

# Component placement zones
[layout.zones.bottom_bar_left]
components = ["app_launcher"]

[layout.zones.bottom_bar_center]
components = ["window_list"]

[layout.zones.bottom_bar_right]
components = ["system_tray", "clock"]

[layout.zones.desktop]
components = ["icon_grid"]

# Component configurations
[components.app_launcher]
renderer = "windowsxp_start_button"
enabled = true

[components.clock]
renderer = "windowsxp_tray_clock"
format = "%I:%M %p"
show_date_on_hover = true

[components.window_list]
renderer = "windowsxp_taskbar_buttons"
group_by_app = false
show_labels = true

[components.workspace_switcher]
# Windows XP didn't have virtual desktops visually
enabled = false

[components.system_tray]
renderer = "windowsxp_system_tray"
show_hidden_icons = true

# Renderer-specific settings
[renderers.windowsxp_start_button]
image_path = "assets/themes/windowsxp/start_button.png"
hover_brightness = 1.05
pressed_brightness = 0.95

[renderers.windowsxp_tray_clock]
# XP tray has distinct gradient
gradient_top = "#316AC5"
gradient_bottom = "#1F4AAE"
separator_color = "#0C3475"
font_size = 13

[renderers.windowsxp_taskbar_buttons]
button_min_width = 160
button_max_width = 200
active_color = "#3169C6"
inactive_color = "#245EDC"
```

### GNOME Shell Theme Example

```toml
# themes/gnomeshell/theme.toml

[meta]
id = "gnomeshell"
name = "GNOME Shell"
author = "Weaver Team"
version = "1.0.0"

[layout]
primary_bar = "top"
bar_height = 28
wallpaper = "adwaita-dark.svg"

[layout.zones.top_bar_left]
components = ["app_launcher", "window_title"]

[layout.zones.top_bar_center]
components = ["clock"]

[layout.zones.top_bar_right]
components = ["system_tray", "power_menu"]

[layout.zones.desktop]
components = [] # GNOME doesn't show desktop icons by default

[layout.zones.left_dock]
components = ["app_dock", "workspace_switcher"]

[components.app_launcher]
renderer = "gnome_activities_button"

[components.clock]
renderer = "gnome_panel_clock"
format = "%a %b %d  %H:%M"

[components.workspace_switcher]
renderer = "gnome_workspace_strip"
orientation = "vertical"
position = "left"

[components.app_dock]
renderer = "gnome_dash"
position = "left"
icon_size = 48
autohide = false
```

---

## Implementation Plan

### Phase 1: Core Refactoring (MVP - Weeks 1-2)

**Goal**: Establish renderer pattern with 2 working themes.

1. **Extract functional components** from existing widgets
   - `AppLauncher`, `Clock`, `WorkspaceSwitcher`, `WindowList`, `SystemTray`
   - Pure logic, no rendering code

2. **Define renderer traits** for each component type
   - `AppLauncherRenderer`, `ClockRenderer`, etc.
   - Standard interface: `render()`, `size()`, `configure()`

3. **Implement 2 concrete renderers**
   - **WindowsXPTheme**: Start button, taskbar, system tray clock
   - **DefaultTheme**: Simple fallback (current generic style)

4. **Update DesktopShell** to use component + renderer pattern
   - Replace direct widget usage with component + renderer pairs
   - Add `set_theme()` method to swap renderer implementations

5. **Manual theme switching** for testing
   - Keyboard shortcut or debug menu to switch themes
   - Verify components work across both themes

**Success criteria**: Can switch between Windows XP and Default theme at runtime, all components functional.

### Phase 2: Theme Configuration (Weeks 3-4)

**Goal**: Move theme definitions to configuration files.

1. **Design theme definition format** (TOML)
   - Metadata, layout zones, component placement
   - Renderer selection and configuration

2. **Theme loader** infrastructure
   - Parse TOML files from `~/.config/weaver/themes/` and system paths
   - Validate theme structure
   - Error handling for malformed themes

3. **Theme registry** system
   - Discover available themes (built-in + user)
   - Theme metadata display (name, author, description)
   - Default theme selection

4. **Built-in theme definitions**
   - Convert WindowsXP and Default to TOML format
   - Bundle with application

5. **Settings UI** for theme selection
   - Theme picker with previews (screenshots)
   - Apply theme without restart
   - Reset to default

**Success criteria**: Themes defined in TOML files, user can switch themes via Settings, changes persist.

### Phase 3: Extended Themes (Weeks 5-6)

**Goal**: Add more theme variety to prove architecture flexibility.

1. **Implement 3 additional themes**
   - **Windows 7**: Aero glass, orb Start button
   - **GNOME Shell**: Activities corner, app grid overlay
   - **macOS**: Top menu bar, bottom dock

2. **Component coverage**
   - Ensure all components have renderers for each theme
   - Graceful fallback when renderer missing

3. **Asset management**
   - Theme-specific images, fonts, icons
   - Asset bundling and loading
   - Cache management

4. **Theme testing suite**
   - Automated visual regression tests
   - Component interaction tests per theme
   - Performance benchmarks

**Success criteria**: 5 distinct themes working, visual differences clear, no regressions.

### Phase 4: Plugin System (Weeks 7-8)

**Goal**: Enable community-created themes.

1. **Plugin trait** for dynamic theme loading

   ```rust
   pub trait ThemePlugin: Send + Sync {
       fn meta(&self) -> ThemeMetadata;
       fn create_renderers(&self) -> ThemeRenderers;
       fn load_assets(&mut self, ctx: &Context);
   }
   ```

2. **Dynamic library loading** (optional, advanced)
   - Load .so/.dll/.dylib theme plugins
   - Sandboxing and safety considerations
   - Plugin validation and signing

3. **Alternative: WASM themes** (safer, portable)
   - Themes compiled to WASM
   - Run in sandboxed environment
   - Cross-platform compatibility

4. **Theme marketplace** concept
   - Directory structure for user themes
   - Theme installation/removal
   - Update mechanism

5. **Theme developer documentation**
   - Tutorial: "Creating Your First Theme"
   - Renderer trait reference
   - Asset guidelines
   - Example theme repository

**Success criteria**: Community member can create and share a custom theme without core codebase changes.

---

## Renderer Trait Specifications

### Core Traits

```rust
/// Base renderer trait - all component renderers extend this.
pub trait Renderer: Send + Sync {
    /// Unique identifier for this renderer.
    fn id(&self) -> &str;
    
    /// Initialize renderer (load assets, prepare resources).
    fn initialize(&mut self, ctx: &Context);
    
    /// Cleanup renderer (free resources).
    fn cleanup(&mut self);
}

/// Renders app launcher component.
pub trait AppLauncherRenderer: Renderer {
    fn render_button(&mut self, ui: &mut Ui, launcher: &AppLauncher) -> Response;
    fn render_menu(&mut self, ui: &mut Ui, launcher: &mut AppLauncher);
    fn button_size(&self) -> Vec2;
}

/// Renders clock component.
pub trait ClockRenderer: Renderer {
    fn render(&mut self, ui: &mut Ui, clock: &Clock) -> Response;
    fn preferred_size(&self) -> Vec2;
}

/// Renders workspace switcher component.
pub trait WorkspaceSwitcherRenderer: Renderer {
    fn render(&mut self, ui: &mut Ui, switcher: &WorkspaceSwitcher) -> Response;
    fn orientation(&self) -> Orientation; // Horizontal or Vertical
}

/// Renders window list (taskbar buttons).
pub trait WindowListRenderer: Renderer {
    fn render(&mut self, ui: &mut Ui, windows: &WindowList);
    fn button_min_width(&self) -> f32;
    fn button_max_width(&self) -> f32;
}

/// Renders system tray.
pub trait SystemTrayRenderer: Renderer {
    fn render(&mut self, ui: &mut Ui, tray: &SystemTray);
    fn icon_size(&self) -> f32;
}
```

### Renderer Registry

```rust
/// Central registry for theme renderers.
pub struct ThemeRenderers {
    app_launcher: Box<dyn AppLauncherRenderer>,
    clock: Box<dyn ClockRenderer>,
    workspace_switcher: Option<Box<dyn WorkspaceSwitcherRenderer>>,
    window_list: Box<dyn WindowListRenderer>,
    system_tray: Box<dyn SystemTrayRenderer>,
}

impl ThemeRenderers {
    /// Create renderers for a specific theme.
    pub fn for_theme(theme_id: &str) -> Self {
        match theme_id {
            "windowsxp" => Self {
                app_launcher: Box::new(WindowsXPStartButton::new()),
                clock: Box::new(WindowsXPClock::new()),
                workspace_switcher: None, // XP doesn't have this
                window_list: Box::new(WindowsXPTaskbarButtons::new()),
                system_tray: Box::new(WindowsXPSystemTray::new()),
            },
            "gnomeshell" => Self {
                app_launcher: Box::new(GnomeActivitiesButton::new()),
                clock: Box::new(GnomePanelClock::new()),
                workspace_switcher: Some(Box::new(GnomeWorkspaceStrip::new())),
                window_list: Box::new(GnomeWindowList::new()),
                system_tray: Box::new(GnomeSystemIndicators::new()),
            },
            _ => Self::default_theme(),
        }
    }
}
```

---

## Asset Management System

Themes require various image assets with different formats and behaviors. A dedicated asset management system handles loading, caching, and rendering these resources efficiently.

### Asset Types & Use Cases

| Asset Type        | Format        | Use Case                                | Example                               |
| ----------------- | ------------- | --------------------------------------- | ------------------------------------- |
| **Icons**         | ICO, PNG, SVG | Application icons, file type icons      | `notepad.ico`, `folder.png`           |
| **Sprite Sheets** | PNG           | Multi-state UI elements (button states) | `start_button.png` (3 states stacked) |
| **Backgrounds**   | PNG, JPG      | Desktop wallpaper, widget backgrounds   | `bliss.jpg`, `panel_bg.png`           |
| **Gradients**     | Code or PNG   | Taskbar backgrounds, button fills       | XP taskbar gradient                   |
| **Cursors**       | CUR, PNG      | Custom mouse cursors                    | `arrow.cur`, `pointer.png`            |

### ThemeAsset Abstraction

```rust
/// Represents a loadable theme asset.
#[derive(Debug, Clone)]
pub enum ThemeAsset {
    /// Single image file
    Image {
        path: PathBuf,
        format: ImageFormat,
    },
    /// Sprite sheet with multiple frames/states
    SpriteSheet {
        path: PathBuf,
        layout: SpriteLayout,
    },
    /// Procedural gradient
    Gradient {
        direction: GradientDirection,
        stops: Vec<ColorStop>,
    },
    /// Solid color
    Color(Color32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Ico,
    Svg,
    Bmp,
}

/// Describes how sprites are arranged in a sheet.
#[derive(Debug, Clone)]
pub enum SpriteLayout {
    /// Vertical stack (states stacked top to bottom)
    /// Each sprite has equal height = total_height / count
    VerticalStack { count: usize },
    
    /// Horizontal strip (states left to right)
    /// Each sprite has equal width = total_width / count
    HorizontalStrip { count: usize },
    
    /// Grid layout
    Grid { rows: usize, cols: usize },
    
    /// Custom rectangles (explicit coordinates for each sprite)
    Custom { rects: Vec<SpriteRect> },
}

#[derive(Debug, Clone)]
pub struct SpriteRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum GradientDirection {
    Vertical,
    Horizontal,
    Diagonal,
    Radial,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorStop {
    pub position: f32, // 0.0 to 1.0
    pub color: Color32,
}
```

### Asset Loader

```rust
/// Manages loading and caching of theme assets.
pub struct AssetLoader {
    /// Texture cache (prevents reloading same image)
    texture_cache: HashMap<PathBuf, TextureHandle>,
    
    /// Sprite sheet cache (textures + sprite info)
    sprite_cache: HashMap<PathBuf, SpriteSheet>,
    
    /// Base path for theme assets
    theme_path: PathBuf,
}

impl AssetLoader {
    pub fn new(theme_path: impl Into<PathBuf>) -> Self {
        Self {
            texture_cache: HashMap::new(),
            sprite_cache: HashMap::new(),
            theme_path: theme_path.into(),
        }
    }
    
    /// Load a single image asset.
    pub fn load_image(
        &mut self,
        ctx: &Context,
        path: impl AsRef<Path>,
    ) -> Result<TextureHandle> {
        let path = self.resolve_path(path.as_ref());
        
        // Check cache first
        if let Some(texture) = self.texture_cache.get(&path) {
            return Ok(texture.clone());
        }
        
        // Load from disk
        let image = image::open(&path)?;
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture = ctx.load_texture(
            path.to_string_lossy(),
            color_image,
            egui::TextureOptions::LINEAR,
        );
        
        // Cache it
        self.texture_cache.insert(path.clone(), texture.clone());
        
        Ok(texture)
    }
    
    /// Load a sprite sheet asset.
    pub fn load_sprite_sheet(
        &mut self,
        ctx: &Context,
        path: impl AsRef<Path>,
        layout: SpriteLayout,
    ) -> Result<SpriteSheet> {
        let path = self.resolve_path(path.as_ref());
        
        // Check cache
        if let Some(sprite_sheet) = self.sprite_cache.get(&path) {
            return Ok(sprite_sheet.clone());
        }
        
        // Load image
        let image = image::open(&path)?;
        let rgba = image.to_rgba8();
        let full_width = rgba.width();
        let full_height = rgba.height();
        
        // Calculate sprite regions based on layout
        let sprites = match layout {
            SpriteLayout::VerticalStack { count } => {
                let sprite_height = full_height / count as u32;
                (0..count)
                    .map(|i| SpriteRect {
                        x: 0,
                        y: i as u32 * sprite_height,
                        width: full_width,
                        height: sprite_height,
                    })
                    .collect()
            }
            SpriteLayout::HorizontalStrip { count } => {
                let sprite_width = full_width / count as u32;
                (0..count)
                    .map(|i| SpriteRect {
                        x: i as u32 * sprite_width,
                        y: 0,
                        width: sprite_width,
                        height: full_height,
                    })
                    .collect()
            }
            SpriteLayout::Grid { rows, cols } => {
                let sprite_width = full_width / cols as u32;
                let sprite_height = full_height / rows as u32;
                let mut sprites = Vec::new();
                for row in 0..rows {
                    for col in 0..cols {
                        sprites.push(SpriteRect {
                            x: col as u32 * sprite_width,
                            y: row as u32 * sprite_height,
                            width: sprite_width,
                            height: sprite_height,
                        });
                    }
                }
                sprites
            }
            SpriteLayout::Custom { rects } => rects,
        };
        
        // Load full texture
        let size = [full_width as usize, full_height as usize];
        let pixels = rgba.into_raw();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture = ctx.load_texture(
            path.to_string_lossy(),
            color_image,
            egui::TextureOptions::LINEAR,
        );
        
        let sprite_sheet = SpriteSheet {
            texture,
            sprites,
            full_size: Vec2::new(full_width as f32, full_height as f32),
        };
        
        // Cache it
        self.sprite_cache.insert(path.clone(), sprite_sheet.clone());
        
        Ok(sprite_sheet)
    }
    
    /// Load Windows ICO file (supports multiple sizes).
    pub fn load_ico(
        &mut self,
        ctx: &Context,
        path: impl AsRef<Path>,
        preferred_size: Option<u32>,
    ) -> Result<TextureHandle> {
        let path = self.resolve_path(path.as_ref());
        
        // ICO files can contain multiple images at different sizes
        // We select the best match for the requested size
        let ico_image = ico::IconDir::read(std::fs::File::open(&path)?)?;
        
        let entry = if let Some(size) = preferred_size {
            // Find closest match to preferred size
            ico_image
                .entries()
                .iter()
                .min_by_key(|e| {
                    let diff = (e.width() as i32 - size as i32).abs();
                    diff
                })
                .ok_or_else(|| anyhow::anyhow!("Empty ICO file"))?
        } else {
            // Use largest available
            ico_image
                .entries()
                .iter()
                .max_by_key(|e| e.width())
                .ok_or_else(|| anyhow::anyhow!("Empty ICO file"))?
        };
        
        let image = entry.decode()?;
        let rgba = image.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let pixels = rgba.into_raw();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture = ctx.load_texture(
            format!("{}@{}x{}", path.display(), size[0], size[1]),
            color_image,
            egui::TextureOptions::LINEAR,
        );
        
        Ok(texture)
    }
    
    /// Resolve relative path to absolute theme asset path.
    fn resolve_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.theme_path.join(path)
        }
    }
    
    /// Clear all cached assets (for theme switching).
    pub fn clear_cache(&mut self) {
        self.texture_cache.clear();
        self.sprite_cache.clear();
    }
}

/// A loaded sprite sheet with sprite regions.
#[derive(Clone)]
pub struct SpriteSheet {
    /// The full texture
    pub texture: TextureHandle,
    /// Individual sprite rectangles
    pub sprites: Vec<SpriteRect>,
    /// Full texture size
    pub full_size: Vec2,
}

impl SpriteSheet {
    /// Get UV coordinates for a specific sprite.
    pub fn sprite_uv(&self, index: usize) -> Option<egui::Rect> {
        let sprite = self.sprites.get(index)?;
        
        // Convert pixel coordinates to UV coordinates (0.0 to 1.0)
        Some(egui::Rect::from_min_max(
            egui::pos2(
                sprite.x as f32 / self.full_size.x,
                sprite.y as f32 / self.full_size.y,
            ),
            egui::pos2(
                (sprite.x + sprite.width) as f32 / self.full_size.x,
                (sprite.y + sprite.height) as f32 / self.full_size.y,
            ),
        ))
    }
    
    /// Get pixel size of a specific sprite.
    pub fn sprite_size(&self, index: usize) -> Option<Vec2> {
        let sprite = self.sprites.get(index)?;
        Some(Vec2::new(sprite.width as f32, sprite.height as f32))
    }
    
    /// Render a specific sprite from the sheet.
    pub fn render_sprite(
        &self,
        painter: &egui::Painter,
        index: usize,
        rect: egui::Rect,
        tint: Color32,
    ) {
        if let Some(uv_rect) = self.sprite_uv(index) {
            painter.image(self.texture.id(), rect, uv_rect, tint);
        }
    }
}
```

### Usage in Renderers

```rust
pub struct WindowsXPAppLauncherRenderer {
    /// Asset loader for this theme
    asset_loader: AssetLoader,
    
    /// Start button sprite sheet (3 states: normal, hover, pressed)
    start_button: Option<SpriteSheet>,
    
    /// Button size (scaled to taskbar height)
    button_size: Vec2,
}

impl WindowsXPAppLauncherRenderer {
    pub fn new(theme_path: impl Into<PathBuf>) -> Self {
        Self {
            asset_loader: AssetLoader::new(theme_path),
            start_button: None,
            button_size: Vec2::new(97.0, 30.0),
        }
    }
}

impl Renderer for WindowsXPAppLauncherRenderer {
    fn id(&self) -> &str {
        "windowsxp_start_button"
    }
    
    fn initialize(&mut self, ctx: &Context) {
        // Load sprite sheet with 3 vertically stacked button states
        if let Ok(sprite_sheet) = self.asset_loader.load_sprite_sheet(
            ctx,
            "start_button.png",
            SpriteLayout::VerticalStack { count: 3 },
        ) {
            self.start_button = Some(sprite_sheet);
        }
    }
    
    fn cleanup(&mut self) {
        self.asset_loader.clear_cache();
        self.start_button = None;
    }
}

impl AppLauncherRenderer for WindowsXPAppLauncherRenderer {
    fn render_button(
        &mut self,
        ui: &mut Ui,
        launcher: &AppLauncher,
        theme: &Theme,
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(
            self.button_size,
            Sense::click(),
        );
        
        if ui.is_rect_visible(rect) {
            if let Some(ref sprite_sheet) = self.start_button {
                // Select sprite based on state
                let sprite_index = if response.is_pointer_button_down_on() {
                    2 // Pressed state
                } else if response.hovered() {
                    1 // Hover state
                } else {
                    0 // Normal state
                };
                
                // Render the appropriate sprite
                sprite_sheet.render_sprite(
                    ui.painter(),
                    sprite_index,
                    rect,
                    Color32::WHITE,
                );
            } else {
                // Fallback rendering
                self.render_fallback_button(ui, rect, &response);
            }
        }
        
        response
    }
    
    // ... other methods
}
```

### Background Image Widget

```rust
/// Widget for rendering background images with various scale modes.
pub struct BackgroundImageWidget {
    asset: Option<TextureHandle>,
    scale_mode: ScaleMode,
    asset_loader: AssetLoader,
    image_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum ScaleMode {
    /// Stretch to fill (may distort)
    Stretch,
    /// Fit within bounds (maintain aspect ratio, may show bars)
    Fit,
    /// Fill bounds (maintain aspect ratio, may crop)
    Fill,
    /// Tile/repeat pattern
    Tile,
    /// Center at actual size
    Center,
}

impl BackgroundImageWidget {
    pub fn new(
        theme_path: impl Into<PathBuf>,
        image_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            asset: None,
            scale_mode: ScaleMode::Fill,
            asset_loader: AssetLoader::new(theme_path),
            image_path: image_path.into(),
        }
    }
    
    pub fn with_scale_mode(mut self, mode: ScaleMode) -> Self {
        self.scale_mode = mode;
        self
    }
    
    pub fn initialize(&mut self, ctx: &Context) {
        if let Ok(texture) = self.asset_loader.load_image(ctx, &self.image_path) {
            self.asset = Some(texture);
        }
    }
    
    pub fn render(&self, ui: &mut Ui, available_rect: egui::Rect) {
        if let Some(ref texture) = self.asset {
            let texture_size = texture.size_vec2();
            
            match self.scale_mode {
                ScaleMode::Stretch => {
                    // Simply stretch to fill
                    ui.painter().image(
                        texture.id(),
                        available_rect,
                        egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
                ScaleMode::Fit => {
                    // Maintain aspect ratio, fit within bounds
                    let aspect = texture_size.x / texture_size.y;
                    let available_aspect = available_rect.width() / available_rect.height();
                    
                    let display_rect = if aspect > available_aspect {
                        // Image is wider - fit to width
                        let height = available_rect.width() / aspect;
                        let y_offset = (available_rect.height() - height) / 2.0;
                        egui::Rect::from_min_size(
                            pos2(available_rect.min.x, available_rect.min.y + y_offset),
                            vec2(available_rect.width(), height),
                        )
                    } else {
                        // Image is taller - fit to height
                        let width = available_rect.height() * aspect;
                        let x_offset = (available_rect.width() - width) / 2.0;
                        egui::Rect::from_min_size(
                            pos2(available_rect.min.x + x_offset, available_rect.min.y),
                            vec2(width, available_rect.height()),
                        )
                    };
                    
                    ui.painter().image(
                        texture.id(),
                        display_rect,
                        egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
                ScaleMode::Fill => {
                    // Maintain aspect ratio, fill bounds (may crop)
                    let aspect = texture_size.x / texture_size.y;
                    let available_aspect = available_rect.width() / available_rect.height();
                    
                    let (uv_rect, display_rect) = if aspect > available_aspect {
                        // Image is wider - crop sides
                        let visible_width = texture_size.y * available_aspect;
                        let x_offset = (texture_size.x - visible_width) / 2.0;
                        
                        (
                            egui::Rect::from_min_max(
                                pos2(x_offset / texture_size.x, 0.0),
                                pos2((x_offset + visible_width) / texture_size.x, 1.0),
                            ),
                            available_rect,
                        )
                    } else {
                        // Image is taller - crop top/bottom
                        let visible_height = texture_size.x / available_aspect;
                        let y_offset = (texture_size.y - visible_height) / 2.0;
                        
                        (
                            egui::Rect::from_min_max(
                                pos2(0.0, y_offset / texture_size.y),
                                pos2(1.0, (y_offset + visible_height) / texture_size.y),
                            ),
                            available_rect,
                        )
                    };
                    
                    ui.painter().image(texture.id(), display_rect, uv_rect, Color32::WHITE);
                }
                ScaleMode::Tile => {
                    // Repeat pattern across entire area
                    let tile_count_x = (available_rect.width() / texture_size.x).ceil() as usize;
                    let tile_count_y = (available_rect.height() / texture_size.y).ceil() as usize;
                    
                    for ty in 0..tile_count_y {
                        for tx in 0..tile_count_x {
                            let tile_rect = egui::Rect::from_min_size(
                                pos2(
                                    available_rect.min.x + tx as f32 * texture_size.x,
                                    available_rect.min.y + ty as f32 * texture_size.y,
                                ),
                                texture_size,
                            );
                            
                            ui.painter().image(
                                texture.id(),
                                tile_rect,
                                egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        }
                    }
                }
                ScaleMode::Center => {
                    // Show at actual size, centered
                    let x_offset = (available_rect.width() - texture_size.x) / 2.0;
                    let y_offset = (available_rect.height() - texture_size.y) / 2.0;
                    
                    let display_rect = egui::Rect::from_min_size(
                        pos2(
                            available_rect.min.x + x_offset,
                            available_rect.min.y + y_offset,
                        ),
                        texture_size,
                    );
                    
                    ui.painter().image(
                        texture.id(),
                        display_rect,
                        egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
        }
    }
}
```

### Theme Asset Organization

Recommended directory structure for theme assets:

```
themes/
└── windowsxp/
    ├── theme.toml              # Theme configuration
    ├── assets/
    │   ├── start_button.png    # Sprite sheet: 3 states stacked
    │   ├── taskbar_button_normal.png
    │   ├── taskbar_button_active.png
    │   ├── wallpaper.jpg       # Desktop background
    │   ├── panel_gradient.png  # Optional: pre-rendered gradient
    │   └── icons/
    │       ├── folder.ico      # ICO files for Windows icons
    │       ├── computer.ico
    │       └── mycomputer.ico
    └── README.md               # Theme documentation
```

In theme.toml:

```toml
[assets]
# Sprite sheets
start_button = { path = "assets/start_button.png", type = "sprite_vertical", count = 3 }

# Background images
wallpaper = { path = "assets/wallpaper.jpg", scale_mode = "fill" }

# Icon directory
icon_path = "assets/icons"

# Individual images
taskbar_button_normal = "assets/taskbar_button_normal.png"
taskbar_button_active = "assets/taskbar_button_active.png"
```

### Performance Considerations

1. **Lazy loading**: Only load assets when renderer is initialized
2. **Texture caching**: Reuse same texture for multiple UI elements
3. **Mipmaps**: For scaled images, consider generating mipmaps
4. **Asset cleanup**: Clear cache when switching themes
5. **Sprite atlases**: Combine small icons into single texture atlas

---

## Integration with Existing Systems

### Connection to PROPOSAL.md

The theme architecture directly supports [PROPOSAL.md](PROPOSAL.md)'s vision:

- ✅ **Template-driven flexibility**: Themes ARE the template system
- ✅ **Touch-first design**: Each theme renderer optimizes for touch targets
- ✅ **Hardware control integration**: Functional components include hardware widgets
- ✅ **Profile-based system**: Profiles can specify preferred theme

**Example**: A cyberdeck profile might use a custom theme with large hardware control buttons, while a media center profile uses a TV-friendly theme with remote-optimized navigation.

### Connection to UI_FABRIC_PROPOSAL.md

The renderer architecture is **complementary** to [UI_FABRIC_PROPOSAL.md](UI_FABRIC_PROPOSAL.md):

- **UI Fabric**: External processes declare UI via sockets → Weaver renders
- **Theme Renderers**: Native Weaver components render consistently per active theme

**Synergy**: When a UI Fabric app declares a button, Weaver renders it using the active theme's button style. A Windows XP theme shows XP-style buttons, GNOME theme shows GNOME-style buttons—even for externally-defined UI.

```rust
// UI Fabric receives button declaration from external process
let fabric_button = FabricWidget::Button {
    label: "Submit Order",
    action: "order.submit",
};

// Rendered using active theme's button renderer
theme_renderers.button.render_fabric_button(ui, &fabric_button);
```

This ensures visual consistency: all UI—native Weaver components AND external fabric UIs—follows the active theme.

---

## Benefits & Trade-offs

### Benefits

✅ **Visual flexibility**: Unlimited theme variations without code changes  
✅ **Code reuse**: Component logic written once, rendered many ways  
✅ **Testability**: Test functional components separately from visual rendering  
✅ **Extensibility**: Community themes without forking  
✅ **Performance**: Only active theme's assets loaded  
✅ **Maintainability**: Clear separation of concerns  
✅ **User choice**: Pick theme that matches workflow/aesthetic preference  

### Trade-offs

⚠️ **Complexity**: More abstraction layers than direct widget approach  
⚠️ **Initial overhead**: Requires refactoring existing code  
⚠️ **Trait overhead**: Dynamic dispatch has minor performance cost (negligible for UI)  
⚠️ **Testing burden**: Must test each renderer implementation  
⚠️ **Documentation**: Theme developers need comprehensive docs  

**Verdict**: The benefits vastly outweigh trade-offs for a project with Weaver's ambitions. The architecture supports the core vision while remaining pragmatic.

---

## Migration Path for Existing Code

### Step 1: Identify Current Theme-Specific Code

Current widgets in [desktop_shell.rs](../crates/weaver_desktop_shell/src/components/desktop_shell.rs):

- `XpStartButton` → Extract to `WindowsXPAppLauncherRenderer`
- `XpTaskbar` → Extract to `WindowsXPBarRenderer` (or part of theme-level layout)
- `XpClock` → Extract to `WindowsXPClockRenderer`
- `MenuButton` → Make generic, use in `DefaultAppLauncherRenderer`
- `ClockWidget` → Becomes `Clock` functional component

### Step 2: Create Functional Component Base

```rust
// Before: Theme-specific widget
pub struct XpStartButton {
    image_path: PathBuf,
    texture: Option<TextureHandle>,
    // ... rendering details
}

// After: Split into functional + renderer
pub struct AppLauncher {
    state: MenuState,
    apps: Vec<AppEntry>,
    // ... business logic only
}

pub struct WindowsXPAppLauncherRenderer {
    image_path: PathBuf,
    texture: Option<TextureHandle>,
    // ... rendering details
}
```

### Step 3: Gradual Migration

1. **Keep existing widgets working** during migration
2. **Introduce new system alongside** old system
3. **Port one component at a time** (start with AppLauncher)
4. **Remove old implementation** once new system validated
5. **Update DesktopShell** to use new architecture

**No big-bang rewrite**—incremental, testable migration.

---

## Success Metrics

### MVP Success (End of Phase 1)

- [ ] 2 themes working (WindowsXP + Default)
- [ ] Can switch themes at runtime
- [ ] All components functional in both themes
- [ ] No visual regressions

### Feature Complete (End of Phase 2)

- [ ] Theme definitions in TOML files
- [ ] Settings UI for theme selection
- [ ] Theme changes persist across restarts
- [ ] User themes discovered from config directory

### Production Ready (End of Phase 3)

- [ ] 5+ distinct themes shipping
- [ ] Visual regression tests passing
- [ ] Performance benchmarks acceptable
- [ ] Documentation complete

### Community Enabled (End of Phase 4)

- [ ] Plugin system working
- [ ] Example community theme exists
- [ ] Theme developer docs published
- [ ] Theme submission process defined

---

## Open Questions

1. **Asset bundling**: Embed theme assets in binary or load from disk?
   - **Proposal**: Built-in themes embedded, user themes from disk

2. **Hot reload**: Should theme changes apply without restart?
   - **Proposal**: Yes for development, optional for production

3. **Theme mixing**: Can user mix components from different themes?
   - **Proposal**: Phase 4 feature—advanced customization

4. **Fallback strategy**: What if theme missing a renderer?
   - **Proposal**: Fall back to Default theme renderer for that component

5. **Theme versioning**: How to handle theme API changes?
   - **Proposal**: Semantic versioning, deprecation warnings

---

## References

- [PROPOSAL.md](PROPOSAL.md) - Overall Weaver Desktop vision
- [UI_FABRIC_PROPOSAL.md](UI_FABRIC_PROPOSAL.md) - External process UI integration
- [ARCHITECTURE_ROADMAP.md](ARCHITECTURE_ROADMAP.md) - Development phases
- [TODO.md](TODO.md) - Current implementation tasks

---

## Appendix: Example Theme Gallery

### Windows XP

- Blue gradient taskbar
- Green "start" button with Windows logo
- Classic two-column Start menu
- System tray with clock showing gradient background
- Window buttons with grouping

### Windows 7

- Translucent Aero glass taskbar
- Glowing orb Start button
- Jump lists on taskbar buttons
- Peek preview on hover
- System tray with chevron for hidden icons

### GNOME Shell

- Dark top panel
- "Activities" text trigger in corner
- Fullscreen app grid overlay
- Vertical workspace switcher
- Integrated system menu

### macOS

- Translucent top menu bar
- Apple menu in top-left
- App name in menu bar
- Dock at bottom with magnification
- App icons bounce on launch

### Kiosk Mode

- Minimal chrome
- Large touch-optimized buttons
- Single-purpose focused interface
- Locked navigation
- No window management visible

---

**Status**: Ready for review and implementation planning.
