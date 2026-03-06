Looking at widget.rs, I can see you're close but have a structural mismatch: you have a WidgetContent trait but no Widget trait. The concrete Widget struct is doing everything, which prevents the composability you want.

Core Issue
Your Widget struct has an internal WidgetKind enum that switches between Container/Leaf/Empty. This creates the same privileged hierarchy problem—containers vs leaves are still special cases, just hidden inside one type.

Recommended Refactor Path
1. Invert the Design
Instead of:
Widget (concrete struct)
  └─ WidgetKind (enum)
       ├─ Container(Vec<Widget>)
       ├─ Leaf(Box<dyn WidgetContent>)
       └─ Empty
Do:
Widget (trait)
  ├─ Container (struct, implements Widget)
  ├─ Image (struct, implements Widget)
  ├─ Icon (struct, implements Widget)
  ├─ Grid (struct, implements Widget)
  └─ [any custom widget implements Widget]

2. Widget Trait Design

Keep it minimal but sufficient:

```rust
pub trait Widget {
    fn id(&self) -> &str;
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn min_size(&self) -> Vec2;
    fn ui(&mut self, ui: &mut Ui, rect: Rect);
    
    // Optional: mutable access to children (containers only)
    fn children_mut(&mut self) -> Option<&mut Vec<Box<dyn Widget>>> {
        None  // Leaf widgets return None
    }
}
```

**Key principle:** Style is a **field** on every widget, not a wrapper. This eliminates double-indirection and keeps the design simple.

---

3. Style Structure

Every widget owns its styling properties:

```rust
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
    
    // State
    disabled: bool,
    visible: bool,
}

impl Style {
    pub fn new() -> Self { /* ... */ }
    
    // Builder methods
    pub fn width(mut self, size: Size) -> Self { /* ... */ }
    pub fn padding(mut self, spacing: impl Into<Spacing>) -> Self { /* ... */ }
    // ... etc
}
```

---

4. Widget Implementations

Every widget has the same structure: `id`, `style`, and widget-specific fields.

### Container (widget with children)

```rust
pub struct Container {
    id: String,
    style: Style,
    children: Vec<Box<dyn Widget>>,
    axis: Axis,
}

impl Container {
    pub fn row(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            style: Style::new(),
            children: Vec::new(),
            axis: Axis::Row,
        }
    }
    
    pub fn column(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            style: Style::new(),
            children: Vec::new(),
            axis: Axis::Column,
        }
    }
    
    pub fn child(mut self, widget: Box<dyn Widget>) -> Self {
        self.children.push(widget);
        self
    }
    
    // Delegate style methods for builder pattern
    pub fn width(mut self, size: Size) -> Self {
        self.style = self.style.width(size);
        self
    }
    // ... other style delegations
}

impl Widget for Container {
    fn id(&self) -> &str { &self.id }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }
    fn min_size(&self) -> Vec2 { /* compute from children */ }
    
    fn ui(&mut self, ui: &mut Ui, rect: Rect) {
        let child_rects = self.compute_layout(rect);
        for (child, child_rect) in self.children.iter_mut().zip(child_rects) {
            child.ui(ui, child_rect);
        }
    }
    
    fn children_mut(&mut self) -> Option<&mut Vec<Box<dyn Widget>>> {
        Some(&mut self.children)
    }
}
```

### Image (leaf widget)

```rust
pub struct Image {
    id: String,
    style: Style,
    path: PathBuf,
    scale_mode: ScaleMode,
}

impl Image {
    pub fn new(id: impl Into<String>, path: PathBuf) -> Self {
        Self {
            id: id.into(),
            style: Style::new(),
            path,
            scale_mode: ScaleMode::Fit,
        }
    }
    
    pub fn width(mut self, size: Size) -> Self {
        self.style = self.style.width(size);
        self
    }
    // ... other delegations
}

impl Widget for Image {
    fn id(&self) -> &str { &self.id }
    fn style(&self) -> &Style { &self.style }
    fn style_mut(&mut self) -> &mut Style { &mut self.style }
    fn min_size(&self) -> Vec2 { Vec2::new(0.0, 0.0) }
    
    fn ui(&mut self, ui: &mut Ui, rect: Rect) {
        // Render image within rect
    }
    
    // No children - returns None (default)
}
```

