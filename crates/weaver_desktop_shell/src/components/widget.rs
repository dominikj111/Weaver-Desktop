//! WidgetStr - the universal building block for Flexbox-inspired layouts.
//!
//! Everything in the UI is a WidgetStr. Widgets can be containers (holding children)
//! or leaves (rendering custom content). Layout follows CSS Flexbox principles:
//!
//! - `direction`: Row (horizontal) or Column (vertical)
//! - `width/height`: Fixed pixels, Flex weight, or Content-fit
//! - `align`: Cross-axis alignment (Start, Center, End, Stretch)
//! - `justify`: Main-axis distribution (Start, Center, End, SpaceBetween, etc.)
//! - `gap`: Fixed space between children
//! - `padding`: Inner spacing
//! - `margin`: Outer spacing
//!
//! # Example
//!
//! ```rust,ignore
//! WidgetStr::column("desktop")
//!     .child(
//!         WidgetStr::row("top-bar")
//!             .height(Size::Fixed(40.0))
//!             .padding(Spacing::xy(12.0, 8.0))
//!             .align(Align::Center)
//!             .justify(Justify::SpaceBetween)
//!             .child(WidgetStr::leaf("menu", MenuButton::new()))
//!             .child(WidgetStr::leaf("title", Label::new("Desktop")))
//!             .child(WidgetStr::leaf("clock", Clock::new()))
//!     )
//!     .child(
//!         WidgetStr::column("content")
//!             .height(Size::Flex(1.0))
//!     )
//! ```

use egui::{Color32, Rect, Ui, Vec2};

use super::ImageSurface;

// ============================================================================
// Layout Types
// ============================================================================

/// Layout axis - how children are arranged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    /// Horizontal: children laid out left-to-right.
    #[default]
    Row,
    /// Vertical: children laid out top-to-bottom.
    Column,
}

/// Size along an axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    /// Exact pixels.
    Fixed(f32),
    /// Weight for distributing remaining space (like flex-grow).
    Flex(f32),
    /// Shrink to fit content.
    Content,
}

impl Default for Size {
    fn default() -> Self {
        Self::Flex(1.0)
    }
}

impl Size {
    /// Flex with weight 1.0 (default flex behavior).
    pub fn flex() -> Self {
        Self::Flex(1.0)
    }

    /// Check if this is a flex size.
    pub fn is_flex(&self) -> bool {
        matches!(self, Size::Flex(_))
    }

    /// Get flex weight, or 0 if not flex.
    pub fn flex_weight(&self) -> f32 {
        match self {
            Size::Flex(w) => *w,
            _ => 0.0,
        }
    }
}

/// Cross-axis alignment (like CSS align-items).
///
/// For Row: vertical alignment of children.
/// For Column: horizontal alignment of children.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    /// Align to start (top for Row, left for Column).
    Start,
    /// Center on cross-axis.
    Center,
    /// Align to end (bottom for Row, right for Column).
    End,
    /// Stretch to fill cross-axis (default).
    #[default]
    Stretch,
}

/// Main-axis distribution (like CSS justify-content).
///
/// For Row: horizontal distribution of children.
/// For Column: vertical distribution of children.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Justify {
    /// Pack children to start.
    #[default]
    Start,
    /// Pack children to center.
    Center,
    /// Pack children to end.
    End,
    /// Even gaps between children, no space at edges.
    SpaceBetween,
    /// Even gaps between children, half-size space at edges.
    SpaceAround,
    /// Equal space everywhere (between children and at edges).
    SpaceEvenly,
}

/// How content overflow is handled (like CSS overflow).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    /// Clip content at widget bounds (default, safest).
    #[default]
    Clip,
    /// Allow content to overflow (no clipping).
    Visible,
    /// Show scrollbars when content exceeds bounds.
    Scroll,
    /// Show scrollbars only when needed.
    Auto,
}

