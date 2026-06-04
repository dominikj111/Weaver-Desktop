# Story 01: Widget System Refactoring (Trait-based Composition)

## Overview
This story covers the completion of the major architectural refactoring of the widget system. We are moving from a monolithic `WidgetStr` (enum/macro-based) to a polymorphic, trait-based composition model using a `Widget` trait and a `Container` struct with layout caching.

This refactoring is critical for resource-constrained targets (256MB-512MB RAM) as it reduces redundant layout calculations and prevents visual flickering during UI updates.

## Current State
- `Widget` trait is defined in `crates/weaver_desktop_shell/src/components/widget.rs`.
- `Container` struct is implemented with standard flexbox-like properties.
- `Style` struct extracts visual and layout properties.
- `CachedLayout` and `layout_dirty` mechanism is implemented but not fully integrated into all components.
- **Build is currently broken** because dependent components still expect `WidgetStr`, `Label`, etc.

## Objectives
1. Fix all build errors by migrating dependent components to the new `Widget` trait.
2. Complete the implementation of basic leaf widgets (Label, Spacer, Icon, Image).
3. Ensure layout caching is working correctly across the tree.
4. Remove legacy `WidgetStr` and related temporary files.

## Definition of Done
- [ ] Code compiles without errors or warnings.
- [ ] Desktop shell renders correctly using the new trait-based system.
- [ ] No visual flickering during layout updates.
- [ ] `old_widget_str.rs` is deleted.
- [ ] `temporals/` directory is deleted.
- [ ] `docs/WIDGET_REFACTORING_DESIGN.md` is updated or archived as complete.

## Sub-tasks
1. [ ] **Task 01**: Fix build errors in `crates/weaver_desktop_shell/src/components/`.
2. [ ] **Task 02**: Implement basic leaf widgets (`Label`, `Spacer`, `IconButton`) as structs implementing `Widget`.
3. [ ] **Task 03**: Update `DesktopShell` to use the new `Container` and `Widget` trait.
4. [ ] **Task 04**: Verify layout caching and fix any flickering or layout issues.
5. [ ] **Task 05**: Cleanup: remove `old_widget_str.rs`, `temporals/`, and update documentation.
