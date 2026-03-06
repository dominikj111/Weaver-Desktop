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
// WidgetStr Content Trait
// ============================================================================

/// Trait for custom widget content (leaf widgets).
///
/// Implement this trait to create custom renderable content that can be
/// placed inside a WidgetStr using `WidgetStr::leaf()`.
pub trait WidgetContent: Send + Sync {
    /// Render the content within the given UI.
    fn ui(&mut self, ui: &mut Ui);

    /// Minimum size required by this content.
    fn min_size(&self) -> Vec2 {
        Vec2::ZERO
    }
}

// ============================================================================
// WidgetStr
// ============================================================================

pub trait Widget {
    fn id(&self) -> &str;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn min_size(&self) -> Vec2;
    fn ui(&mut self, ui: &mut Ui, rect: Rect);

    // State
    fn is_visible(&self) -> bool;
    fn is_disabled(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn set_disabled(&mut self, disabled: bool);
}

pub struct Style {
    // Layout
    width: Size,
    height: Size,
    align: Align,
    justify: Justify,
    gap: f32,
    padding: Spacing,
    margin: Spacing,
    overflow: Overflow,

    // Appearance
    background: Option<ImageSurface>,
    border_radius: f32,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    // ---------------------------------------------------------
    // Builder methods for each style property can be added here
    // ---------------------------------------------------------

    pub fn width(mut self, size: Size) -> Self {
        self.width = size;
        self
    }

    pub fn height(mut self, size: Size) -> Self {
        self.height = size;
        self
    }

    // ... all other builders
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

pub struct Container {
    id: String,
    style: Style,
    children: Vec<Box<dyn Widget>>,
    axis: Axis,

    // State
    disabled: bool,
    visible: bool,
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
        }
    }

    pub fn child(mut self, widget: Box<dyn Widget>) -> Self {
        self.children.push(widget);
        self
    }

    // ---------------------------------------------------------
    // Builder methods for each style property can be added here
    // ---------------------------------------------------------

    pub fn width(mut self, size: Size) -> Self {
        self.style = self.style.width(size);
        self
    }

    pub fn height(mut self, size: Size) -> Self {
        self.style = self.style.height(size);
        self
    }