/// Spacing for padding and margin.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    /// Zero spacing.
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    /// Same spacing on all sides.
    pub fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }

    /// Horizontal and vertical spacing.
    pub fn xy(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Specific spacing for each side.
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Total horizontal spacing (left + right).
    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    /// Total vertical spacing (top + bottom).
    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

impl From<f32> for Spacing {
    fn from(v: f32) -> Self {
        Self::all(v)
    }
}

impl From<(f32, f32)> for Spacing {
    fn from((h, v): (f32, f32)) -> Self {
        Self::xy(h, v)
    }
}

// ============================================================================
// Widget
// ============================================================================

pub trait Widget {
    fn id(&self) -> &str;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn min_size(&self) -> Vec2;
    fn ui(&mut self, ui: &mut Ui, rect: Rect);

    // Layout
    /// Compute layout for the given rect. Called before rendering.
    /// Containers should recursively compute layout for their children.
    fn compute_layout(&mut self, rect: Rect);

    // State
    fn is_visible(&self) -> bool;
    fn is_disabled(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn set_disabled(&mut self, disabled: bool);
}

pub struct Style {
    // Layout
    pub width: Size,
    pub height: Size,
    pub align: Align,
    pub justify: Justify,
    pub gap: f32,
    pub padding: Spacing,
    pub margin: Spacing,
    pub overflow: Overflow,

    // Appearance
    pub background: Option<ImageSurface>,
    pub border_radius: f32,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: Size::Flex(1.0),
            height: Size::Flex(1.0),
            align: Align::default(),
            justify: Justify::default(),
            gap: 0.0,
            padding: Spacing::ZERO,
            margin: Spacing::ZERO,
            overflow: Overflow::default(),
            background: None,
            border_radius: 0.0,
        }
    }
}

/// Cached layout computation results.
#[derive(Debug, Clone)]
struct CachedLayout {
    /// The rect this layout was computed for.
    for_rect: Rect,
    /// Content rect after margin and padding.
    content_rect: Rect,
    /// Computed rects for each child.
    child_rects: Vec<Rect>,
}

pub struct Container {
    id: String,
    style: Style,
    children: Vec<Box<dyn Widget>>,
    axis: Axis,

    // State
    disabled: bool,
    visible: bool,

    // Layout caching (prevents flickering by keeping stale layout until fresh one is ready)
    cached_layout: Option<CachedLayout>,
    layout_dirty: bool, // True when layout needs recomputation
}

impl Container {
    pub fn row(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            style: Style::new(),
            children: Vec::new(),
            axis: Axis::Row,
            disabled: false,
            visible: true,
            cached_layout: None,
            layout_dirty: true,
        }
    }

    pub fn column(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            style: Style::new(),
            children: Vec::new(),
            axis: Axis::Column,
            disabled: false,
            visible: true,
            cached_layout: None,
            layout_dirty: true,
        }
    }

    pub fn child(mut self, widget: Box<dyn Widget>) -> Self {
        self.children.push(widget);
        self.layout_dirty = true; // Mark for recomputation, keep stale layout
        self
    }

    pub fn width(mut self, size: Size) -> Self {
        self.style.width = size;
        self
    }

    pub fn height(mut self, size: Size) -> Self {
        self.style.height = size;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.style.align = align;
        self
    }

    pub fn justify(mut self, justify: Justify) -> Self {
        self.style.justify = justify;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.style.gap = gap;
        self
    }

    pub fn padding(mut self, padding: impl Into<Spacing>) -> Self {
        self.style.padding = padding.into();
        self
    }

    pub fn margin(mut self, margin: impl Into<Spacing>) -> Self {
        self.style.margin = margin.into();
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.style.overflow = overflow;
        self
    }

    pub fn background(mut self, surface: ImageSurface) -> Self {
        self.style.background = Some(surface);
        self
    }

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.style.border_radius = radius;
        self
    }

    /// Manually invalidate cached layout.
    /// Call this after modifying style or children outside of builders.
    /// The stale layout will continue to be used until compute_layout() is called.
    pub fn invalidate_layout(&mut self) {
        self.layout_dirty = true;
    }

    /// Check if layout needs recomputation.
    /// Returns true if dirty flag is set or rect has changed.
    pub fn needs_layout(&self, rect: Rect) -> bool {
        self.layout_dirty
            || self
                .cached_layout
                .as_ref()
                .map(|layout| layout.for_rect != rect)
                .unwrap_or(true)
    }
}

impl Widget for Container {
    fn id(&self) -> &str {
        &self.id
    }
    fn style(&self) -> &Style {
        &self.style
    }
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn min_size(&self) -> Vec2 {
        let mut size = Vec2::ZERO;

        for child in &self.children {
            let child_size = child.min_size();
            match self.axis {
                Axis::Row => {
                    size.x += child_size.x;
                    size.y = size.y.max(child_size.y);
                }
                Axis::Column => {
                    size.x = size.x.max(child_size.x);
                    size.y += child_size.y;
                }
            }
        }

        let gap_count = self.children.len().saturating_sub(1) as f32;
        match self.axis {
            Axis::Row => size.x += self.style.gap * gap_count,
            Axis::Column => size.y += self.style.gap * gap_count,
        }

        Vec2::new(
            size.x + self.style.padding.horizontal() + self.style.margin.horizontal(),
            size.y + self.style.padding.vertical() + self.style.margin.vertical(),
        )
    }

