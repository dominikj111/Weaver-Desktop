# Egui Layering Control

## Order Hierarchy (lowest to highest)

1. `Order::Background`
2. `Order::PanelResizeLine`
3. `Order::Middle` (default for panels)
4. `Order::Foreground` (modals, popups)
5. `Order::Debug`
6. `Order::Tooltip` (highest)

## Granular Control Within Same Order

### Method 1: Area Creation Order

```rust
// Later Areas render on top within same Order
Area::new("first").order(Order::Foreground).show(ctx, |ui| { ... });
Area::new("second").order(Order::Foreground).show(ctx, |ui| { ... }); // On top
```

### Method 2: LayerId with Custom Id

```rust
use egui::{LayerId, Order, Id};

// Higher Id hash values may affect stacking (implementation detail)
let layer_id = LayerId::new(Order::Foreground, Id::new("my_high_priority"));
ctx.layer_painter(layer_id); // Pre-allocate layer

// Or with UiBuilder:
ui.scope_builder(
    UiBuilder::new().layer_id(LayerId::new(Order::Foreground, Id::new("priority"))),
    |ui| { /* content */ }
);
```

### Method 3: Manual Layer Management

```rust
// Move layer to front within its order
ctx.move_to_top(LayerId::new(Order::Foreground, Id::new("my_area")));
```

### Method 4: Separate Rendering Pass

```rust
// Render critical UI after all other UI in update()
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Regular UI
        CentralPanel::default().show(ctx, |ui| { ... });

        // High priority UI rendered last
        Area::new("overlay").order(Order::Foreground).show(ctx, |ui| { ... });
    }
}
```

## Best Practices

1. **Use appropriate Order levels** - Don't put everything at Tooltip level
2. **Render order matters** - Render high-priority elements last in update()
3. **Use Area for floating content** - Panels use fixed layers
4. **Consider user intent** - Tooltips should be on top, but transient
5. **Test interactions** - Ensure clickability and hover states work correctly

## Current Solution

We patched egui-toast to use `Order::Tooltip` because:

- Modals use `Order::Foreground`
- Toasts need to be visible above modals
- Toasts are user-dismissible so should be accessible

## Future Improvements

Consider submitting PR to egui-toast to add:

```rust
impl Toasts {
    pub fn order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }
}
```
