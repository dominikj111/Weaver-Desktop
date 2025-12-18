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

## Origin Story

Weaver Desktop emerged from a concrete need: **secure, trusted control of distributed solar power grids**.

A colleague building solar installations needed a way to monitor and control physical infrastructure—switches, relays, voltage rails—that was:

- **Trustworthy** — clear authority, predictable control, auditable actions
- **Offline-first** — works without internet or central servers  
- **Field-capable** — runs on minimal hardware in harsh environments
- **Safety-conscious** — high-voltage control requires deliberate, visible interactions

This isn't "IoT dashboard with a UI" — it's a **human-machine interface for distributed physical systems**.

The development platform became a cyberdeck: a Raspberry Pi Zero with touchscreen, controlling GPIO to switch relays for 3.3V / 5V / 12V / 240V power rails. Perfect for live demonstrations, perfect for validating the architecture under real constraints.

**The key insight**: Weaver doesn't control "GPIO pins" — it controls **devices** through semantically meaningful **panels**. The UI never knows "GPIO 17" — it knows "230V Desk Socket is ON and drawing 120W".

This reframing changes everything:

- **Panels, not primitives** — domain-level control surfaces, not electrical debugging
- **Safety by design** — dangerous operations look dangerous, require confirmation
- **Audit by default** — every action logged with operator, timestamp, result
- **Presets for workflows** — "All Off", "Presentation Mode", "Power Cycle"

Weaver Desktop isn't competing with XFCE or GNOME. It's creating a new category: **device-oriented desktop environments** for cyberdecks, kiosks, field terminals, and industrial control surfaces.

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

**Panels, Not Primitives**

Weaver presents hardware through domain-level control surfaces, not electrical primitives:

| ❌ What you won't see | ✅ What you will see |
|-----------------------|----------------------|
| GPIO 17 | 🔌 Desk Power Socket (230V) |
| PWM Channel 2 | 💡 LED Strip Brightness |
| I2C 0x40 | 📊 Current Sensor: 2.4A |
| GPIO 22, 23, 24 | ⚡ 12V Rail Status |

Each **panel** is a semantic control surface that:

- **Owns multiple pins/devices** — abstracted behind a meaningful name
- **Enforces safety logic** — dangerous operations require confirmation, long-press, or arming
- **Provides live feedback** — status, duration, load, error states
- **Supports presets** — "All Off", "Presentation Mode", "Power Cycle"

**Example panel for a 230V relay:**

```
┌─────────────────────────────────┐
│  🔌 Desk Socket                 │
│  Status: ON                     │
│  Voltage: 230V                  │
│  Load: ~120W                    │
│  Enabled for: 12m 04s           │
│                                 │
│  ⚠ [ HOLD 2s TO DISABLE ]       │
└─────────────────────────────────┘
```

**Why this matters:** Industrial control systems and field devices need clarity and trust, not pin debuggers. When someone asks "Can Weaver control GPIO?" — the answer is: *"No. Weaver controls devices."*

**Capabilities include:**

- Digital/analog I/O through named panels
- PWM control with visual sliders and live feedback
- I2C device communication with sensor widgets
- MCU integration (Tiny2040, Arduino via virtual COM port)
- Preset-based automation (one tap → multiple device state changes)
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
| **Cloud app developers** | UI Fabric: Cloud logic with local rendering |
| **AI integration builders** | Safe, governed AI-driven interfaces |

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

### `ai_scripts/`

Structured task definitions for AI-assisted code quality checks and optimization reviews. See [`ai_scripts/README.md`](ai_scripts/README.md).

## Documentation

| Document | Description |
|----------|-------------|
| [PROPOSAL.md](docs/PROPOSAL.md) | Technical specification and feature roadmap. Core capabilities including profile-based system management, hardware control, and architecture design. |
| [UI_FABRIC_PROPOSAL.md](docs/UI_FABRIC_PROPOSAL.md) | Socket-driven UI runtime. External processes declare UI, Weaver renders and governs. Enables cloud apps, AI interfaces, dynamic dashboards. |
| [USE_CASES.md](docs/USE_CASES.md) | Reference use cases driving design decisions. Solar grid control, home automation hub, robot control node, media center, kiosk deployments. |
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

## Future: UI Fabric (Socket-Driven UI)

Beyond templates, Weaver evolves into a **UI fabric** — a runtime where external processes can declare user interfaces through sockets, and Weaver renders them inside governed containers.

```
External Process ──► UI Declaration (JSON)
         Weaver ──► Validate & Render
           User ──► Interaction
         Weaver ──► Semantic Event
External Process ──► State Update
```

**Key principles:**

- **External processes describe UI — they don't draw pixels**
- Weaver remains sole authority over rendering, safety, and action execution
- UI sessions bound to template-defined slots (no arbitrary window spawning)
- Capability-based security (sessions receive limited widget/action sets)
- Every UI-triggered action logged for audit

**This enables:**

| Capability | Description |
|------------|-------------|
| **Cloud apps without browsers** | Backend runs in cloud, UI materializes locally — no Electron |
| **AI-driven interfaces** | AI proposes layouts, Weaver governs execution |
| **Dynamic industrial dashboards** | Devices declare their own control panels |
| **Language-agnostic UI** | Any language that writes to a socket can create UI |

**Example: Coffee machine declares its UI**

```json
{
  "type": "ui.define",
  "container": { "kind": "modal", "title": "Coffee Order" },
  "widgets": [
    { "id": "espresso", "type": "button", "label": "Espresso", "action": "order.espresso" },
    { "type": "status", "source": "machine.temperature" }
  ]
}
```

Weaver validates, renders native egui widgets, and routes actions to workmeshd. The coffee machine never has direct system access.

See [docs/UI_FABRIC_PROPOSAL.md](docs/UI_FABRIC_PROPOSAL.md) for full specification.

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