    fn ui(&mut self, ui: &mut Ui, rect: Rect) {
        if !self.visible {
            return;
        }

        // Use cached layout - render with stale layout to prevent flickering
        // Caller is responsible for calling compute_layout() to refresh
        let Some(layout) = self.cached_layout.as_ref() else {
            // No layout ever computed - skip rendering
            return;
        };

        // Inherit parent's clip rect
        let parent_clip = ui.clip_rect();

        // Apply margin to get widget bounds
        let widget_rect = Rect::from_min_max(
            egui::pos2(
                rect.min.x + self.style.margin.left,
                rect.min.y + self.style.margin.top,
            ),
            egui::pos2(
                rect.max.x - self.style.margin.right,
                rect.max.y - self.style.margin.bottom,
            ),
        );

        // Calculate effective clip rect for this widget
        let effective_clip = widget_rect.intersect(parent_clip);

        // Skip rendering if completely clipped
        if effective_clip.width() <= 0.0 || effective_clip.height() <= 0.0 {
            return;
        }

        // Draw background (respects parent clip)
        if let Some(ref mut bg) = self.style.background {
            ui.painter().with_clip_rect(effective_clip).rect(
                widget_rect,
                self.style.border_radius,
                Color32::TRANSPARENT,
                egui::Stroke::NONE,
                egui::StrokeKind::Middle,
            );
            bg.render(ui, widget_rect, self.style.border_radius);
        }

        // Use cached child rects from layout
        let child_rects = &layout.child_rects;

        // Render children based on overflow mode
        match self.style.overflow {
            Overflow::Clip => {
                let content_clip = layout.content_rect.intersect(effective_clip);
                ui.with_clip_rect(content_clip, |ui| {
                    for (child, child_rect) in self.children.iter_mut().zip(child_rects.iter()) {
                        child.ui(ui, *child_rect);
                    }
                });
            }
            Overflow::Visible => {
                // No clipping - use parent's clip rect
                for (child, child_rect) in self.children.iter_mut().zip(child_rects.iter()) {
                    child.ui(ui, *child_rect);
                }
            }
            Overflow::Scroll | Overflow::Auto => {
                // TODO: Implement scrolling - for now, clip like Overflow::Clip
                let content_clip = layout.content_rect.intersect(effective_clip);
                ui.with_clip_rect(content_clip, |ui| {
                    for (child, child_rect) in self.children.iter_mut().zip(child_rects.iter()) {
                        child.ui(ui, *child_rect);
                    }
                });
            }
        }

        // Draw disabled overlay
        if self.disabled {
            ui.painter().with_clip_rect(effective_clip).rect_filled(
                widget_rect,
                self.style.border_radius,
                Color32::from_black_alpha(128),
            );
        }
    }

