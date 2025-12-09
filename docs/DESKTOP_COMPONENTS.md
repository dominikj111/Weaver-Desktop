# SystemWeaver Desktop Components

This document tracks the components needed for a complete desktop environment.

## Status Legend

- ✅ Done
- 🚧 In Progress
- ⏳ Planned
- ❌ Not Started

---

## Design Philosophy

### Visual Keyboard Navigation

When the **Super/Meta key** is pressed, the UI displays hint overlays on all interactive
elements. Press the shown key to activate that element. This provides:

- **Zero learning curve** - hints are always visible when needed
- **Fast navigation** - muscle memory develops naturally
- **Touch-friendly** - hints can also be tapped
- **Accessibility** - full keyboard-only operation

```
┌─────────────────────────────────────────────────────────────────┐
│  [A] Menu     [S] Search     [D] Date/Time     [F] Files       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                      ┌─────────────────┐                        │
│                      │   [T] Terminal  │                        │
│                      │   [C] Calc      │                        │
│                      │   [B] Browser   │                        │
│                      └─────────────────┘                        │
│                                                                 │
│         Press Super → see hints → press key → action            │
└─────────────────────────────────────────────────────────────────┘
```

### Fullscreen-Only External Apps

Third-party applications (browsers, office suites, games) run in **fullscreen mode only**.
This simplifies window management and optimizes for small screens.

- **No window decorations** - maximizes usable space
- **Top banner on hover** - appears when mouse/touch reaches top edge
- **Task switching** - via keyboard shortcut or gesture, not window list
- **Future: split-screen** - optional side-by-side mode (not MVP)

```
┌─────────────────────────────────────────────────────────────────┐
│ ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │ ← Hidden top bar
├─────────────────────────────────────────────────────────────────┤   (appears on hover)
│                                                                 │
│                                                                 │
│                    FULLSCREEN APPLICATION                       │
│                    (Firefox, LibreOffice, etc.)                 │
│                                                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Target Platforms

| Priority      | Target        | Notes                                   |
| ------------- | ------------- | --------------------------------------- |
| **Primary**   | Small screens | Tablets, embedded devices, Raspberry Pi |
| **Secondary** | Consumer PCs  | Laptops, desktops                       |

---

## Architecture Principle

> **GUI ↔ Commands ↔ Backend Separation**
>
> All components are **pure UI** (egui views). They display state and dispatch commands.
> Actual system operations (network config, package install, etc.) are handled by
> **command processors** or the **workmeshd daemon**, keeping GUI decoupled from tasks.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   UI Component  │ ──▶ │   CommandBus    │ ──▶ │ Command Handler │
│   (pure view)   │     │   (dispatch)    │     │ (side effects)  │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                                        ▼
                                                ┌─────────────────┐
                                                │   workmeshd     │
                                                │   (daemon)      │
                                                └─────────────────┘
```

---

## Core Shell Components

| Component           | Status | Description                           |
| ------------------- | ------ | ------------------------------------- |
| Top Bar             | ✅     | Clock, status icons, quick actions    |
| Bottom Bar          | ✅     | Status, version info                  |
| Toast Notifications | ✅     | Basic popup notifications             |
| Command Bus         | ✅     | Event dispatch system                 |
| Task Spawner        | ✅     | Background task execution             |
| Log Panel           | ✅     | Tabbed log viewer (App/Weaver/System) |
| Terminal            | ✅     | Embedded terminal (alacritty backend) |
| Top Menu            | 🚧     | Quick access menu (commented out)     |
| Calendar Popup      | 🚧     | Date picker (commented out)           |

---

## Essential Desktop Components

### High Priority

| Component           | Status | Description                 | Notes                                |
| ------------------- | ------ | --------------------------- | ------------------------------------ |
| App Launcher        | ❌     | Start applications, search  | Grid of apps, search bar             |
| Settings Panel      | ❌     | System settings UI          | Container for settings views         |
| Quick Settings      | ❌     | Dropdown from top bar       | Toggles, sliders for common settings |
| Task/Window Manager | ❌     | Switch between running apps | For fullscreen app switching         |
| Power Menu          | ❌     | Shutdown, reboot, suspend   | Modal dialog                         |
| Login/Lock Screen   | ❌     | Authentication screen       | PIN/password, user selection         |

