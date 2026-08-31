# Weaver Desktop

> A lightweight desktop environment for embedded Linux and SBCs. Thin UI client — delegates all system operations to system daemon.

**Status:** 🚧 Not a fully polished MVP yet — active development and portfolio piece.

**Branch Status:**

- ✅ `main` — stable, builds
- 🚧 `feature/ux-ui-flex-layouting-app-design` — widget system refactoring, currently build errors

```bash
git clone --recursive https://github.com/dominikj111/Weaver-Desktop.git
cd Weaver-Desktop
cargo run
```

> `--recursive` is required to fetch submodules. Icons are optional — app uses emoji fallbacks.

---

## What It Is

Weaver is a **pure GUI desktop environment** built in Rust/egui, targeting Raspberry Pi Zero, cyberdecks, kiosks, and resource-constrained embedded Linux. Target footprint: <50MB.

The UI never performs privileged operations directly — it delegates everything to a system daemon handling hardware control, service management, and device abstraction. This keeps the interface lightweight and the architecture clean.

**Key idea:** Weaver doesn't know about GPIO pins or system services — it knows about *devices* and *panels*. A 230V relay becomes "Desk Socket", an I2C sensor becomes "Current: 2.4A". Domain-level control, not electrical primitives.

The same binary reshapes into a traditional desktop, kiosk, cyberdeck control panel, or industrial HMI — driven by configuration templates, not code changes.

## Architecture

Weaver's widget system is built around a **thin contract** between application and UI:

```
[ application backend ] <-> [ thin contract ] <-> [ ui toolkit ]
```

The contract owns **application state + a typed event channel + a per-backend renderer** —
widgets hold UI state and `render()` into the toolkit (React-inspired), dispatch happens via
event objects (no per-frame lambdas), and the UI toolkit is replaceable (egui today, GTK for
HoverClock, web runtime later).

Design details: [docs/WIDGET_FABRIC_DESIGN.md](docs/WIDGET_FABRIC_DESIGN.md) ·
[docs/UI_FABRIC_PROPOSAL.md](docs/UI_FABRIC_PROPOSAL.md) (socket-driven remote UI) ·
[docs/MULTI_TARGET_ARCHITECTURE.md](docs/MULTI_TARGET_ARCHITECTURE.md) (remote control via workmeshd).

## Origin

Built from a concrete need: a trusted, offline-first control interface for distributed solar power installations. Field hardware, high-voltage relays, minimal compute. The constraint made the architecture.

## Current Development Focus

- **Widget system refactoring** — trait-based `Widget` + `Container` (flexbox) with layout caching; in flight on `feature/ux-ui-flex-layouting-app-design` (Story 01: land to green build)
- **Thin contract / widget fabric** — the widget model becomes the renderer-neutral core (Story 02: `crates/weaver_fabric`), enabling toolkit swap and contractual rendering across the network
- **State management** — state out of the rendering pipeline, reactive updates for ARM targets

## Family: the shared-backend vision (recorded 2026-08-31)

Weaver and **unfold** (`/development/unfold/`, private) are one family: a **shared
headless-capable backend**, two renderers.

- The backend daemon owns state and exposes the capability surface; everything works
  without a GUI — AI integration, SaaS tools, remote control, notifications, logs.
- **Unfold renders GTK** (X11-first desktop shell); **Weaver renders egui** (pure GUI,
  <50 MB, Pi Zero / cyberdeck / kiosk / embedded / HMI). Same principles: thin
  contract, daemon-owns-state, domain-level control, config-driven reshaping.
- **Headless direction:** headless PCs and servers — full system without a display,
  controlled remotely.
- Weaver's widget architecture (the thin contract / widget fabric) is the piece that
  makes the shared backend possible — being designed now, first release = free visual
  reshaping, end target kiosks.
- Family map: unfold + Weaver → WorkFlows distro / workmeshd / moasis vision.

## License

MIT — see [LICENSE](LICENSE)
