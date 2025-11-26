# SystemWeaver

A GUI-focused Linux system management application with optional profile-driven automation.

---

## Overview

SystemWeaver is a Rust/egui.rs application that provides centralized GUI control over Linux system operations. Built primarily for touchscreen kiosk environments (7" displays), it offers manual control of system configuration, package management, service control, and hardware operations. Profile files enable optional automation to reduce manual interaction when provisioning fresh systems or switching between predefined environments.

### Core Purpose

- **Single control point**: Unified GUI for all system management operations
- **Manual or automated**: Full manual control via GUI, with optional profile automation for fresh systems
- **Profile support**: Load profile files to automatically satisfy system requirements (packages, services, settings)
- **Kiosk-ready**: Designed for touchscreen devices (7") with scalable UI
- **Distro-agnostic**: Targets Linux generically, currently developed on Debian-based systems

---

## Architecture

### Technology Stack

- **Language**: Rust
- **GUI Framework**: egui.rs (immediate mode, scalable UI)
- **Target Platform**: Linux (primary), potential cross-platform GUI support
- **Authorization**: `pkexec` for privilege escalation when required
- **Configuration**: Profile files (format TBD: TOML/JSON)

### Project Structure

```()
SystemWeaver/
├── src/
│   ├── system-operations/    # System control modules
│   │   ├── mod.rs
│   │   └── pckg.rs           # Package management
│   └── main.rs               # GUI application entry
└── Cargo.toml
```

### System Operations Module

The `src/system-operations/` module encapsulates all system-level operations:

- Package installation and management
- Service control (systemd, etc.)
- System configuration
- Hardware control (IO pins, PWM, power switching)
- Privilege escalation handling

**Future Architecture**: This module may be extracted as a standalone crate for reuse in other projects or migrated to a plugin for the `workmeshd` daemon (which runs with elevated privileges, eliminating repeated authorization prompts).

---

## Key Features

### System Management

- Manual GUI control for all system operations
- Package installation and management
- Service configuration (FTP, PHP server, SSH, etc.)
- System updates and dependency management
- Hardware control (IO, PWM, power)

### Profile Automation (Optional)

- Load profile files defining complete system states
- Automatically provision fresh systems without manual interaction
- Switch between predefined environments (dev A, dev B, production, etc.)
- Ensure system state matches profile requirements

### Authorization Strategy

- Application knows which commands require `pkexec` elevation
- Uses `pkexec` only when necessary for privileged operations
- Future: Delegate to `workmeshd` daemon running with root privileges to avoid repeated authorization

### Hardware Control

- Built-in IO pin control
- PWM management
- Power switching (220V sockets)
- Generic hardware interface for cyberdeck/embedded platforms

### UI Design

- Scalable interface for small touchscreens (7")
- Kiosk mode operation
- Real-time operation feedback
- Window-size adaptive (not responsive, but scalable)

---

## Use Cases

### Cyberdeck Platform

Primary target: Hardware development platform with touchscreen interface running in kiosk mode. Use the GUI for manual system control, or plug in an SD card with OS + profile file for automatic system provisioning.

### Profile Switching

- Development environment A (web stack)
- Development environment B (embedded tools)
- Production/deployment configuration
- System maintenance mode

### Ensurance Application

Ensures system state matches profile requirements:

- Installs missing packages
- Enables/disables services
- Applies configuration changes
- Manages system updates

---

## Workmesh Ecosystem Context

SystemWeaver is part of the broader Workmesh project vision:

### Related Projects

- **workmeshd**: Pluggable daemon for P2P mesh networking and remote orchestration. SystemWeaver may communicate with this daemon in the future, potentially offloading system operations to daemon plugins.
- **WorkFlows**: Debian-based Linux distribution. SystemWeaver is compatible but not tightly coupled.

### Future Integration

- Cloud connectivity for profile distribution
- Remote system management via workmeshd
- Multi-device orchestration
- Profile synchronization across mesh network

---

## Development Status

**Current State**: Initial development

- Empty `system-operations` module structure
- Basic `main.rs` entry point
- Architecture planning phase

**Next Steps**:

1. Implement core GUI structure with egui
2. Define profile file schema
3. Build system operations modules (package management, service control)
4. Implement `pkexec` authorization handling
5. Create profile loading and validation
6. Add hardware control interfaces

---

## Technical Considerations

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

## Vision

SystemWeaver aims to enable:

- **Automated system provisioning**: Insert SD card → boot → automatic configuration
- **Multi-profile workflows**: Switch entire system stacks based on current task
- **Package manager orchestration**: Coordinate system packages, Linuxbrew, Nix for complete dependency management
- **Cloud-connected infrastructure**: Part of larger Workmesh ecosystem for distributed system management

---

**Project Location**: `/Volumes/WORKING/Development/repositories/SystemWeaver`

**Last Updated**: November 2025

**Status**: Initial Development