---

5. Uniform Construction API

Every widget is constructed the same way—no special cases:

```rust
// Container with children
let desktop = Container::column("desktop")
    .width(Size::Flex(1.0))
    .padding(8.0)
    .child(Box::new(
        Image::new("background", bg_path)
            .width(Size::Flex(1.0))
            .height(Size::Flex(1.0))
    ))
    .child(Box::new(
        Container::row("icon-bar")
            .height(Size::Fixed(64.0))
            .gap(12.0)
            .child(Box::new(Icon::new("home")))
            .child(Box::new(Icon::new("settings")))
    ));

// Empty container (just renders background/style)
let panel = Container::column("panel")
    .width(Size::Fixed(200.0))
    .height(Size::Fixed(100.0))
    .background(ImageSurface::color(Color32::DARK_GRAY))
    .border_radius(8.0);
    // No children - just visual styling
```

**Key insight:** Empty containers are valid—they render background, padding, borders without content.

6. Migration Path

Since you want to keep current API working during transition:

1. **Keep current `WidgetStr` struct** temporarily (rename to avoid conflicts)
2. **Define new `Widget` trait** alongside existing code
3. **Define `Style` struct** with all layout/appearance properties
4. **Implement specific widget types** (Container, Image, Icon, Grid, etc.)
5. **Update construction sites** one by one to use new trait-based widgets
6. **Remove old `WidgetStr` struct and `WidgetKind` enum** once migration complete

This allows incremental refactoring without breaking existing code.

---

## Key Benefits

✅ **True uniformity** - Shell/Desktop/Panel/Icon all implement same trait  
✅ **No wrapper indirection** - Style is a field, not a wrapper type  
✅ **Separation of concerns** - Layout logic in `Style`, rendering in widget implementations  
✅ **Infinite nesting** - Can nest shells/containers arbitrarily deep  
✅ **Multiple visual representations** - Same content tree, different styles  
✅ **Testability** - Mock widgets just implement the trait  
✅ **Type safety** - Compiler enforces widget requirements  
✅ **Empty containers valid** - Render backgrounds/borders without children

---

## What Stays from Current Code

✅ All layout types (`Size`, `Align`, `Justify`, `Spacing`, `Axis`, `Overflow`)  
✅ Flexbox layout algorithm in `compute_child_rects()`  
✅ Builder pattern for configuration  
✅ The general rendering flow  
✅ Background rendering with `ImageSurface`

---

## What Changes

❌ `WidgetKind` enum → **eliminated**  
❌ `WidgetContent` trait → **becomes `Widget` trait** (broader scope)  
❌ `WidgetStr` struct → **becomes individual widget types** (Container, Image, etc.)  
❌ Special `leaf()` constructor → **all widgets constructed uniformly**  
❌ No `Styled` wrapper needed → **Style is a field on every widget**

---

## Why Style-as-Field Is Superior

**Old approach (wrapper pattern):**
```rust
Styled::new(Container::new("root"))  // Box<Box<dyn Widget>>
    .child(Styled::new(Image::new("bg")))
```

**New approach (style as field):**
```rust
Container::row("root")  // Box<dyn Widget>
    .child(Box::new(Image::new("bg")))
```

**Advantages:**
- Less indirection (one `Box` instead of two)
- More Rust-idiomatic (composition via fields, not wrappers)
- Simpler ownership model
- Direct mutation: `widget.style_mut().width = Size::Fixed(100.0)`
- Less boilerplate (one type per widget, not widget + wrapper)

---

## Implementation Notes

The refactor is **mechanical** once you commit to trait-based design. Your existing layout algorithm can be lifted nearly as-is into:

- `Style::compute_layout()` - for applying margins, padding, backgrounds
- `Container::compute_child_rects()` - for flexbox layout of children

Each widget type is responsible for:
1. Storing its own `Style` 
2. Computing its `min_size()`
3. Rendering itself in `ui(rect)`
4. Optionally managing children (containers only)