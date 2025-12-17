# Weaver Desktop

A lightweight, template-driven desktop environment for resource-constrained systems.

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
- **WorkMesh SaaS** (future) - Secure P2P connectivity between devices, remote control, automation, and headless fleet management

Together, these enable scenarios like:

- Managing a fleet of kiosks from a central location
- Controlling a cyberdeck's GPIO from your phone
- Headless automation across distributed devices

## Status

**Early development** - Private repository

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

---

*Part of the WorkMesh ecosystem*
