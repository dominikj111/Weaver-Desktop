# Weaver Desktop - TODO & Recommendations

**Last Updated**: November 29, 2025  
**Phase**: POC - Foundation Building

---

## Current Status

✅ Basic application layout  
✅ Top panel with menu icon (right) and date/time (center)  
✅ Calendar popup on date/time click  
✅ Icon-based menu system  
✅ Overlay blocking for menu interactions  
✅ Theme-aware overlay (light/dark mode compatible)

---

## Foundation Improvements (Priority)

### View System & Navigation

⬜️ Implement view state management system

```rust
enum View {
    Dashboard,
    Hardware,    // GPIO, PWM, MCU
    Profiles,    // Profile management
    System,      // Packages, services
    Files,       // File browser
    Settings,
}
```

⬜️ Add current view/path indicator in UI  
⬜️ Show breadcrumb or view title to indicate where user is  
⬜️ Navigation only by clicking icons (no gestures, keep simple)

### Dashboard View (Default Central Panel)

Replace current demo form with system status dashboard:

```
┌─────────────────────────────────────┐
│ Profile: Cyberdeck-Main     [Edit]  │
├─────────────────────────────────────┤
│ System Status:                      │
│   CPU: ███████░░░ 70%               │
│   RAM: █████░░░░░ 50%               │
│   Disk: ████░░░░░░ 40%              │
│                                     │
│ Active Services:                    │
│   ✓ SSH  ✓ FTP  ✗ Apache            │
│                                     │
│ Hardware:                           │
│   GPIO 17: HIGH  [Toggle]           │
│   PWM 1: ████░░░░ 40%               │
│                                     │
│ Quick Actions:                      │
│   [Update]  [Backup]  [Reboot]      │
└─────────────────────────────────────┘
```

#### System Status Section

⬜️ CPU usage bar with percentage  
⬜️ RAM usage bar with percentage  
⬜️ Disk usage bar with percentage  
⬜️ Visual progress bars (colored, easy to read on touchscreen)

#### Active Services Section

⬜️ List of key services with status (SSH, FTP, Apache, etc.)  
⬜️ Visual indicators (✓ running, ✗ stopped)  
⬜️ Click to view service details

#### Hardware Section (Killer Feature)

⬜️ GPIO pin states display  
⬜️ Quick toggle buttons for GPIO pins  
⬜️ PWM channel sliders with percentages  
⬜️ MCU connection status

#### Quick Actions

⬜️ Update system button  
⬜️ Backup profile button  
⬜️ Reboot system button  
⬜️ Other common operations

#### Profile Information

⬜️ Show currently active profile name  
⬜️ Profile status indicator (system matches profile vs. drift)  
⬜️ Quick profile switch button

---

## Menu Enhancements

### Additional Views to Add

Current menu icons (6 items):

- 🏠 Home (Dashboard)
- 📊 Dashboard (duplicate? or Analytics?)
- 👤 Profile
- ⚙ Settings
- 📁 Files
- 📈 Analytics

**Recommended menu structure:**

⬜️ 🏠 **Dashboard** - System status, quick actions (home view)  
⬜️ 🔧 **Hardware** - GPIO, PWM, MCU control (unique differentiator!)  
⬜️ 📋 **Profiles** - Switch/edit/create profiles  
⬜️ 📦 **System** - Packages, services, maintenance  
⬜️ 📱 **Apps** - Application launcher (fullscreen apps)  
⬜️ 📁 **Files** - File browser/viewer  
⬜️ ⚙ **Settings** - App configuration

---

## Application Launcher (Essential Feature)

### Core Functionality

⬜️ Launch external applications fullscreen  
⬜️ Hide Weaver Desktop while app is running  
⬜️ Auto-return to Weaver Desktop when app exits  
⬜️ Process monitoring and health checks  
⬜️ Configurable app definitions (per profile)

### App Management

⬜️ Hardcoded common apps (Phase 1)

- Browser (Firefox with --kiosk)
- Media player (Kodi/VLC fullscreen)
- Terminal (xterm/alacritty fullscreen)
- File manager (optional, or use built-in)

