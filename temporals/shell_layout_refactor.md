# Shell Layout Refactor Tasks

**Goal**: Unified widget-based architecture inspired by CSS Flexbox - everything is a widget.

**Core Insight**: Desktop, bars, panels, content areas are all **widgets in a tree**. A widget can contain other widgets. Layout is achieved through **direction** (row/column) and **sizing** (fixed, flex, content).

---

## The Widget Model (Flexbox-Inspired)

```
┌─────────────────────────────────────────────────────┐
│ Desktop (Column)                                    │
│ ┌─────────────────────────────────────────────────┐ │
│ │ Top Bar (Row, height: Fixed(40))                │ │
│ │   [AppMenu] [Title] [StatusItems →]             │ │
│ └─────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────┐ │
│ │ Main Area (Row, height: Flex(1))                │ │
│ │ ┌─────────┐ ┌─────────────────────┐ ┌─────────┐ │ │
│ │ │ Left    │ │ Content             │ │ Right   │ │ │
│ │ │ Panel   │ │ (Flex(1))           │ │ Panel   │ │ │
│ │ │ Fixed   │ │                     │ │ Fixed   │ │ │
│ │ └─────────┘ └─────────────────────┘ └─────────┘ │ │
│ └─────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────┐ │
│ │ Bottom Bar (Row, height: Fixed(30))             │ │
│ │   [Status Items]                                │ │
│ └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

### Key Types

```rust
/// Layout direction (like flex-direction)
pub enum Direction {
    Row,      // Horizontal: children laid out left-to-right
    Column,   // Vertical: children laid out top-to-bottom
}

/// Size along an axis (like flex-basis + flex-grow)
pub enum Size {
    Fixed(f32),    // Exact pixels
    Flex(f32),     // Weight for remaining space (like flex-grow)
    Content,       // Shrink to fit content
}

/// Alignment on cross-axis (like align-items)
/// Row: vertical alignment | Column: horizontal alignment
pub enum Align {
    Start,     // Top (row) or Left (column)
    Center,    // Center on cross-axis
    End,       // Bottom (row) or Right (column)
    Stretch,   // Fill cross-axis (default)
}

/// Distribution on main-axis (like justify-content)
/// Row: horizontal distribution | Column: vertical distribution
pub enum Justify {
    Start,         // Pack to start
    Center,        // Pack to center
    End,           // Pack to end
    SpaceBetween,  // Even gaps, no edge space
    SpaceAround,   // Even gaps, half-size edge space
    SpaceEvenly,   // Equal space everywhere
}

/// Spacing (same for padding and margin)
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    pub fn all(v: f32) -> Self;                           // Same on all sides
    pub fn xy(x: f32, y: f32) -> Self;                    // Horizontal, Vertical
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self;
}

/// Widget - the universal building block
pub struct Widget {
    pub id: String,
    
    // Layout
    pub direction: Direction,         // Row or Column
    pub width: Size,                  // Horizontal size
    pub height: Size,                 // Vertical size
    pub align: Align,                 // Cross-axis alignment
    pub justify: Justify,             // Main-axis distribution
    pub gap: f32,                     // Space between children
    pub padding: Spacing,             // Inner spacing (inside border)
    pub margin: Spacing,              // Outer spacing (outside border)
    
    // Appearance
    pub background: Option<ImageSurface>,
    pub border_radius: f32,
    
    // Content
    pub children: Vec<Widget>,        // Child widgets (container)
    // OR leaf widget renders via trait impl
    
    // State
    pub disabled: bool,               // Blocks interaction (overlay effect)
    pub visible: bool,                // Show/hide
}
```

### Visual Reference

**Margin, Padding, Gap:**

```
┌─────────────────────────────────────────────────────────┐
│                       MARGIN                            │
│   ┌─────────────────────────────────────────────────┐   │
│   │                   PADDING                       │   │
│   │   ┌─────────┐ ←gap→ ┌─────────┐ ←gap→ ┌─────┐   │   │
│   │   │ Child 1 │       │ Child 2 │       │  3  │   │   │
│   │   └─────────┘       └─────────┘       └─────┘   │   │
│   │                   PADDING                       │   │
│   └─────────────────────────────────────────────────┘   │
│                       MARGIN                            │
└─────────────────────────────────────────────────────────┘
```

**Gap vs Justify** (they're different!):

```
gap = 8px (fixed space between each child):
┌──────────────────────────────────────────────────┐
│ [A]--8px--[B]--8px--[C]                          │  ← packed left
└──────────────────────────────────────────────────┘

