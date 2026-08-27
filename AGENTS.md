# Weaver Desktop Agent Guide

Essential guide for agents working on Weaver Desktop, a lightweight Rust/egui desktop environment targeting resource-constrained ARM systems.

Think before coding. State your assumptions out loud. If the request is ambiguous, ask. If a simpler approach exists, push back. Stop when you are confused, name what is unclear, do not just pick one interpretation and run.

Simplicity first. Write the minimum code that solves the problem. No speculative abstractions. No flexibility nobody asked for. The test: would a senior engineer call this overcomplicated.

Surgical changes. Touch only what the task requires. Do not improve neighboring code. Do not refactor what is not broken. Every changed line should trace back to the request.

Goal-driven execution. Turn vague instructions into verifiable targets before writing a line. “Add validation” becomes “write tests for invalid inputs, then make them pass.”

## Project Vision & Constraints

- **Targets**: Raspberry Pi Zero, SBCs, cyberdecks, kiosks.
- **Goal**: A pure GUI shell that delegates privileged operations to a system daemon.
- **Hardware Integration**: Domain-level control (e.g., "Desk Socket") rather than electrical primitives (GPIO).
- **Resources**: 256MB-512MB RAM, limited single/quad-core ARM.
- **Footprint**: <50MB final app size (including assets).
- **Optimization**: Prefer stack over heap (`ArrayString`, `arrayvec`), static function pointers over closures, cached values over per-frame work.
- **Offline-First**: Reliable operation without network.

## Core Commands

| Action | Command |
|--------|---------|
| Build | `cargo build` |
| Run (Shell) | `cargo run` |
| Test | `cargo test` |
| Lint | `cargo clippy` |
| Release | `cargo build --release` |

## Project Structure

- `crates/weaver_lib`: Core UI framework (reactive primitives, theme, icons, commands).
- `crates/weaver_desktop_shell`: Desktop UI components and the `Widget`/`Container` layout engine.
- `src/`: Main entry point and orchestration.
- `docs/`: In-depth documentation on roadmap, components, and design.

## Key Architectures

### 1. Unified Data Flow (Command Bus)
UI emits commands; state is updated *after* rendering to satisfy Rust borrow rules.
- `CommandBus<AppCommand>`: Main UI-to-State bridge.
- `ExternalReceiver`: Thread-safe bridge for daemon/network commands.
- Pattern: `bus.dispatch(cmd)` in UI → `bus.drain(|cmd| ...)` in `update()`.

### 2. Widget System (`Widget` trait + `Container`) — design intent
A CSS Flexbox-inspired, React-inspired widget model. The design goal is a **thin contract**
between application and UI: `[ application backend ] <-> [ thin contract ] <-> [ ui toolkit ]`.
The contract owns application state + typed event channel + per-backend renderer. See
`docs/WIDGET_FABRIC_DESIGN.md` for the full design.
- **State lives in the widget object, never in egui objects** — the widget tree persists
  across frames; rendering is a function of state.
- **`render()` calls backend utilities** (React-inspired) — egui today, GTK later; per-backend.
- **Dispatch = event objects** (local/global) — no per-frame lambda callbacks. The
  `CommandBus<AppCommand>` (§1) is the event-channel prototype.
- **Composition**: layout widgets + presentational widgets; flexbox layout (`Axis`, `Size`,
  `Align`, `Justify`, `Overflow`, `Spacing`, `CachedLayout` caching).

### 3. Reactive Primitives (`weaver_lib::reactive`)
Designed for zero-allocation event handling.
- `SignalFn<T>`: Wraps an `fn(&T)` pointer. No heap allocation, no captures.
- `Observable<T>`: Value wrapper that notifies subscribers (Signals) when changed.
- `Interactable<T>`: Utility for tracking clicks/presses/releases on any UI element.

### 4. Theme & Icons
- `Theme`: Maps `egui::Visuals` to semantic tokens (`colors.accent`, `spacing.padding`).
- `IconTheme`: Freedesktop-compatible icon lookup by name/size.

## Development Patterns & Gotchas

### UI Contexts
- **Static Handlers**: Use function pointers (`fn(&T)`) for callbacks to avoid closure allocation.
- **Component Trait**: Use for top-level panels needing full `&Context` access.
- **Widget trait**: Use for atomic elements rendering into a `&mut Ui` (old `WidgetContent`).

### Memory & Performance
- **Zero-Allocation Logging**: Use `thread_local` format buffers (see `DATETIME_BUF` in `shell/mod.rs`).
- **Icon Loading**: Cache `TextureHandle` within widgets; don't reload icons per frame.
- **Desktop Strategy**: The same binary reshapes via config templates into different modes: Desktop, Kiosk, Cyberdeck, or Industrial HMI.

### Layout Gotchas
- The `Widget`/`Container` engine is hand-rolled. Changes to layout logic require thorough testing of child recursion and space distribution.
- Overflow defaults to `Clip`. Use `Visible` only when absolutely necessary.

## Documentation Index for Agents
- `docs/WIDGET_FABRIC_DESIGN.md`: **The design doc** — widget model, thin contract architecture, trajectories (egui/GTK/web/remote). Read before touching the widget system.
- `docs/UI_FABRIC_PROPOSAL.md`: Socket-driven remote UI runtime (external processes declare UI; semantic events; workmeshd executes actions).
- `docs/MULTI_TARGET_ARCHITECTURE.md`: Remote control of multiple machines via workmeshd.
- `docs/ARCHITECTURE_ROADMAP.md`: Current implementation status and phase-by-phase goals.
- `docs/WIDGET_REFACTORING_DESIGN.md`: Historical plan for the widget refactor (its incremental approach was superseded by the trait-based implementation).
- `docs/TODO.md`: Detailed feature backlog (Hardware view, Dashboard widgets, App launcher).
- `docs/THEME_ARCHITECTURE.md`: Vision for the semantic token system.
- `docs/DESKTOP_COMPONENTS.md`: UI specifications and keyboard navigation hints system.

## MWP Protocol

@.mwp/protocol.md

At every session start:

1. Run `bash .mwp/changes.sh`
2. Read `.mwp/topology.md`
3. Read `.mwp/discoveries.md` (if it exists)

When a target is known:

- Run `bash .mwp/concat-context.sh <target>` ONCE
- Update `.mwp/discoveries.md` with new findings