⬜️ Profile-based app configuration (Phase 2)

- Load apps from profile TOML
- Custom icons, names, commands, arguments
- Different app sets per profile (TV profile: Kodi, Dev profile: VSCode)

⬜️ Apps view with launcher grid  
⬜️ Quick launch buttons on Dashboard  
⬜️ App categories/organization  
⬜️ Recent/favorite apps tracking

### Advanced Features (Later)

⬜️ Launch modes (fullscreen hide vs. windowed keep)  
⬜️ App health monitoring (auto-restart on crash)  
⬜️ Background app management  
⬜️ Inter-app communication  
⬜️ App state preservation

**Value Proposition**: Makes Weaver Desktop a complete environment, not just a system tool. Essential for kiosk/cyberdeck use - users can browse web, watch media, etc., without leaving Weaver Desktop ecosystem.

---

## UX Improvements (Lower Priority - After Foundation)

### Touch Target Optimization

⬜️ Enlarge all buttons to 50-70px minimum for touch  
⬜️ Increase spacing between menu items  
⬜️ Make calendar dates clickable (currently just labels)  
⬜️ Add visual press feedback to all buttons (color change on touch)

### Visual Feedback

⬜️ Press states for all interactive elements  
⬜️ Active view indicator in menu  
⬜️ Loading states for system operations (already have spinner, expand usage)  
⬜️ Success/error toast notifications for operations

### Profile Integration

⬜️ Display current profile name in top bar  
⬜️ Profile status badge (matches/drift)  
⬜️ Quick profile switcher widget  
⬜️ Profile comparison view (current state vs. profile)

### Hardware Control Panel View (Core Differentiator)

#### GPIO Control Section

```
GPIO Control:
  Pin 17: [HIGH] [Toggle]
  Pin 18: [LOW]  [Toggle]
  Pin 27: [HIGH] [Toggle]
  Pin 22: [LOW]  [Toggle]
```

⬜️ Visual pin layout (if possible, map to actual Pi layout)  
⬜️ Toggle switches for digital pins  
⬜️ Input/Output mode indicators  
⬜️ Pin descriptions/labels

#### PWM Control Section

```
PWM Control:
  Channel 1: [========----] 60%
  Channel 2: [====---------] 30%
  Channel 3: [==========--] 80%
```

⬜️ Slider controls for PWM channels  
⬜️ Percentage display  
⬜️ Frequency settings  
⬜️ Duty cycle visualization

#### MCU Communication Section

```
MCU Communication:
  Tiny2040: Connected ✓
  Last command: LED_ON
  Status: Ready
  [Send Command] [View Log]
```

⬜️ Connection status for MCU devices  
⬜️ Command interface  
⬜️ Communication log viewer  
⬜️ Preset command buttons

---

## System Operations Integration

### Package Management

⬜️ Integrate `system-operations/pckg.rs` module  
⬜️ Display installed packages  
⬜️ Search and install new packages  
⬜️ Update/upgrade operations with progress  
⬜️ Support for apt (Debian/Ubuntu) initially

### Service Management

⬜️ List system services  
⬜️ Start/stop/restart controls  
⬜️ Enable/disable at boot  
⬜️ Service logs viewer  
⬜️ Service status monitoring

### Profile System

⬜️ Define profile file schema (TOML/JSON)  
⬜️ Implement profile loading  
⬜️ Profile validation  
⬜️ Profile switching mechanism  
⬜️ Profile comparison (system state vs. profile)  
⬜️ Profile editing interface

---

## Architecture & Code Organization

### Component Extraction

⬜️ Create reusable widgets:

- `StatusCard` - System metrics display
- `GPIOControl` - Pin management widget
- `ProfileBadge` - Profile status indicator
- `ServiceRow` - Service list item
- `ProgressBar` - Touch-friendly progress bars

### State Management

⬜️ Implement proper view routing  
⬜️ State persistence between views  
⬜️ Clean separation of UI and system operations  
⬜️ Error handling and user feedback