Justify::SpaceBetween (distribute remaining space):
┌──────────────────────────────────────────────────┐
│ [A]              [B]              [C]            │  ← fills width
└──────────────────────────────────────────────────┘

gap = 8px + Justify::SpaceBetween (gap is minimum):
┌──────────────────────────────────────────────────┐
│ [A]              [B]              [C]            │  ← at least 8px apart
└──────────────────────────────────────────────────┘
```

**Justify options** (main-axis distribution):

```
Start:        [A][B][C].......................
Center:       ...........[A][B][C]............
End:          .......................[A][B][C]
SpaceBetween: [A]...........[B]...........[C]   ← edges flush
SpaceAround:  ...[A].......[B].......[C]...     ← half-space at edges  
SpaceEvenly:  ....[A].....[B].....[C]....       ← equal everywhere
```

**Align options** (cross-axis alignment):

```
Direction: Row, height: 100px, children: 40px tall

Start:    ┌─────┐           Stretch: ┌─────┐
          │Child│                    │     │
          └─────┘                    │Child│
                                     │     │
                                     └─────┘

Center:      ┌─────┐        End:           ┌─────┐
             │Child│                       │Child│
             └─────┘                       └─────┘
```

### Why This Model?

1. **Familiar** - Mirrors CSS Flexbox (direction, align, justify, gap, padding, margin)
2. **Simple** - One concept: Widget. No separate Bar, Panel, Layout types
3. **Composable** - Any widget can contain any widget
4. **Config-driven** - Easy to serialize to TOML/JSON
5. **Declarative** - Describe structure, not imperative positioning

### Builder API

```rust
Widget::column("desktop")
    .background(ImageSurface::with_source(ImageSource::Image(path)))
    .child(
        Widget::row("top-bar")
            .height(Size::Fixed(40.0))
            .padding(Spacing::xy(12.0, 8.0))    // Horizontal: 12, Vertical: 8
            .align(Align::Center)                // Vertically center items
            .justify(Justify::SpaceBetween)      // Spread items across
            .gap(8.0)
            .background(ImageSurface::with_source(ImageSource::Color(BAR_COLOR)))
            .child(Widget::leaf("app-menu", AppMenuButton::new()))
            .child(Widget::leaf("title", Label::new("Desktop")).width(Size::Flex(1.0)))
            .child(Widget::leaf("clock", ClockWidget::new()))
    )
    .child(
        Widget::row("main-area")
            .height(Size::Flex(1.0))
            .child(Widget::column("sidebar").width(Size::Fixed(200.0)))
            .child(Widget::column("content").width(Size::Flex(1.0)))
    )
    .child(
        Widget::row("bottom-bar")
            .height(Size::Fixed(30.0))
            .padding(Spacing::xy(12.0, 4.0))
            .align(Align::Center)
            .child(Widget::leaf("status", StatusWidget::new()))
    )