### Medium Priority

| Component           | Status | Description          | Notes                            |
| ------------------- | ------ | -------------------- | -------------------------------- |
| On-Screen Keyboard  | ❌     | Touch input          | Essential for touch-first vision |
| Notification Center | ❌     | Notification history | Expandable from top bar          |

### Low Priority

| Component         | Status | Description            | Notes                           |
| ----------------- | ------ | ---------------------- | ------------------------------- |
| Clipboard Manager | ❌     | Copy/paste history     | Optional enhancement            |
| Screenshot Tool   | ❌     | Capture screen regions | Can use external tool initially |

---

## Settings Views

Settings Panel contains multiple sub-views. Each view is **pure UI** that displays
current state and dispatches commands for changes.

| View              | Status | Description                      | Commands Dispatched                    |
| ----------------- | ------ | -------------------------------- | -------------------------------------- |
| WiFi              | ❌     | Network list, connect/disconnect | `WifiConnect`, `WifiDisconnect`        |
| Bluetooth         | ❌     | Device pairing, connections      | `BtPair`, `BtConnect`, `BtForget`      |
| Display           | ❌     | Resolution, brightness, rotation | `SetBrightness`, `SetResolution`       |
| Audio             | ❌     | Volume, input/output devices     | `SetVolume`, `SetAudioDevice`          |
| Keyboard          | ❌     | Layout, shortcuts                | `SetKeyboardLayout`                    |
| Language/Locale   | ❌     | System language, formats         | `SetLocale`                            |
| Date/Time         | ❌     | Timezone, NTP, manual set        | `SetTimezone`, `SetDateTime`           |
| Printers          | ❌     | Printer management               | `AddPrinter`, `RemovePrinter`          |
| Firewall          | ❌     | Rules, enable/disable            | `SetFirewallRule`, `EnableFirewall`    |
| Users/Accounts    | ❌     | User management                  | `AddUser`, `RemoveUser`, `SetPassword` |
| Power/Battery     | ❌     | Power profiles, battery info     | `SetPowerProfile`                      |
| Storage           | ❌     | Disk usage, mount points         | `Mount`, `Unmount`, `Eject`            |
| Accessibility     | ❌     | Font size, contrast, etc.        | `SetAccessibilityOption`               |
| About/System Info | ❌     | OS version, hardware info        | Read-only view                         |

---

## File Management

| Component     | Status | Description          | Notes                            |
| ------------- | ------ | -------------------- | -------------------------------- |
| File Explorer | ❌     | Browse filesystem    | Sidebar tree + grid/list view    |
| Image Viewer  | ❌     | View images          | PNG, JPG, etc. via `image` crate |
| Text Viewer   | ❌     | View/edit text files | Syntax highlighting optional     |
| Media Player  | ❌     | Play audio/video     | `vlc-rs` - handles all formats   |

---

## Built-in Utilities

Basic tools that ship with the desktop environment.

| Component   | Status | Description              | Notes                   |
| ----------- | ------ | ------------------------ | ----------------------- |
| Calculator  | ❌     | Basic + scientific modes | Essential utility       |
| Notes/Memo  | ❌     | Quick text capture       | Simple, no formatting   |
| Calendar    | 🚧     | Full calendar view       | Expand from popup       |
| Clock/Timer | ❌     | Alarms, stopwatch, timer | Useful for productivity |
| Contacts    | ❌     | Address book             | Optional, low priority  |
| PDF Viewer  | ❌     | View PDF documents       | Common need             |

---

## Productivity Tools

Offline-first task and time management.