### System Operations Module

⬜️ Complete `system-operations/mod.rs` structure  
⬜️ Package manager abstraction  
⬜️ Service manager abstraction  
⬜️ Hardware control abstraction (GPIO/PWM)  
⬜️ MCU communication protocol  
⬜️ Privilege escalation with `pkexec`

---

## Hardware Support (Phase 2+)

### Initial Target

⬜️ Raspberry Pi 4/5 support  
⬜️ GPIO control via sysfs or libgpiod  
⬜️ PWM control  
⬜️ USB device detection

### MCU Integration

⬜️ Serial/virtual COM port communication  
⬜️ Tiny2040 protocol implementation  
⬜️ Arduino support  
⬜️ Other common MCU platforms

---

## Testing & Validation

### Touchscreen Testing

⬜️ Test on actual 7" touchscreen  
⬜️ Verify all touch targets are accessible  
⬜️ Test in kiosk/fullscreen mode  
⬜️ Performance testing on Pi hardware

### System Operations Testing

⬜️ Package installation/removal  
⬜️ Service control operations  
⬜️ Profile loading/switching  
⬜️ Hardware control (GPIO/PWM)

---

## Documentation

⬜️ User guide for basic operations  
⬜️ Profile creation guide  
⬜️ Hardware configuration guide  
⬜️ API documentation for system-operations module  
⬜️ Contributing guidelines

---

## Future Considerations (Post-POC)

### Multi-Device Support

⬜️ Profile sharing mechanism  
⬜️ Device-specific profile extensions  
⬜️ Profile inheritance system

### Advanced Features

⬜️ System state snapshots (time travel)  
⬜️ Profile version control  
⬜️ Diff view (current vs. profile)  
⬜️ Rollback functionality  
⬜️ Profile marketplace concept

### Extensibility & Automation

⬜️ Bash script integration for custom system operations  
⬜️ Script library management (add/edit/run scripts from GUI)  
⬜️ Service auto-configuration (PHP, Node.js, MySQL, Apache, nginx)  
⬜️ Build/deployment automation dashboard  
⬜️ Visual pipeline execution with real-time output  
⬜️ CI/CD integration capabilities

### Built-in Views & Tools

⬜️ File manager view (browse, view files)  
⬜️ Text viewer (read-only, syntax highlighting optional)  
⬜️ Log viewer (system logs, service logs)  
⬜️ Process monitor (running processes, resource usage)  
⬜️ Network monitor (connections, bandwidth)

### Remote Control & Media

⬜️ Network-based remote control API  
⬜️ Audio/video streaming control  
⬜️ Remote command execution  
⬜️ Media service integration (Kodi, Plex, MPD)  
⬜️ TV control interface mode  
⬜️ Multi-device orchestration via network

### Integration

⬜️ workmeshd daemon integration  
⬜️ Remote management capabilities  
⬜️ Cloud sync (optional, aligned with local-first principle)

---

## Notes

**Design Principle**: Keep navigation simple - icon-based clicking only, no gestures or context menus for now.

**Focus**: Build solid foundation first, UX polish later.

**Differentiator**: Hardware control (GPIO, PWM, MCU) integrated with system management - this is unique and should be showcased prominently.

**Target**: Cyberdeck use case with 7" touchscreen in kiosk mode, no mouse required.

**Resource Efficiency**: Target 30-50 MB RAM footprint vs. 300-600 MB for traditional DEs (XFCE, LXDE, Pixel). Suitable for low-resource devices (Pi Zero/1/2) and 24/7 operation.

**Extensibility**: Support bash scripts, service auto-configuration, file management, and remote control to enable broad use cases while keeping core lightweight.

---

## Next Immediate Steps

1. Implement view state management (View enum)
2. Create Dashboard view with system status
3. Add Hardware view with GPIO/PWM controls
4. Enhance menu with proper view icons
5. Add current view indicator in UI
6. Integrate basic system-operations functionality

---

**Remember**: Ship Phase 1 before expanding scope. Focus on core functionality that demonstrates the unique value proposition.
