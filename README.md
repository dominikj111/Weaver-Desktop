# Weaver Desktop

> A 30MB desktop environment for Raspberry Pi and embedded Linux with built-in hardware control.

**Status:** 🚧 Early development - building towards MVP. Core infrastructure works, views in progress.

```bash
# Quick start (requires Rust toolchain)
git clone <repo>
cd DesktopWeaver
cargo run
```

*Philosophy: It has to build and run.*

---

## What is Weaver Desktop?

Weaver Desktop is a **pure GUI desktop environment** built in Rust/egui, designed to run on everything from Raspberry Pi Zero to consumer PCs. Unlike traditional monolithic desktop environments consuming 300-600MB RAM, Weaver Desktop targets <50MB footprint while remaining fully featured.

**Core Philosophy:** The DE is a thin GUI client. All system operations (package management, service control, hardware abstraction) are delegated to the **workmeshd** daemon, keeping the interface lightweight and the architecture clean.

## Key Features

### 🎨 Template-Driven Flexibility

Shape the entire desktop through configuration files (templates):

- **Visual theming** - Colors, backgrounds, fonts, component styles
- **Layout control** - Which components appear, where, and how they behave
- **Widget configuration** - App menus, shortcuts, widgets, calendars, quick actions
- **Use-case profiles** - Transform the same DE into a kiosk, media center, cyberdeck control panel, or traditional desktop

One DE, infinite configurations.

### 📱 Touch-First Design

Built for 7" touchscreens and embedded displays:

- Large touch targets, gesture-friendly navigation
- Virtual keyboard support
- Visual keyboard hints (press Super key to see shortcuts)
- Fullscreen-only external apps (maximizes screen real estate)

### 🔌 Hardware Control Integration

Direct GPIO/hardware control for cyberdecks and embedded projects:

- Digital/analog I/O pin states and toggles
- PWM channel control with sliders
- I2C device communication
- MCU integration (Tiny2040, Arduino via virtual COM port)
- Widget-based control panels or scripted automation
- Future: Ladder diagram logic for boolean control sequences

### 🌐 Thin Client Architecture

Operate locally or control remote machines transparently:

- **Local mode** - GUI + workmeshd on same machine
- **Remote mode** - GUI connects to remote workmeshd via TCP/IP, UDP, or WorkMesh P2P
- Switch targets seamlessly - same interface, different machine
- No explicit SSH per operation - session-based connection

### 🔧 Extensible

- **Plugin system** (planned) - Community-driven widgets, components, and automations
- **Command bus architecture** - Clean separation between UI and operations
- **Profile inheritance** - Base profiles with device-specific extensions

## Who Is This For?

| Audience | Why Weaver Desktop |
|----------|--------------------|
| **Cyberdeck builders** | Lightweight DE that won't drain battery, built-in GPIO control |
| **SBC hobbyists** | Touch-friendly, runs well on Pi Zero, no bloat |
| **Home lab enthusiasts** | Manage multiple machines from one interface |
| **Kiosk/signage developers** | Template-driven locked-down interfaces |
| **Maker spaces** | Profile-based workstations, easy reset between users |
| **Old hardware revivers** | Sub-50MB footprint for legacy laptops |

## Target Platforms

| Priority | Platform | Notes |
|----------|----------|-------|
| **Primary** | Raspberry Pi Zero W2, Pi 4/5 | First-class SBC support |
| **Primary** | 7" touchscreen displays | Touch-first optimization |
| **Secondary** | Legacy laptops (Acer Aspire One 725) | Low-resource x86 devices |
| **Secondary** | Consumer PCs | Full desktop replacement |
| **Future** | Android/iOS | Fullscreen app/launcher replacement |
| **Future** | Web browser | Via egui WASM support |
| **Future** | TV/media center | Large screen, remote-friendly layouts |

## Part of the WorkMesh Ecosystem

Weaver Desktop is one component of the larger **WorkMesh** project:

- **Weaver Desktop** - GUI desktop environment (this project)
- **workmeshd** - System management daemon (privileged operations)
- **WorkMesh SaaS** (future, optional) - Secure P2P connectivity for NAT traversal and fleet management

Together, these enable scenarios like:

- Managing a fleet of kiosks from a central location
- Controlling a cyberdeck's GPIO from your phone
- Headless automation across distributed devices

## Development Progress

Core infrastructure complete:

- ✅ Theming system
- ✅ Event/command bus
- ✅ Icon system (Numix, Papirus, Vimix themes)
- ✅ Reactive primitives (zero-allocation)
- ✅ Shell components (panels, menus, toasts)
- ✅ Embedded terminal (alacritty backend)

In progress:

- 🚧 View system and navigation
- 🚧 Dashboard with system status
- 🚧 Application launcher
- 🚧 workmeshd integration

See [docs/ARCHITECTURE_ROADMAP.md](docs/ARCHITECTURE_ROADMAP.md) for full development phases.

## Project Structure

### `crates/`

Workspace crates implementing core functionality:

- **`weaver_lib/`** - UI framework abstractions. Contains theming, event/command bus, icon management, reactive primitives, and reusable widgets. Built on egui with zero-allocation optimizations for SBC targets.

- **`weaver_desktop_shell/`** - Desktop shell implementation. Contains UI components, views (Dashboard, Hardware, Settings, etc.), and shell-specific logic.

### `assets/`

Static assets:

- **`icons/`** - Icon theme packs (Numix Circle, Papirus, Vimix installed in linux system as normal - not embedded)
- Background images (Baya weaver bird imagery, Various weave patterns, ...)

### `forks/`

Forked dependencies with custom modifications:

- **`egui-toast/`** - Toast notification library with local customizations

### `ai_tasks/`

