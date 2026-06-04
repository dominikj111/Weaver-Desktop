# Task 01: Fix Build Errors and Initialize Leaf Widgets

## Focus
Resolve immediate compiler errors caused by the removal of `WidgetStr` and `WidgetContent` in favor of the `Widget` trait. This involves updating imports and providing placeholder or initial implementations for missing leaf widgets like `Label` and `Spacer`.

## Steps
1. **Fix Imports**: Update `crates/weaver_desktop_shell/src/components/mod.rs` and other files to import `Widget` instead of `WidgetStr`/`WidgetContent`.
2. **Implement Label**: Create a basic `Label` struct that implements the `Widget` trait.
3. **Implement Spacer**: Create a basic `Spacer` struct that implements the `Widget` trait.
4. **Update Component Exports**: Ensure all new widget types are correctly exported in `components/mod.rs`.
5. **Resolve DesktopShell Errors**: Begin adapting `desktop_shell.rs` to the new API (may require multiple steps).

## Next
Once the basic types are in place and imports are fixed, we will move to Task 02 to fully implement the rendering logic for these new widgets.
