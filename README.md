# Weaver Desktop

> A lightweight desktop environment for embedded Linux and SBCs. Thin UI client — delegates all system operations to [workmeshd](https://github.com/dominikj111/workmeshd) daemon.

**Status:** 🚧 Not a fully polished MVP yet — active development and portfolio piece.

**Branch Status:**

- ✅ `main` — stable, builds
- 🚧 `feature/ux-ui-flex-layouting-app-design` — widget system refactoring, currently build errors

```bash
git clone --recursive https://github.com/dominikj111/DesktopWeaver.git
cd DesktopWeaver
cargo run
```

> `--recursive` is required to fetch submodules. Icons are optional — app uses emoji fallbacks.

---

## What It Is

Weaver is a **pure GUI desktop environment** built in Rust/egui, targeting Raspberry Pi Zero, cyberdecks, kiosks, and resource-constrained embedded Linux. Target footprint: <50MB.

The UI never performs privileged operations directly — it delegates everything to **workmeshd**, a system daemon handling hardware control, service management, and device abstraction. This keeps the interface lightweight and the architecture clean.

**Key idea:** Weaver doesn't know about GPIO pins or system services — it knows about *devices* and *panels*. A 230V relay becomes "Desk Socket", an I2C sensor becomes "Current: 2.4A". Domain-level control, not electrical primitives.

The same binary reshapes into a traditional desktop, kiosk, cyberdeck control panel, or industrial HMI — driven by configuration templates, not code changes.

## Origin

Built from a concrete need: a trusted, offline-first control interface for distributed solar power installations. Field hardware, high-voltage relays, minimal compute. The constraint made the architecture.

## Current Development Focus

- **Widget composition** — trait-based, flexible layout system
- **State management** — efficient reactive updates
- **Minimal rendering** — lazy layout computation, on-demand updates for ARM targets

See [CURRENT_STATE.md](CURRENT_STATE.md) for active branch details and [docs/](docs/) for full technical documentation.

## License

MIT — see [LICENSE](LICENSE)