Structured task definitions for AI-assisted code quality checks and optimization reviews. See [`ai_tasks/README.md`](ai_tasks/README.md).

## Documentation

| Document | Description |
|----------|-------------|
| [PROPOSAL.md](docs/PROPOSAL.md) | Technical specification and feature roadmap. Core capabilities including profile-based system management, hardware control, and architecture design. |
| [ARCHITECTURE_ROADMAP.md](docs/ARCHITECTURE_ROADMAP.md) | Phased development plan. Component status tracking and implementation priorities from MVP to advanced features. |
| [DESKTOP_COMPONENTS.md](docs/DESKTOP_COMPONENTS.md) | Complete component inventory. All planned UI components, settings views, utilities, games, and their status. |
| [TODO.md](docs/TODO.md) | Current task backlog with UI mockups and implementation details. |
| [MULTI_TARGET_ARCHITECTURE.md](docs/MULTI_TARGET_ARCHITECTURE.md) | Thin client architecture. How the DE controls local or remote machines transparently. |
| [STRATEGIC_ANALYSIS.md](docs/STRATEGIC_ANALYSIS.md) | Market positioning and competitive landscape analysis. |
| [BUSINESS_STRATEGY.md](docs/BUSINESS_STRATEGY.md) | Monetization plan and business model. |
| [GO_TO_MARKET_STRATEGY.md](docs/GO_TO_MARKET_STRATEGY.md) | Launch strategy, target segments, and pricing. |

## Branding

- **Name:** Weaver Desktop (or just "Weaver")
- **Mascot:** Weave pattern - procedural, animated, code-driven graphics
- **Philosophy:** Lightweight, adaptive, purposeful

### Why a Weave Pattern?

The mascot isn't a static image - it's a procedural pattern rendered in code. This reflects the project's DNA:

- **Programmable** - The pattern can animate, respond to state, visualize activity
- **Flexible** - Same underlying structure, infinite visual variations
- **Lightweight** - Generated, not loaded from heavy assets
- **Symbolic** - Weaving threads together = integrating components into a cohesive whole

The Baya weaver bird imagery remains as background art, but the weave pattern is the true identity - something that can only exist because this is a code-first project.

---

## Why Not Just Use XFCE/LXDE/i3?

| Concern | Traditional DEs | Weaver Desktop |
|---------|-----------------|----------------|
| **Memory** | 300-600MB (XFCE/LXDE) | <50MB target |
| **Touch support** | Bolted-on, awkward | Touch-first design |
| **Hardware control** | None - need separate tools | GPIO/PWM/I2C built-in |
| **Remote management** | SSH + manual commands | Unified thin-client architecture |
| **Adaptability** | Theme colors only | Template-driven: layouts, widgets, components |
| **Embedded use** | Overkill for kiosks | Purpose-built for SBCs and kiosks |

Lightweight window managers (i3, sway) are keyboard-focused and developer-oriented. Full DEs (GNOME, KDE, XFCE) are heavy. Touch-first embedded DEs with hardware control? That space is empty.

---

## One DE, Many Faces

Weaver Desktop isn't just themeable - it's **reshapable**. The same binary, same codebase, becomes:

| Template | Use Case |
|----------|----------|
| **Desktop** | Traditional layout with panels, app grid, widgets |
| **Kiosk** | Single-purpose locked interface (coffee shop POS, museum display) |
| **Cyberdeck** | Hardware control panel with GPIO widgets, system status |
| **Media Center** | Large buttons, remote-friendly, media controls |
| **Control Panel** | Sidebar navigation, dashboard-style monitoring |

Templates define not just colors, but which components exist, where they're placed, what actions they expose. One DE for all your devices - each looking exactly right for its purpose.

---

## MVP Focus

The minimum viable product to demonstrate value:

| Feature | Why Critical |
|---------|--------------|
| ✅ Desktop shell (bars, menu) | Foundation - done |
| ✅ Theming system | Visual identity - done |
| 🎯 **App launcher** | Core DE functionality |
| 🎯 **GPIO/Hardware widget** | Unique differentiator - no other DE has this |
| 🎯 **System status dashboard** | Proves lightweight operation |
| 🎯 **Power menu** | Essential for standalone use |
| 🎯 **Profile loading** | Core value proposition |
| 🎯 **2-3 template layouts** | Demonstrates "many faces" capability |

**Post-MVP:** Settings panels, file manager, cloud sync, widget system, advanced theming editor.

See [docs/GO_TO_MARKET_STRATEGY.md](docs/GO_TO_MARKET_STRATEGY.md) for full launch strategy.

---

## The Bigger Picture: Remote Management

**Why use Weaver Desktop to manage servers?**

You don't need a cloud account. You don't need a SaaS subscription. Weaver Desktop + workmeshd works fully offline:

```
┌─────────────────────────────────────────────────────────────┐
│  Your Laptop                                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Weaver Desktop (GUI)                               │    │
│  │  • Renders locally                                  │    │
│  │  • Sends commands to active target                  │    │
│  │  • Target selector: [Local] [Server] [Pi Cluster]   │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
   ┌──────────┐    ┌──────────┐    ┌──────────┐
   │ workmeshd│    │ workmeshd│    │ workmeshd│
   │ (local)  │    │ (server) │    │ (Pi)     │
   └──────────┘    └──────────┘    └──────────┘
```

- **Local LAN:** Direct TCP/IP to any workmeshd on your network
- **Same interface:** Whether local or remote, the experience is identical
- **No cloud required:** Fully functional without any account or subscription

**Future option (WorkMesh SaaS):** For users who want secure P2P connectivity across NAT boundaries, fleet management dashboards, or managed device orchestration - an optional cloud service. But the core remains open source and self-hostable.

This is infrastructure software. It should work without permission from a server.

---

*Part of the WorkMesh ecosystem*