```

### Quick Reference

| Property | What it does | CSS Equivalent |
|----------|--------------|----------------|
| `direction` | Row (→) or Column (↓) | `flex-direction` |
| `width/height` | Fixed, Flex, or Content | `flex-basis` + `flex-grow` |
| `align` | Cross-axis alignment | `align-items` |
| `justify` | Main-axis distribution | `justify-content` |
| `gap` | Space between children | `gap` |
| `padding` | Inner spacing | `padding` |
| `margin` | Outer spacing | `margin` |

---

## Design Principles

- **Everything is a Widget** - Desktop, bars, panels, buttons - all widgets
- **Stateful components** - All state computed outside `ui()`, rendering is pure
- **Flexbox layout** - Row/Column direction, Fixed/Flex/Content sizing
- **Disabled = Overlay** - Widget `disabled` state blocks interaction
- **Config-driven** - Layout describable in TOML/JSON
- **Use Interactable** - Existing pattern for pointer interaction (click, press, release)

---

## Existing Components (POC, working)

- `ImageSurface` - Image/color surface for backgrounds ✅ (Task 1 complete)
- `Background` - Legacy image loading with fallback gradient ✅
- `Bar` - Generic with `BarStyle` trait ✅
- `IconButton` - Circular button with image/SVG support ✅
- `Interactable` - Pointer interaction with SignalFn callbacks ✅
- `LogPanel`, `TerminalPanel`, `Menu` - Commented POC code ✅

---

## Task 1: Create ImageSurface Component ✅ COMPLETE

Reusable primitive for filling containers with images/colors.

**Files Created**:

- [image_surface.rs](../crates/weaver_desktop_shell/src/components/image_surface.rs)

**Types**: `ImageSurface`, `ImageSource` (Image/Color/None), `ScaleMode` (Cover/Contain/Stretch/Tile)

**Methods**: `paint()`, `paint_rect()`, `paint_to_painter()`, `paint_background()`

---

## Task 2: Create Widget Core ✅ COMPLETE

**Files Created**:

- [widget.rs](../crates/weaver_desktop_shell/src/components/widget.rs)

**Types**: `Axis` (Row/Column), `Size` (Fixed/Flex/Content), `Align`, `Justify`, `Spacing`, `Widget`, `WidgetContent`, `Label`, `Spacer`

**Features**:

- Flexbox-inspired layout (axis, size, align, justify, gap, padding, margin)
- Builder pattern for fluent widget construction
- Container widgets with children
- Leaf widgets with `WidgetContent` trait
- Disabled state renders overlay
- Layout algorithm distributes space according to Flex weights

---

## Task 3: Implement Widget Layout Engine ✅ COMPLETE

Layout algorithm implemented in `compute_child_rects()`:

1. Apply margin → widget bounds
2. Apply padding → content area
3. Calculate main-axis sizes (Fixed/Content/Flex)
4. Distribute remaining space by Flex weight
5. Apply Justify for main-axis distribution
6. Apply Align for cross-axis positioning

---

## Task 4: Add Leaf Widget Support ✅ COMPLETE

Implemented `WidgetContent` trait and `Widget::leaf()` constructor.

Built-in content types:

- `Label` - text display with optional color
- `Spacer` - flexible empty space

---

## Task 5: Integrate Interactable with Widget

**Description**:
Allow widgets to be interactive using the existing `Interactable` pattern. Widget can optionally have click/press/release handlers.

**Changes**:

```rust
impl Widget {
    /// Make widget respond to clicks
    pub fn on_click(self, handler: impl Fn() + 'static) -> Self;
    
    /// Make widget respond to press/release
    pub fn on_press(self, handler: impl Fn() + 'static) -> Self;
    pub fn on_release(self, handler: impl Fn() + 'static) -> Self;
}
```

**Implementation**:

- Widget stores optional `Interactable`
- During `ui()`, if interactable exists, wrap content in sense area
- When `disabled = true`, skip interaction handling

**Files**:

- `crates/weaver_desktop_shell/src/components/widget.rs`

**Commit**: `feat(widget): integrate Interactable for click/press/release`

---

## Task 5.5: Overflow Handling

**Problem**: Widget content can render outside its boundaries. Need predictable overflow behavior.

**Overflow Enum** (like CSS overflow):

```rust
/// How content overflow is handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Clip content at widget bounds (default, safest).
    #[default]
    Clip,
    /// Allow content to overflow (current behavior).
    Visible,
    /// Show scrollbars when content exceeds bounds.
    Scroll,
    /// Show scrollbars only when needed.
    Auto,
}
```

**Widget Changes**:

```rust
pub struct Widget {
    // ... existing fields ...
    
    /// How overflow is handled
    pub overflow: Overflow,
}

impl Widget {
    /// Set overflow behavior.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }
}
```

**Rendering Changes**:

```rust
fn render_content(&mut self, ui: &mut Ui, content_rect: Rect) {
    match self.overflow {
        Overflow::Clip => {
            // Clip to content rect
            ui.painter().set_clip_rect(content_rect);
            // ... render children ...
        }
        Overflow::Scroll | Overflow::Auto => {
            // Wrap in ScrollArea
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // ... render children ...
                });
        }
        Overflow::Visible => {
            // Current behavior - no clipping
        }
    }
}
```

**Text Truncation** (for Label widget):

```rust
impl WidgetContent for Label {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(egui::Label::new(&self.text)
            .wrap_mode(egui::TextWrapMode::Truncate));  // Already supports ellipsis
    }
}
```

**Priority Order**:

1. **Clip by default** - Prevents visual bugs, safe
2. **ScrollArea for content** - For long lists, panels
3. **Text truncation** - Already supported via egui

**Files**:

- `crates/weaver_desktop_shell/src/components/widget.rs`

**Commit**: `feat(widget): add Overflow property for clip/scroll behavior`

---

## Task 6: Desktop Shell with Widget Tree ✅ COMPLETE

**Files Created**:

- [desktop_shell.rs](../crates/weaver_desktop_shell/src/components/desktop_shell.rs)
- [modal.rs](../crates/weaver_desktop_shell/src/components/modal.rs)

**Architecture** (4-layer model):

```
Layer 0: Background (ImageSurface)
Layer 1: Desktop (Widget tree)
Layer 2: Modal (centered, floating)
Layer 3: Toasts (notifications)
```

**DesktopShell Features**:

- Widget-based desktop with top-bar, content area, bottom-bar
- Floating menu button (always visible)
- Modal system with backdrop click dismiss
- Desktop dimming when modal is active
- App menu with icon grid (Dashboard, Hardware, Profiles, etc.)
- Power buttons (Restart, Shutdown)

**Modal Features**:

- Content-fit sizing with max 80% of screen
- Backdrop click to dismiss
- Returns `ModalResult::Active` or `ModalResult::Dismissed`
- Shell handles dimming via desktop overlay

**Widget Content Types Created**:

- `ClockWidget` - Time display
- `DateWidget` - Date with button
- `MenuButton` - Hamburger menu button
- `StatusText` - Status bar text
- `VersionLabel` - App version
- `MenuItemContent` - App menu icon + label
- `PowerButtonContent` - Power action buttons
- `DesktopIcon` - Icon entry with label, path, action
- `IconGridWidget` - Grid of clickable icons with texture loading
- `DesktopImageWidget` - Photo frame widget with image + title

**Desktop Content Widgets**:
The desktop can now hold arbitrary widgets in its content area:

```rust
// Example: Places icon grid as a desktop widget
let places_widget = Widget::leaf("places-grid", IconGridWidget::new()
    .with_icons(vec![
        DesktopIcon::new("Home", "places.home").with_icon(path),
        DesktopIcon::new("Documents", "places.docs").with_icon(path),
    ])
    .icon_size(48.0)
    .columns(3)
)
.width(Size::Fixed(220.0))
.height(Size::Fixed(200.0))
.background(ImageSurface::with_source(ImageSource::Color(color)))
.border_radius(12.0);

