//! Layout primitives — renderer-neutral flexbox model.

/// Layout axis — how children are arranged.
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