| Component      | Status | Description               | Notes                          |
| -------------- | ------ | ------------------------- | ------------------------------ |
| Todo List      | ❌     | Task management           | Simple lists, due dates        |
| Pomodoro Timer | ❌     | Focus/break intervals     | 25/5 min cycles, notifications |
| Kanban Board   | ❌     | Visual task flow          | Columns: Todo/Doing/Done       |
| Habit Tracker  | ❌     | Daily habit streaks       | Optional, low priority         |
| Expense Track  | ❌     | Simple income/expense log | Basic personal finance         |

---

## Disk & Storage Tools

GUI wrappers for system commands - all operations via CommandBus → backend.

| Component     | Status | Description               | Commands Dispatched             |
| ------------- | ------ | ------------------------- | ------------------------------- |
| Disk Manager  | ❌     | View/manage partitions    | `ListDisks`, `MountDisk`        |
| USB Formatter | ❌     | Format external drives    | `FormatDisk { fs_type, label }` |
| ISO Flasher   | ❌     | Create bootable USB       | `FlashISO { iso_path, device }` |
| Disk Usage    | ❌     | Visual space analyzer     | `ScanDiskUsage`                 |
| Backup Tool   | ❌     | Simple file/folder backup | `CreateBackup`, `RestoreBackup` |

---

## Games & Recreation

Every desktop needs some fun. Simple, offline games.

### Card & Board Games

| Component   | Status | Description       | Notes                  |
| ----------- | ------ | ----------------- | ---------------------- |
| Solitaire   | ❌     | Classic card game | The essential DE game  |
| Minesweeper | ❌     | Logic puzzle      | Another classic        |
| Chess       | ❌     | vs AI or local 2P | Optional               |
| Sudoku      | ❌     | Number puzzle     | Good for touch screens |
| Gomoku      | ❌     | Five in a row     | Simple strategy game   |
| Battleships | ❌     | Naval strategy    | vs AI or local 2P      |

### Arcade Games

| Component      | Status | Description   | Notes             |
| -------------- | ------ | ------------- | ----------------- |
| Snake          | ❌     | Classic snake | Easy to implement |
| Tetris         | ❌     | Block puzzle  | Addictive classic |
| Pacman         | ❌     | Maze chase    | Touch-friendly    |
| Space Invaders | ❌     | Shoot 'em up  | Retro arcade      |

---

## System Integration (workmeshd)

These components interact heavily with the daemon for privileged operations.

| Component            | Status | Description              | Notes                         |
| -------------------- | ------ | ------------------------ | ----------------------------- |
| Service Status Panel | ❌     | Show running services    | Health, restart controls      |
| Hardware Monitor     | ❌     | GPIO states, sensors     | For embedded/IoT use          |
| Profile Switcher     | ❌     | Quick profile switching  | System configuration profiles |
| Update Manager       | ❌     | System/package updates   | Via workmeshd daemon          |
| Software Center      | ❌     | Install/remove apps      | See Software Center section   |
| System Cleanup       | ❌     | Clear caches, temp files | Free up disk space            |

---

## Software Center

Unified app installation UI supporting multiple package sources.

### Supported Backends

| Backend            | Status | Description               | Notes                     |
| ------------------ | ------ | ------------------------- | ------------------------- |
| System Package     | ❌     | apt, dnf, pacman, etc.    | Native distro packages    |
| Flatpak            | ❌     | Sandboxed apps            | Cross-distro, isolated    |
| Nix                | ❌     | Declarative packages      | Reproducible installs     |
| Homebrew/Linuxbrew | ❌     | Brew packages             | User-space installs       |
| AppImage           | ❌     | Portable apps             | No install, just run      |
| Custom Repos       | ❌     | GitHub, FTP, HTTP sources | User-defined repositories |

### UI Features

| Feature        | Status | Description                | Notes                         |
| -------------- | ------ | -------------------------- | ----------------------------- |
| Browse/Search  | ❌     | Find apps by name/category | Unified search across sources |
| Install/Remove | ❌     | One-click operations       | Progress feedback via toasts  |
| Updates        | ❌     | Check & apply updates      | Per-source or all at once     |
| Source Manager | ❌     | Add/remove/enable sources  | Configure backends            |
| Installed Apps | ❌     | List what's installed      | Filter by source              |

