# Weaver Desktop

**A lightweight, template-driven desktop environment for resource-constrained systems**

Pure GUI desktop environment built in Rust/egui.rs, designed for everything from Raspberry Pi Zero to consumer PCs.

---

## Overview

Weaver Desktop is a **pure GUI desktop environment** that delegates all system operations to the **workmeshd** daemon. This separation keeps the interface lightweight (<50MB RAM) while enabling full system management capabilities. The DE can be completely reshaped through configuration files (templates), transforming into a kiosk, media center, cyberdeck control panel, or traditional desktop.

Built with touchscreen-first design (7" displays), hardware awareness (GPIO, PWM, MCU communication), and thin-client architecture (control local or remote machines transparently), Weaver Desktop enables profile-driven system configuration across heterogeneous devices.

### Core Purpose

- **Pure desktop environment**: GUI-only, all system operations delegated to workmeshd daemon
- **Template-driven flexibility**: Reshape entire DE through configuration files (layouts, themes, widgets, components)
- **Touch-first design**: Optimized for 7" touchscreens, kiosks, and embedded displays
- **Hardware control integration**: Direct GPIO, PWM, I2C, analog/digital I/O control for cyberdecks and embedded projects
- **Thin client architecture**: Control local or remote machines transparently via TCP/IP, UDP, or WorkMesh P2P
- **Resource-efficient**: <50MB RAM footprint vs 300-600MB for traditional desktop environments
- **Cross-device profiles**: Base profiles with device-specific extensions (laptop, Raspberry Pi, cyberdeck)
- **Extensible**: Plugin system for community-driven widgets, components, and automations
- **Distro-agnostic**: Works on Debian, Ubuntu, Arch, and other Linux distributions

### Target Platforms

| Priority | Platform | Notes |
|----------|----------|-------|
| **Primary** | Raspberry Pi Zero W2, Pi 4/5 | First-class SBC support |
| **Primary** | 7" touchscreen displays | Touch-first optimization |
| **Secondary** | Legacy laptops (Acer Aspire One 725) | Low-resource x86 devices |
| **Secondary** | Consumer PCs | Full desktop replacement |
| **Future** | Android/iOS | Fullscreen app/launcher replacement |
| **Future** | Web browser | Via egui WASM support |
| **Future** | TV/media center | Large screen, remote-friendly layouts |

---

## Architecture

### Technology Stack

- **Language**: Rust
- **GUI Framework**: egui.rs (immediate mode, scalable UI)
- **Target Platform**: Linux (primary), potential cross-platform GUI support
- **Authorization**: `pkexec` for privilege escalation when required
- **Configuration**: Profile files (format TBD: TOML/JSON)

### Project Structure

```
Weaver Desktop/
├── src/
│   ├── system-operations/    # System control modules
│   │   ├── mod.rs
│   │   └── pckg.rs           # Package management
│   └── main.rs               # GUI application entry
└── Cargo.toml
```

### System Operations Module

The `src/system-operations/` module encapsulates all system-level operations:

- Package installation and management (apt, dnf, pacman)
- Multi-package-manager orchestration (system packages + Homebrew + Nix)
- Service control (systemd, etc.)
- System configuration
- System maintenance (orphaned package cleanup, cache management, permission repair)
- Health monitoring (disk space, service status, configuration validation)
- Hardware control (GPIO, PWM, power switching)
- MCU communication (via virtual COM port for devices like Tiny2040)
- Privilege escalation handling

**Future Architecture**: This module may be extracted as a standalone crate for reuse in other projects or migrated to a plugin for the `workmeshd` daemon (which runs with elevated privileges, eliminating repeated authorization prompts).

---

## Key Features

### Cross-Device Profile System

- **Hierarchical profiles**: Base profiles with device-specific extensions
- **Profile inheritance**: Share common configuration, override per device
- **Multi-device support**: Laptop, Raspberry Pi, cyberdeck from same base profile
- **State synchronization**: Continuously ensure system matches profile (detect drift)
- **Profile sharing**: Export/import profiles for team collaboration

**Example Profile Structure:**

```
base-profile.toml          # Shared: git, vim, Python, SSH
├── laptop-profile.toml    # Extends base: Firefox, VSCode
├── raspi-tv-profile.toml  # Extends base: Kodi, media codecs
└── cyberdeck-profile.toml # Extends base: GPIO tools, MCU config
```

### System Management

- Manual GUI control for all system operations
- Package installation and management (apt/dnf/pacman)
- Multi-package-manager orchestration (system + Homebrew + Nix)
- Service configuration (FTP, PHP server, SSH, etc.)
- System updates and dependency management
- Hardware control (GPIO, PWM, power switching)
- MCU communication (Tiny2040, Arduino, etc.)

### Extensibility & Automation

- **Bash script integration**: Extend system management through custom scripts
- **Service auto-configuration**: Automated setup for PHP, Node.js, MySQL, Apache, nginx, etc.
- **Built-in views**: File manager, text viewer, system log viewer
- **Automation workstation**: Build/deployment automation with visual feedback
- **Remote capabilities**: Network-based audio/video streaming and control
- **TV control interface**: Direct network control for media center setups

### Application Launcher

- **Fullscreen app launching**: Launch external applications (browser, media player, terminal) in fullscreen mode
- **Seamless transitions**: Weaver Desktop hides while app runs, automatically returns when app exits
- **Profile-based app sets**: Different profiles can define different available applications
- **Complete environment**: Makes Weaver Desktop a full kiosk/cyberdeck environment, not just a system tool
- **Common apps**: Pre-configured launchers for Firefox, Kodi, VLC, terminal emulators
- **Custom apps**: User-configurable app definitions with icons, commands, and arguments

**Use Cases:**

- Cyberdeck: Boot to Weaver Desktop → Launch browser when needed → Return to control panel
- Media center: Weaver Desktop as TV control panel → Launch Kodi for media → Return for system management
- Kiosk: Locked-down interface with only approved applications accessible

### Profile Automation (Optional)

- Load profile and automatically provision fresh systems
- Headless operation: Install OS + profile → automatic configuration
- Switch between predefined environments (dev A, dev B, production)
- Ensure system state matches profile requirements

### System Integrity & Maintenance

Weaver Desktop continuously monitors system health relative to profile requirements:

- **Profile compliance**: Detect when system drifts from defined state
- **Dependency cleanup**: Remove orphaned packages and unused dependencies
- **Disk space management**: Clean caches, logs, old kernels to meet profile requirements
- **Permission verification**: Ensure file permissions match security requirements
- **Service health**: Monitor that required services are running correctly
- **Configuration validation**: Detect broken symlinks, missing files, corrupted configs

**Positioning**: Not a generic system cleaner, but profile-aware maintenance that ensures your system stays in the defined state over time.

### Authorization Strategy

- Application knows which commands require `pkexec` elevation
- Uses `pkexec` only when necessary for privileged operations
- Future: Delegate to `workmeshd` daemon running with root privileges to avoid repeated authorization

### Hardware Control

- Built-in GPIO pin control
- PWM management
- Power switching (220V sockets)
- MCU communication via virtual COM port (Tiny2040, Arduino, etc.)
- Device-specific hardware views (cyberdeck control panel)
- Generic hardware interface for SBC and industrial platforms

### UI Design

- Scalable interface for small touchscreens (7")
- Kiosk mode operation
- Real-time operation feedback
- Window-size adaptive (not responsive, but scalable)

### Resource Efficiency

- **Lightweight architecture**: Native Rust application with egui immediate-mode GUI
- **Minimal dependencies**: No heavy frameworks, no web browser engine
- **Low memory footprint**: Suitable for Raspberry Pi and low-resource devices
- **Efficient rendering**: GPU-accelerated UI with minimal CPU overhead
- **Desktop environment alternative**: Replaces resource-heavy DEs (XFCE, LXDE, Pixel) for kiosk/appliance use cases
- **Always-on capable**: Low enough resource usage for 24/7 operation on resource-constrained devices

---

## Use Cases

### Multi-Device Management

**Scenario**: Developer with laptop, Raspberry Pi TV, and cyberdeck

- Define base profile: git, vim, Python, SSH, Docker
- Laptop extends: Firefox, VSCode, development tools
- Raspberry Pi TV extends: Kodi, media codecs, HDMI config
- Cyberdeck extends: GPIO tools, MCU communication, touchscreen config
- Reinstall any device → load profile → automatic configuration

### Complete Kiosk/Cyberdeck Environment

**Scenario**: Weaver Desktop as the primary interface (no traditional desktop environment)

1. Boot directly to Weaver Desktop in fullscreen/kiosk mode
2. Dashboard shows system status, hardware controls, profile info
3. Need to browse web? Click "Browser" → Firefox launches fullscreen
4. Done browsing? Exit Firefox → Returns to Weaver Desktop automatically
5. Need media? Click "Media" → Kodi launches fullscreen
6. All system management accessible without leaving Weaver Desktop

**Benefits:**

- No heavy desktop environment needed (saves 300-500 MB RAM)
- Single, consistent interface for all operations
- Touch-optimized throughout
- Perfect for Raspberry Pi TV, cyberdeck, kiosk displays
- Profile defines both system config AND available applications

### Docker Complementary Workflow

Weaver Desktop configures the host, Docker runs the applications:

1. Weaver Desktop provisions system (drivers, packages, services)
2. Weaver Desktop installs Docker (if profile requires it)
3. Docker containers run on properly configured host
4. Weaver Desktop manages host updates and hardware

**Positioning**: Docker manages applications, Weaver Desktop manages systems.

### Headless Provisioning

- Install OS on SD card with profile file
- Boot device (laptop, Pi, cyberdeck)
- Weaver Desktop auto-configures system without manual interaction
- Device ready for work

### State Synchronization & Health

Continuously ensure system matches profile and remains healthy:

- **Drift detection**: "Profile says Python 3.9, but 3.11 installed"
- **Health monitoring**: Orphaned packages, disk space, service status
- **Auto-fix or alert**: Configuration mismatches, permission issues
- **Maintenance tasks**: Clean caches, remove old kernels, repair broken configs
- **Profile compliance dashboard**: Visual overview of system health vs. profile requirements

### Automation Workstation

Weaver Desktop as a build/deployment automation interface:

- **Visual build pipelines**: Run bash scripts with real-time output
- **Automated deployments**: Profile-driven server provisioning
- **Service orchestration**: Auto-configure PHP, Node.js, MySQL, Apache, nginx
- **CI/CD dashboard**: Monitor build status, deployment progress
- **Script library**: Reusable automation scripts accessible via GUI

### Media Center & Remote Control

Weaver Desktop as a resource-efficient media control interface:

- **TV control panel**: Touchscreen interface for media centers (Raspberry Pi TV)
- **Remote audio/video**: Network streaming control for connected devices
- **Service management**: Control Kodi, Plex, MPD, PulseAudio services
- **Hardware integration**: GPIO-controlled displays, IR remotes, power management
- **Kiosk mode**: Boot directly to control interface, minimal resource overhead

---

## WorkMesh Ecosystem Context

Weaver Desktop is one component of the larger **WorkMesh** project:

### Related Projects

- **Weaver Desktop**: Pure GUI desktop environment (this project)
- **workmeshd**: System management daemon handling privileged operations (package management, service control, hardware abstraction). Weaver Desktop delegates all system operations to this daemon.
- **WorkMesh SaaS** (future): Secure P2P connectivity platform enabling devices with Weaver Desktop/workmeshd to connect together for remote control, automation, and headless fleet management.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Weaver Desktop (GUI)                     │
│         Pure UI - renders locally, dispatches commands      │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
       ┌───────────┐   ┌───────────┐   ┌───────────┐
       │  Local    │   │  Remote   │   │  WorkMesh │
       │ workmeshd │   │ workmeshd │   │   P2P     │
       │  (Unix)   │   │  (TCP/IP) │   │ (future)  │
       └───────────┘   └───────────┘   └───────────┘
```

### Current & Future Integration

- **Now**: GUI + local workmeshd on same machine
- **Near-term**: Remote workmeshd control via TCP/IP, UDP
- **Future**: WorkMesh SaaS for secure P2P device mesh
- Profile synchronization across mesh network
- Fleet management dashboard
- Headless automation across distributed devices

---

## Development Status

**Current State**: Initial development

- Empty `system-operations` module structure
- Basic `main.rs` entry point
- Architecture planning phase

**Next Steps**:

1. Implement core GUI structure with egui
2. Define profile file schema (with hierarchical inheritance)
3. Build system operations modules (package management, service control)
4. Implement `pkexec` authorization handling
5. Create profile loading and validation
6. Add system health monitoring and maintenance
7. Add hardware control interfaces (GPIO, PWM, MCU communication)
8. Implement bash script integration for extensibility
9. Build file manager and text viewer views
10. Add remote control capabilities (audio/video streaming)

---

## Technical Considerations

### Resource Efficiency vs. Traditional Desktop Environments

**Memory Comparison (Estimated):**

- XFCE: ~400-600 MB RAM idle
- LXDE/Pixel: ~300-400 MB RAM idle
- i3wm: ~100-200 MB RAM idle
- **Weaver Desktop (target)**: ~30-50 MB RAM idle

**Advantages:**

- No X11 window manager overhead (single fullscreen application)
- Native Rust compilation (no interpreted languages)
- Immediate-mode GUI (no retained widget trees)
- Minimal background services

**Ideal For:**

- Raspberry Pi Zero/1/2 (limited RAM)
- Always-on kiosk displays
- Cyberdeck devices (battery life)
- Media centers (TV control panels)
- Industrial automation systems

### Privilege Management

- Application runs as normal user
- Elevates privileges via `pkexec` for specific operations
- Tracks which commands require elevation
- Future: Daemon-based architecture to minimize authorization prompts

### Modularity

- `system-operations` module designed for extraction
- Clean separation enables reuse in other projects
- Potential open-source release as standalone library
- Plugin architecture for workmeshd integration

### Platform Support

- **Linux**: Full support (distro-agnostic)
- **Windows/macOS**: Potential GUI support for remote management (future consideration)

---

## Desktop Environment Stages

Weaver Desktop evolves through progressive stages from simple fullscreen app to complete desktop environment:

### Stage 1: Fullscreen Kiosk App

**Current Implementation**

- Single fullscreen egui application
- Replaces traditional desktop environment entirely
- Application launcher opens external apps (Firefox, Kodi) above Weaver Desktop
- Touch-activated panels (swipe top-center to reveal system panel)
- Modal dialogs as egui windows with always-on-top behavior

**Benefits:**

- Simple implementation (stays within egui)
- Works immediately with any compositor/window manager
- Resource efficient (30-50MB vs 300-600MB for traditional DEs)
- Perfect for kiosks, cyberdecks, SBC displays

### Stage 2: Multi-Window egui Architecture

**Future Enhancement**

- System panel as separate egui window (always-on-top)
- Modal dialogs as dedicated windows with proper layering
- Touch zones for panel activation (top-center swipe)
- Auto-hide behavior with configurable timeouts

**Implementation:**

```rust
egui::Window::new("system_panel")
    .fixed_pos([0.0, 0.0])
    .fixed_size([screen_width, 60.0])
    .always_on_top(true)
    .show(ctx, |ui| { /* panel content */ });
```

**Benefits:**

- Better separation of concerns
- Panel visible even when external apps are running
- More traditional desktop-like behavior while staying in Rust

### Stage 3: Native X11/Wayland Integration

**Full Desktop Environment**

- X11 EWMH struts for proper panel area reservation
- Wayland layer-shell protocol support
- Window management capabilities
- Proper integration with system compositor

**Technical Implementation:**

- X11: `_NET_WM_STRUT_PARTIAL` to reserve screen areas
- Wayland: `zwlr_layer_shell_v1` for panels and overlays
- Window manager functionality for external app control

**Benefits:**

- External apps automatically respect panel areas
- Professional desktop environment behavior
- Full window management capabilities
- Seamless integration with existing Linux desktop stack

### Migration Strategy

Each stage builds upon the previous while maintaining backward compatibility:

1. **Stage 1 → 2**: Add egui multi-window support while keeping fullscreen base
2. **Stage 2 → 3**: Implement X11/Wayland protocols while maintaining egui fallback
3. **Deployment flexibility**: Choose stage based on target environment complexity

This progressive approach allows Weaver Desktop to serve both simple kiosk use cases and complex desktop replacement scenarios.

---

## Desktop Widget Layout System

The desktop uses a text-based layout configuration for keyboard-friendly, profile-compatible widget management.

### Layout File Format

Layouts are defined in simple text files (`~/.config/weaver/desktop.layout` or embedded in profiles):

```
# desktop.layout
# Format: slot:widget[:config]
# Use span:N to span multiple columns

layout: grid 2x2

1: app_launcher
2: resource_monitor refresh=2s
3: log_monitor tabs=system,app,weaver
4: quick_paths paths=/home,/mnt,/var/log
```

### Cross-Column Widgets

Widgets can span multiple columns using the `span` modifier:

```
# Layout with full-width bottom widget
layout: grid 2x3

1: app_launcher
2: peripherals pins=gpio17,gpio18,pwm0
3: resource_monitor
4: quick_paths
5: log_monitor span:2    # Spans columns 1-2 in row 3
```

### Predefined Layout Templates

| Template    | Description                       |
| ----------- | --------------------------------- |
| `grid NxM`  | N columns × M rows, equal sizing  |
| `sidebar`   | Large main area + narrow sidebar  |
| `focus`     | Single widget, full desktop       |
| `dashboard` | 2x2 top + full-width bottom strip |

### Widget Types

| Widget             | Purpose                   | Config Options           |
| ------------------ | ------------------------- | ------------------------ |
| `app_launcher`     | Application grid          | `columns`, `show_labels` |
| `resource_monitor` | CPU/RAM/disk/temp         | `refresh`, `show_graph`  |
| `log_monitor`      | Tabbed log viewer         | `tabs`, `lines`          |
| `quick_paths`      | Directory shortcuts       | `paths`                  |
| `peripherals`      | GPIO/PWM/terminal control | `pins`, `mode`           |
| `clock`            | Time/date display         | `format`, `timezone`     |
| `notes`            | Quick text capture        | `file`                   |

### Layout Management via Command Center

All layout operations are keyboard-driven through the command center:

```
> layout dashboard           # Switch to dashboard layout
> widget add clock slot:3    # Add clock to slot 3
> widget remove 2            # Remove widget from slot 2
> reload                     # Reload layout from file
```

### Profile Integration

Layouts can be embedded in or referenced from profile files:

```toml
# profile.toml
[desktop]
layout_file = "layouts/cyberdeck.layout"

# Or inline:
[desktop.layout]
template = "dashboard"
widgets = [
    { slot = 1, type = "app_launcher" },
    { slot = 2, type = "peripherals", pins = ["gpio17", "pwm0"] },
]
```

---

## Target Audience

### Primary Users

- **Multi-device Linux users**: Developers managing laptops + Raspberry Pis + custom hardware
- **SBC developers**: Building projects on Raspberry Pi and similar platforms with hardware control needs
- **Cyberdeck builders**: Custom hardware platforms requiring system + hardware management
- **Home lab enthusiasts**: Managing multiple Linux systems consistently

### Secondary Users

- **Development teams**: Standardizing environments across heterogeneous infrastructure
- **Makers/hobbyists**: Raspberry Pi projects, robotics, IoT devices
- **System administrators**: Managing bare-metal Linux servers with hardware components

### Why They'll Use Weaver Desktop

- **vs. Docker**: Need to configure the host system, not just run containers
- **vs. Ansible**: Want GUI instead of YAML, need hardware control
- **vs. NixOS**: Want to use existing distro (Debian/Ubuntu/Arch), not switch to NixOS
- **vs. Native settings**: Need reproducibility across multiple devices

---

## Vision

Weaver Desktop aims to enable:

- **Bare-metal reproducibility**: Docker-style "define once, run anywhere" for native Linux systems
- **Heterogeneous device management**: Single profile system for laptops, SBCs, and custom hardware
- **Docker complementary**: Configure hosts that run Docker containers
- **Multi-profile workflows**: Switch entire system stacks based on current task (dev/production/maintenance)
- **Package manager orchestration**: Coordinate system packages + Homebrew + Nix for complete dependency management
- **Hardware abstraction**: Unified interface for GPIO, PWM, MCU communication across devices
- **Cloud-connected infrastructure**: Part of larger Workmesh ecosystem for distributed system management

### Market Positioning

"Docker manages applications. Weaver Desktop manages systems."

Weaver Desktop brings reproducible environments to bare-metal Linux, filling the gap between manual configuration and container-based deployment. It's the infrastructure layer that Docker runs on top of.

---

**Project Location**: `/Volumes/WORKING/Development/repositories/DesktopWeaver`

**Last Updated**: December 2025

**Status**: Early Development - Core infrastructure complete, building MVP views
