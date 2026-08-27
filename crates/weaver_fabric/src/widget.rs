//! The widget model — renderer-neutral.
//!
//! `Widget` is the neutral model a widget exposes: identity, style, minimum size,
//! layout computation, and visibility/disabled state. It has **no render method** —
//! rendering is the per-backend adapter's job (the fabric stays toolkit-agnostic).
//!
//! `Container` is the flexbox composition node: it owns children, computes their
//! rects (pure geometry), and caches the layout so it is not recomputed unless the
//! rect, style, or children change.

use crate::layout::{Align, Axis, Justify, Overflow, Size, Spacing};
use crate::{pos2, vec2, Rect, Vec2};

/// A widget in the tree. State lives here; rendering is a pure function of state
/// (per-backend). Leaf widgets implement the layout/identity methods and leave the
/// default state methods; containers override where they hold state.
pub trait Widget {
    fn id(&self) -> &str;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn min_size(&self) -> Vec2;

    /// Compute layout for the given rect. Called before rendering.
    /// Containers recursively compute layout for their children.
    /// Leaf widgets (no children) have nothing to compute — default is a no-op.
    fn compute_layout(&mut self, _rect: Rect) {}

    // State — defaults suit leaf widgets; containers override where they hold state.
    fn is_visible(&self) -> bool {
        true
    }
    fn is_disabled(&self) -> bool {
        false
    }
    fn set_visible(&mut self, _visible: bool) {}
    fn set_disabled(&mut self, _disabled: bool) {}
}

/// Layout and visual properties of a widget.
///
/// Visual surface (background color/image) is a render concern and lives in the
/// backend adapter, not here — the fabric carries geometry + border radius only.
#[derive(Debug, Clone)]
pub struct Style {
    pub width: Size,
    pub height: Size,
    pub align: Align,
    pub justify: Justify,
    pub gap: f32,
    pub padding: Spacing,
    pub margin: Spacing,
    pub overflow: Overflow,
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
            border_radius: 0.0,
        }
    }
}

/// Cached layout computation results.
#[derive(Debug, Clone)]
pub struct CachedLayout {
    /// The rect this layout was computed for.
    pub for_rect: Rect,
    /// Content rect after margin and padding.
    pub content_rect: Rect,
    /// Computed rects for each child.
    pub child_rects: Vec<Rect>,
}

/// A flexbox container: owns children and lays them out along an axis.
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

    /// Add a child widget with explicit width/height sizes (sets the child's style).
    pub fn child_sized(
        mut self,
        mut widget: Box<dyn Widget>,
        width: Size,
        height: Size,
    ) -> Self {
        widget.style_mut().width = width;
        widget.style_mut().height = height;
        self.children.push(widget);
        self.layout_dirty = true;
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

    pub fn border_radius(mut self, radius: f32) -> Self {
        self.style.border_radius = radius;
        self
    }

    /// Access the children (for adapters to walk and render).
    pub fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    /// Access the cached layout (valid after `compute_layout`).
    pub fn cached_layout(&self) -> Option<&CachedLayout> {
        self.cached_layout.as_ref()
    }

    pub fn axis(&self) -> Axis {
        self.axis
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

    /// Compute child rects using the flexbox layout algorithm.
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
                    pos2(content_rect.min.x + pos, content_rect.min.y + cross_offset),
                    vec2(main_child_size, aligned_cross_size),
                ),
                Axis::Column => Rect::from_min_size(
                    pos2(content_rect.min.x + cross_offset, content_rect.min.y + pos),
                    vec2(aligned_cross_size, main_child_size),
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

    fn compute_layout(&mut self, rect: Rect) {
        // Apply margin to get widget bounds
        let widget_rect = Rect::from_min_max(
            pos2(
                rect.min.x + self.style.margin.left,
                rect.min.y + self.style.margin.top,
            ),
            pos2(
                rect.max.x - self.style.margin.right,
                rect.max.y - self.style.margin.bottom,
            ),
        );

        // Apply padding to get content area
        let content_rect = Rect::from_min_max(
            pos2(
                widget_rect.min.x + self.style.padding.left,
                widget_rect.min.y + self.style.padding.top,
            ),
            pos2(
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal leaf for testing layout.
    struct Leaf {
        id: String,
        style: Style,
    }

    impl Leaf {
        fn fixed(id: &str, w: f32, h: f32) -> Self {
            let mut style = Style::new();
            style.width = Size::Fixed(w);
            style.height = Size::Fixed(h);
            Self {
                id: id.to_string(),
                style,
            }
        }
    }

    impl Widget for Leaf {
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
            vec2(10.0, 10.0)
        }
    }

    #[test]
    fn row_layout_distributes_fixed_and_flex() {
        let mut row = Container::row("r");
        row = row
            .child(Box::new(Leaf::fixed("a", 20.0, 10.0)))
            .child_sized(Box::new(Leaf::fixed("b", 0.0, 10.0)), Size::Flex(1.0), Size::Fixed(10.0));

        row.compute_layout(Rect::from_min_size(pos2(0.0, 0.0), vec2(100.0, 50.0)));

        let layout = row.cached_layout().expect("layout computed");
        assert_eq!(layout.child_rects.len(), 2);
        // First child: fixed 20 wide.
        assert_eq!(layout.child_rects[0].width(), 20.0);
        // Second child: flex takes remaining 80 (100 - 20).
        assert_eq!(layout.child_rects[1].width(), 80.0);
        // Cross-axis stretch to 50.
        assert_eq!(layout.child_rects[1].height(), 50.0);
    }

    #[test]
    fn layout_cache_is_reused_when_rect_unchanged() {
        let mut col = Container::column("c").child(Box::new(Leaf::fixed("a", 10.0, 10.0)));
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(50.0, 50.0));

        col.compute_layout(rect);
        assert!(!col.needs_layout(rect)); // same rect, not dirty -> cached

        col.invalidate_layout();
        assert!(col.needs_layout(rect)); // dirty again

        let other = Rect::from_min_size(pos2(0.0, 0.0), vec2(60.0, 60.0));
        assert!(col.needs_layout(other)); // rect changed
    }

    #[test]
    fn justify_center_offsets_children() {
        let mut row = Container::row("r")
            .justify(Justify::Center)
            .child(Box::new(Leaf::fixed("a", 20.0, 10.0)));
        row.compute_layout(Rect::from_min_size(pos2(0.0, 0.0), vec2(100.0, 50.0)));

        let layout = row.cached_layout().unwrap();
        // Centered: (100 - 20) / 2 = 40 offset on x.
        assert_eq!(layout.child_rects[0].min.x, 40.0);
    }
}