    fn compute_layout(&mut self, rect: Rect) {
        // Apply margin to get widget bounds
        let widget_rect = Rect::from_min_max(
            egui::pos2(
                rect.min.x + self.style.margin.left,
                rect.min.y + self.style.margin.top,
            ),
            egui::pos2(
                rect.max.x - self.style.margin.right,
                rect.max.y - self.style.margin.bottom,
            ),
        );

        // Apply padding to get content area
        let content_rect = Rect::from_min_max(
            egui::pos2(
                widget_rect.min.x + self.style.padding.left,
                widget_rect.min.y + self.style.padding.top,
            ),
            egui::pos2(
                widget_rect.max.x - self.style.padding.right,
                widget_rect.max.y - self.style.padding.bottom,
            ),
        );

        // Compute child rects
        let child_rects = self.compute_child_rects(content_rect);

        // Recursively compute layout for children
        for (child, child_rect) in self.children.iter_mut().zip(&child_rects) {
            child.compute_layout(*child_rect);
        }

        // Update cache and clear dirty flag
        self.cached_layout = Some(CachedLayout {
            for_rect: rect,
            content_rect,
            child_rects,
        });
        self.layout_dirty = false;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
    fn is_disabled(&self) -> bool {
        self.disabled
    }
    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

impl Container {
    /// Compute child rects using flexbox layout algorithm.
    fn compute_child_rects(&self, content_rect: Rect) -> Vec<Rect> {
        if self.children.is_empty() {
            return Vec::new();
        }

        let n = self.children.len();

        // Calculate sizes along main axis
        let (main_size, cross_size) = match self.axis {
            Axis::Row => (content_rect.width(), content_rect.height()),
            Axis::Column => (content_rect.height(), content_rect.width()),
        };

        // First pass: calculate fixed and content sizes, sum flex weights
        let mut fixed_total = 0.0f32;
        let mut flex_total = 0.0f32;
        let mut sizes: Vec<f32> = Vec::with_capacity(n);

        for child in &self.children {
            let style = child.style();
            let main_spec = match self.axis {
                Axis::Row => style.width,
                Axis::Column => style.height,
            };

            match main_spec {
                Size::Fixed(v) => {
                    sizes.push(v);
                    fixed_total += v;
                }
                Size::Flex(weight) => {
                    sizes.push(0.0); // Will be resolved in second pass
                    flex_total += weight;
                }
                Size::Content => {
                    let min = child.min_size();
                    let content_size = match self.axis {
                        Axis::Row => min.x,
                        Axis::Column => min.y,
                    };
                    sizes.push(content_size);
                    fixed_total += content_size;
                }
            }
        }

        // Account for gaps
        let gap_total = self.style.gap * (n.saturating_sub(1)) as f32;
        let remaining = (main_size - fixed_total - gap_total).max(0.0);

        // Second pass: resolve flex sizes
        for (i, child) in self.children.iter().enumerate() {
            let style = child.style();
            let main_spec = match self.axis {
                Axis::Row => style.width,
                Axis::Column => style.height,
            };

            if let Size::Flex(weight) = main_spec {
                if flex_total > 0.0 {
                    sizes[i] = (weight / flex_total) * remaining;
                }
            }
        }

        // Calculate positions based on justify
        let total_children_size: f32 = sizes.iter().sum();
        let total_with_gaps = total_children_size + gap_total;
        let extra_space = (main_size - total_with_gaps).max(0.0);

        let (start_offset, between_gap) = match self.style.justify {
            Justify::Start => (0.0, self.style.gap),
            Justify::Center => (extra_space / 2.0, self.style.gap),
            Justify::End => (extra_space, self.style.gap),
            Justify::SpaceBetween => {
                if n > 1 {
                    (0.0, self.style.gap + extra_space / (n - 1) as f32)
                } else {
                    (0.0, self.style.gap)
                }
            }
            Justify::SpaceAround => {
                let gap = extra_space / n as f32;
                (gap / 2.0, self.style.gap + gap)
            }
            Justify::SpaceEvenly => {
                let gap = extra_space / (n + 1) as f32;
                (gap, self.style.gap + gap)
            }
        };

        // Third pass: calculate final rects
        let mut rects = Vec::with_capacity(n);
        let mut pos = start_offset;

        for (i, child) in self.children.iter().enumerate() {
            let main_child_size = sizes[i];
            let style = child.style();

            // Determine cross-axis size
            let cross_spec = match self.axis {
                Axis::Row => style.height,
                Axis::Column => style.width,
            };

            let cross_child_size = match cross_spec {
                Size::Fixed(v) => v.min(cross_size),
                Size::Flex(_) => cross_size, // Flex on cross-axis = fill available
                Size::Content => {
                    let min = child.min_size();
                    let content_size = match self.axis {
                        Axis::Row => min.y,
                        Axis::Column => min.x,
                    };
                    content_size.min(cross_size)
                }
            };

            // Apply cross-axis alignment
            let cross_offset = match self.style.align {
                Align::Start => 0.0,
                Align::Center => (cross_size - cross_child_size) / 2.0,
                Align::End => cross_size - cross_child_size,
                Align::Stretch => 0.0, // Stretching handled by cross_child_size = cross_size
            };

            let aligned_cross_size = if self.style.align == Align::Stretch {
                cross_size
            } else {
                cross_child_size
            };

            // Build the rect based on axis
            let rect = match self.axis {
                Axis::Row => Rect::from_min_size(
                    egui::pos2(content_rect.min.x + pos, content_rect.min.y + cross_offset),
                    egui::vec2(main_child_size, aligned_cross_size),
                ),
                Axis::Column => Rect::from_min_size(
                    egui::pos2(content_rect.min.x + cross_offset, content_rect.min.y + pos),
                    egui::vec2(aligned_cross_size, main_child_size),
                ),
            };

            rects.push(rect);
            pos += main_child_size;
            if i < n - 1 {
                pos += between_gap;
            }
        }

        rects
    }
}