### Architecture

```
┌─────────────────┐
│  Software Center │  (Pure UI)
│     (egui)       │
└────────┬────────┘
         │ dispatch commands
         ▼
┌─────────────────┐
│   CommandBus     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Package Backend  │  (workmeshd or TaskSpawner)
│   Abstraction    │
└────────┬────────┘
         │ calls appropriate tool
         ▼
┌───────────────────────────────────────┐
│ apt │ dnf │ nix │ flatpak │ brew │ ... │
└───────────────────────────────────────┘
```

---

## Session Management

| Component       | Status | Description               | Notes                       |
| --------------- | ------ | ------------------------- | --------------------------- |
| Session Manager | ❌     | Track running apps, state | For logout/restart recovery |
| Autostart       | ❌     | Apps to launch on login   | Config-based                |
| Idle Detection  | ❌     | Screen lock after timeout | Triggers lock screen        |

---

## Suggested Build Order

1. **Login/Lock Screen** - Security gate, first thing users see
2. **App Launcher** - Core UX, needed to start anything
3. **Keyboard Hint System** - Visual navigation overlay (differentiator)
4. **Settings Panel** - Container with initial views (Display, Audio)
5. **Quick Settings** - Fast access to common toggles
6. **File Explorer** - Navigate and open files
7. **Calculator** - Essential utility
8. **Image Viewer** - Simple, egui handles images well
9. **Text Viewer** - Readonly TextEdit with syntax highlighting
10. **Media Player** - Audio/video via vlc-rs
11. **On-Screen Keyboard** - Touch-first support
12. **Task Switcher** - Fullscreen app switching (no window management)
13. **Remaining Settings Views** - WiFi, Bluetooth, etc.

---

## Architecture Notes

### Component Pattern

All components should follow the established pattern:

```rust
pub struct MyComponent {
    // UI state only (not system state)
}

impl MyComponent {
    pub fn new() -> Self { ... }

    // Render in a Ui context
    pub fn ui(&mut self, ui: &mut egui::Ui, bus: &CommandBus<AppCommand>) { ... }

    // Or render as floating window
    pub fn ui_window(&mut self, ctx: &egui::Context, bus: &CommandBus<AppCommand>) { ... }
}
```

### Command Integration

Components dispatch commands via `CommandBus`. Commands are processed separately,
keeping UI code free of side effects:

```rust
// UI dispatches intent
bus.dispatch(AppCommand::WifiConnect { ssid, password });

// Handler (in App or daemon) performs action
fn handle_command(&mut self, cmd: AppCommand) {
    match cmd {
        AppCommand::WifiConnect { ssid, password } => {
            // Spawn background task or send to workmeshd
            self.task_spawner.spawn(|ctx| {
                let result = wifi::connect(&ssid, &password);
                ctx.send(AppCommand::WifiConnectResult(result));
            });
        }
        AppCommand::WifiConnectResult(Ok(())) => {
            self.state.wifi_connected = true;
        }
        AppCommand::WifiConnectResult(Err(e)) => {
            bus.dispatch(AppCommand::ShowToast {
                message: format!("WiFi failed: {}", e),
                kind: ToastKind::Error,
            });
        }
        // ...
    }
}
```

### External App Launching

For standalone applications (office suite, games, etc.):

```rust
// Spawn external app in fullscreen
std::process::Command::new("libreoffice")
    .arg(&file_path)
    .spawn();
```

---

## Dependencies to Add

| Crate     | Purpose                               |
| --------- | ------------------------------------- |
| `walkdir` | Directory traversal for file explorer |
| `notify`  | Filesystem change watching            |
| `image`   | Image loading/decoding                |
| `syntect` | Syntax highlighting for text viewer   |
| `vlc-rs`  | Media playback (audio/video)          |

---

## References

- [PROPOSAL.md](./PROPOSAL.md) - Project vision and goals
- [egui_term](https://github.com/Harzu/egui_term) - Terminal implementation reference