    // ... delegate other style methods
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
        // Compute from children based on axis
        // (port your existing logic)
        Vec2::ZERO // Placeholder
    }

    fn ui(&mut self, ui: &mut Ui, rect: Rect) {
        if !self.visible {
            return;
        }

        // Apply margin, padding, background
        // Compute child rects using flexbox
        // Render each child with its rect
        // (port your existing render logic from WidgetStr)
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

/// The kind of content a widget holds.
enum WidgetKind {
    /// Container with child widgets.
    Container(Vec<WidgetStr>),
    /// Leaf with custom content.
    Leaf(Box<dyn WidgetContent>),
    /// Empty widget (placeholder).
    Empty,
}

impl Default for WidgetKind {
    fn default() -> Self {
        Self::Empty
    }
}

/// WidgetStr - the universal building block for UI layouts.
///
/// A WidgetStr can be a container (holding children) or a leaf (rendering custom content).
/// Layout follows CSS Flexbox principles with direction, sizing, alignment, and spacing.
pub struct WidgetStr {
    /// Unique identifier for this widget.
    id: String,

    // Layout
    axis: Axis,
    width: Size,
    height: Size,
    align: Align,
    justify: Justify,
    gap: f32,
    padding: Spacing,
    margin: Spacing,
    overflow: Overflow,

    // Appearance
    background: Option<ImageSurface>,
    border_radius: f32,

    // Content
    kind: WidgetKind,

    // State
    disabled: bool,
    visible: bool,
}

impl WidgetStr {
    // ========================================================================
    // Constructors
    // ========================================================================

    /// Create a new row widget (horizontal layout).
    pub fn row(id: impl Into<String>) -> Self {
        Self::new(id, Axis::Row)
    }

    /// Create a new column widget (vertical layout).
    pub fn column(id: impl Into<String>) -> Self {
        Self::new(id, Axis::Column)
    }

    /// Create a leaf widget with custom content.
    pub fn leaf(id: impl Into<String>, content: impl WidgetContent + 'static) -> Self {
        Self {
            id: id.into(),
            axis: Axis::Row,
            width: Size::Content,
            height: Size::Content,
            align: Align::default(),
            justify: Justify::default(),
            gap: 0.0,
            padding: Spacing::ZERO,
            margin: Spacing::ZERO,
            overflow: Overflow::default(),
            background: None,
            border_radius: 0.0,
            kind: WidgetKind::Leaf(Box::new(content)),
            disabled: false,
            visible: true,
        }
    }

    fn new(id: impl Into<String>, axis: Axis) -> Self {
        Self {
            id: id.into(),
            axis,
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
            kind: WidgetKind::Container(Vec::new()),
            disabled: false,
            visible: true,
        }
    }

    // ========================================================================
    // Builder Methods - Sizing
    // ========================================================================

    /// Set the width of this widget.
    pub fn width(mut self, size: Size) -> Self {
        self.width = size;
        self
    }

    /// Set the height of this widget.
    pub fn height(mut self, size: Size) -> Self {
        self.height = size;
        self
    }

    // ========================================================================
    // Builder Methods - Alignment & Distribution
    // ========================================================================

    /// Set the cross-axis alignment.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Set the main-axis distribution.
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    // ========================================================================
    // Builder Methods - Spacing
    // ========================================================================

    /// Set the gap between children.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set the inner padding.
    pub fn padding(mut self, padding: impl Into<Spacing>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the outer margin.
    pub fn margin(mut self, margin: impl Into<Spacing>) -> Self {
        self.margin = margin.into();
        self
    }

    /// Set the overflow behavior.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    // ========================================================================
    // Builder Methods - Appearance
    // ========================================================================

    /// Set the background surface.
    pub fn background(mut self, surface: ImageSurface) -> Self {
        self.background = Some(surface);
        self
    }

    /// Set the border radius.
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    // ========================================================================
    // Builder Methods - Children
    // ========================================================================

    /// Add a child widget.
    pub fn child(mut self, widget: WidgetStr) -> Self {
        if let WidgetKind::Container(ref mut children) = self.kind {
            children.push(widget);
        }
        self
    }

    /// Add multiple child widgets.
    pub fn children(mut self, widgets: impl IntoIterator<Item = WidgetStr>) -> Self {
        if let WidgetKind::Container(ref mut children) = self.kind {
            children.extend(widgets);
        }
        self
    }

    // ========================================================================
    // Builder Methods - State
    // ========================================================================

    /// Set whether this widget is disabled (blocks interaction).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set whether this widget is visible.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the widget ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Check if this widget is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Check if this widget is disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    // ========================================================================
    // Layout
    // ========================================================================

    /// Calculate the minimum size needed by this widget.
    pub fn min_size(&self) -> Vec2 {
        let content_size = match &self.kind {
            WidgetKind::Leaf(content) => content.min_size(),
            WidgetKind::Container(children) => {
                let mut size = Vec2::ZERO;
                for child in children {
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
                // Add gaps
                let gap_count = children.len().saturating_sub(1) as f32;
                match self.axis {
                    Axis::Row => size.x += self.gap * gap_count,
                    Axis::Column => size.y += self.gap * gap_count,
                }
                size
            }
            WidgetKind::Empty => Vec2::ZERO,
        };

        Vec2::new(
            content_size.x + self.padding.horizontal() + self.margin.horizontal(),
            content_size.y + self.padding.vertical() + self.margin.vertical(),
        )
    }

    // ========================================================================
    // Rendering
    // ========================================================================

    /// Render the widget into the available UI space.
    pub fn ui(&mut self, ui: &mut Ui) {
        // Use the intersection of available rect and max rect to respect constraints
        let available = ui.available_rect_before_wrap();
        let max_rect = ui.max_rect();
        let constrained = available.intersect(max_rect);
        self.ui_in_rect(ui, constrained);
    }

    /// Render the widget into a specific rect.
    pub fn ui_in_rect(&mut self, ui: &mut Ui, rect: Rect) {
        if !self.visible {
            return;
        }

        // Inherit parent's clip rect
        let parent_clip = ui.clip_rect();

        // Apply margin to get widget bounds
        let widget_rect = Rect::from_min_max(
            egui::pos2(rect.min.x + self.margin.left, rect.min.y + self.margin.top),
            egui::pos2(
                rect.max.x - self.margin.right,
                rect.max.y - self.margin.bottom,
            ),
        );

        // Calculate effective clip rect for this widget
        let effective_clip = widget_rect.intersect(parent_clip);

        // Skip rendering if completely clipped
        if effective_clip.width() <= 0.0 || effective_clip.height() <= 0.0 {
            return;
        }

        // Draw background (respects parent clip)
        if let Some(ref mut bg) = self.background {
            if self.border_radius > 0.0 {
                // TODO: Add rounded rect support to ImageSurface
                bg.paint_rect(ui, widget_rect);
            } else {
                bg.paint_rect(ui, widget_rect);
            }
        }

        // Apply padding to get content area
        let content_rect = Rect::from_min_max(
            egui::pos2(
                widget_rect.min.x + self.padding.left,
                widget_rect.min.y + self.padding.top,
            ),
            egui::pos2(
                widget_rect.max.x - self.padding.right,
                widget_rect.max.y - self.padding.bottom,
            ),
        );

        // Render content based on kind
        self.render_content(ui, content_rect);

        // Draw disabled overlay
        if self.disabled {
            let painter = ui.painter();
            painter.rect_filled(
                widget_rect,
                self.border_radius,
                Color32::from_black_alpha(128),
            );
        }
    }

    /// Helper to render content, avoiding borrow issues.
    fn render_content(&mut self, ui: &mut Ui, content_rect: Rect) {
        // Calculate layout before borrowing children
        let rects: Vec<Rect> = match &self.kind {
            WidgetKind::Container(children) if !children.is_empty() => {
                self.compute_child_rects(content_rect)
            }
            _ => Vec::new(),
        };

        let overflow = self.overflow;
        let disabled = self.disabled;

        match overflow {
            Overflow::Clip => {
                // Clip content to bounds - intersect with existing clip rect
                let existing_clip = ui.clip_rect();
                let clipped_rect = content_rect.intersect(existing_clip);

                // Create a child UI with the clipped rect applied
                // Using allocate_ui_at_rect ensures the UI is positioned correctly
                // and set_clip_rect on the child UI will be inherited by painters
                ui.allocate_ui_at_rect(content_rect, |ui| {
                    ui.set_clip_rect(clipped_rect);
                    self.render_content_inner(ui, content_rect, &rects, disabled);
                });
            }
            Overflow::Scroll | Overflow::Auto => {
                // Wrap in ScrollArea for scrollable content
                let auto_shrink = matches!(overflow, Overflow::Auto);
                egui::ScrollArea::both()
                    .auto_shrink([auto_shrink, auto_shrink])
                    .show(ui, |ui| {
                        self.render_content_inner(ui, content_rect, &rects, disabled);
                    });
            }
            Overflow::Visible => {
                // No clipping - content can overflow
                self.render_content_inner(ui, content_rect, &rects, disabled);
            }
        }
    }

    /// Inner content rendering logic.
    fn render_content_inner(
        &mut self,
        ui: &mut Ui,
        content_rect: Rect,
        rects: &[Rect],
        disabled: bool,
    ) {
        match &mut self.kind {
            WidgetKind::Container(children) => {
                for (child, child_rect) in children.iter_mut().zip(rects.iter()) {
                    child.ui_in_rect(ui, *child_rect);
                }
            }
            WidgetKind::Leaf(content) => {
                let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
                if disabled {
                    child_ui.disable();
                }
                content.ui(&mut child_ui);
            }
            WidgetKind::Empty => {}
        }
    }

    /// Compute child rects without borrowing children mutably.
    fn compute_child_rects(&self, content_rect: Rect) -> Vec<Rect> {
        let children = match &self.kind {
            WidgetKind::Container(c) => c,
            _ => return Vec::new(),
        };

        if children.is_empty() {
            return Vec::new();
        }

        let n = children.len();

        // Calculate sizes along main axis
        let (main_size, cross_size) = match self.axis {
            Axis::Row => (content_rect.width(), content_rect.height()),
            Axis::Column => (content_rect.height(), content_rect.width()),
        };

        // First pass: calculate fixed and content sizes, sum flex weights
        let mut fixed_total = 0.0f32;
        let mut flex_total = 0.0f32;
        let mut sizes: Vec<f32> = Vec::with_capacity(n);

        for child in children {
            let child_main_size = match self.axis {
                Axis::Row => child.width,
                Axis::Column => child.height,
            };

            match child_main_size {
                Size::Fixed(px) => {
                    fixed_total += px;
                    sizes.push(px);
                }
                Size::Content => {
                    let min = child.min_size();
                    let s = match self.axis {
                        Axis::Row => min.x,
                        Axis::Column => min.y,
                    };
                    fixed_total += s;
                    sizes.push(s);
                }
                Size::Flex(w) => {
                    flex_total += w;
                    sizes.push(-w); // Negative = flex weight placeholder
                }
            }
        }

        // Account for gaps
        let gap_total = self.gap * (n.saturating_sub(1)) as f32;
        let remaining = (main_size - fixed_total - gap_total).max(0.0);

        // Second pass: resolve flex sizes
        for size in &mut sizes {
            if *size < 0.0 {
                let weight = -(*size);
                *size = if flex_total > 0.0 {
                    remaining * (weight / flex_total)
                } else {
                    0.0
                };
            }
        }

        // Calculate positions based on justify
        let total_children_size: f32 = sizes.iter().sum();
        let total_with_gaps = total_children_size + gap_total;
        let extra_space = (main_size - total_with_gaps).max(0.0);

        let (start_offset, between_gap) = match self.justify {
            Justify::Start => (0.0, self.gap),
            Justify::Center => (extra_space / 2.0, self.gap),
            Justify::End => (extra_space, self.gap),
            Justify::SpaceBetween => {
                if n > 1 {
                    (0.0, self.gap + extra_space / (n - 1) as f32)
                } else {
                    (0.0, self.gap)
                }
            }
            Justify::SpaceAround => {
                let gap_space = extra_space / n as f32;
                (gap_space / 2.0, self.gap + gap_space)
            }
            Justify::SpaceEvenly => {
                let gap_space = extra_space / (n + 1) as f32;
                (gap_space, self.gap + gap_space)
            }
        };

        // Third pass: calculate final rects
        let mut rects = Vec::with_capacity(n);
        let mut pos = start_offset;

        for (i, child) in children.iter().enumerate() {
            let child_main = sizes[i];

            // Cross-axis size
            let child_cross_size = match self.axis {
                Axis::Row => child.height,
                Axis::Column => child.width,
            };

            let child_cross_natural = || {
                let min = child.min_size();
                match self.axis {
                    Axis::Row => min.y,
                    Axis::Column => min.x,
                }
            };

            let (child_cross_final, cross_offset) = match self.align {
                Align::Start => {
                    let s = match child_cross_size {
                        Size::Fixed(px) => px,
                        Size::Content => child_cross_natural(),
                        Size::Flex(_) => cross_size,
                    };
                    (s, 0.0)
                }
                Align::Center => {
                    let s = match child_cross_size {
                        Size::Fixed(px) => px,
                        Size::Content => child_cross_natural(),
                        Size::Flex(_) => cross_size,
                    };
                    (s, (cross_size - s) / 2.0)
                }
                Align::End => {
                    let s = match child_cross_size {
                        Size::Fixed(px) => px,
                        Size::Content => child_cross_natural(),
                        Size::Flex(_) => cross_size,
                    };
                    (s, cross_size - s)
                }
                Align::Stretch => (cross_size, 0.0),
            };

            // Build rect based on axis
            let rect = match self.axis {
                Axis::Row => Rect::from_min_size(
                    egui::pos2(content_rect.min.x + pos, content_rect.min.y + cross_offset),
                    egui::vec2(child_main, child_cross_final),
                ),
                Axis::Column => Rect::from_min_size(
                    egui::pos2(content_rect.min.x + cross_offset, content_rect.min.y + pos),
                    egui::vec2(child_cross_final, child_main),
                ),
            };

            rects.push(rect);
            pos += child_main + between_gap;
        }

        rects
    }
}

// ============================================================================
// Common WidgetContent Implementations
// ============================================================================

/// Simple label widget content.
pub struct Label {
    text: String,
    color: Option<Color32>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

impl WidgetContent for Label {
    fn ui(&mut self, ui: &mut Ui) {
        let text = if let Some(color) = self.color {
            egui::RichText::new(&self.text).color(color)
        } else {
            egui::RichText::new(&self.text)
        };
        let label = egui::Label::new(text).wrap_mode(egui::TextWrapMode::Truncate);
        ui.add(label);
    }

    fn min_size(&self) -> Vec2 {
        // Approximate size - actual size depends on font
        Vec2::new(self.text.len() as f32 * 8.0, 20.0)
    }
}

/// Spacer widget - flexible empty space.
pub struct Spacer;

impl WidgetContent for Spacer {
    fn ui(&mut self, _ui: &mut Ui) {
        // Nothing to render
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_constructors() {
        let s = Spacing::all(10.0);
        assert_eq!(s.top, 10.0);
        assert_eq!(s.right, 10.0);
        assert_eq!(s.bottom, 10.0);
        assert_eq!(s.left, 10.0);

        let s = Spacing::xy(20.0, 10.0);
        assert_eq!(s.top, 10.0);
        assert_eq!(s.right, 20.0);
        assert_eq!(s.bottom, 10.0);
        assert_eq!(s.left, 20.0);
    }

    #[test]
    fn test_widget_builder() {
        let widget = WidgetStr::row("test")
            .width(Size::Fixed(100.0))
            .height(Size::Flex(1.0))
            .padding(Spacing::all(8.0))
            .gap(4.0);

        assert_eq!(widget.id(), "test");
        assert_eq!(widget.width, Size::Fixed(100.0));
        assert_eq!(widget.gap, 4.0);
    }
}