// Create shell with desktop widgets
let shell = DesktopShell::with_content(vec![places_widget, devices_widget, image_widget]);
```

**Usage**:

```rust
let mut shell = DesktopShell::new();
shell.set_background_image("path/to/wallpaper.jpg");

// In update loop
shell.ui(ctx, |ui| {
    // View content here
});
```

---

## Task 7: Config Serialization

**Description**:
Make Widget tree serializable to TOML/JSON for config-driven layouts.

**Changes**:

- Add `#[derive(Serialize, Deserialize)]` to Widget, Direction, Size
- Handle `WidgetContent` via registry pattern (named content types)
- Add `WidgetRegistry` for resolving content type names to constructors

**Config Example**:

```toml
[[widget]]
id = "desktop"
direction = "column"
background = { type = "image", path = "wallpaper.jpg" }

[[widget.children]]
id = "top-bar"
direction = "row"
height = { fixed = 40 }
background = { type = "color", value = "#1a1a1a" }

[[widget.children.children]]
id = "clock"
type = "clock"
width = "content"
```

**Files**:

- `crates/weaver_desktop_shell/src/components/widget.rs`
- `crates/weaver_desktop_shell/src/config/widget_config.rs` (new)

**Commit**: `feat(widget): add TOML/JSON serialization for config-driven layouts`

---

## Execution Order

| Phase | Tasks | Description | Status |
|-------|-------|-------------|--------|
| **Foundation** | 1 | ImageSurface primitive | ✅ COMPLETE |
| **Widget Core** | 2, 3, 4 | Widget struct + layout engine + leaf support | ✅ COMPLETE |
| **Widget Interaction** | 5 | Interactable integration | |
| **Overflow Handling** | 5.5 | Clip, scroll, truncate | |
| **Integration** | 6 | Desktop Shell with Widget tree + Modal | ✅ COMPLETE |
| **Config** | 7 | TOML/JSON serialization | |

Each task is independently committable. The app should build after each task.

---

## Deferred Features

These are explicitly NOT in scope for this refactor:

- **Drag and drop** - No widget repositioning via mouse
- **Mouse resizing** - No runtime resize handles
- **Grid layout** - Start with Flexbox only, grid can come later
- **Animation** - Widget transitions/animations are future work
- **Touch gestures** - Beyond basic tap, gestures are future work

---

## Preserved POC Components

Keep these commented-out components for reference:

- `log_panel.rs` - Tabbed log viewer
- `terminal_panel.rs` - Embedded alacritty
- `top_menu.rs` - Button grid with Observable

These can be converted to `WidgetContent` implementations later.
