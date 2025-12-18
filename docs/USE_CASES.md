# Weaver Desktop: Reference Use Cases

This document describes concrete use cases that drive Weaver Desktop's design decisions. These aren't theoretical scenarios — they're the actual applications shaping the architecture.

---

## 1. Solar Grid Control Terminal (Origin Use Case)

### The Problem

Distributed solar power installations require secure, auditable control:

- **Switch relays** for load management and safety isolation
- **Monitor voltage/current** across multiple nodes
- **Log all operator actions** for accountability
- **Work offline** or in spotty connectivity environments
- **Physical trust anchor** — not a web dashboard accessible from anywhere

Traditional solutions fail this use case:

| Approach | Problem |
|----------|---------|
| **Web dashboards** | Require internet, no physical trust anchor, browser tabs left open |
| **SSH scripts** | No audit trail, easy to run wrong command, no visual state |
| **Industrial SCADA** | Expensive, proprietary, overkill for small installations |
| **Generic IoT platforms** | Cloud-dependent, subscription-based, data leaves your control |

### The Weaver Solution

**Cyberdeck as field terminal:**

- Raspberry Pi Zero with 7" touchscreen
- Physical device = physical trust anchor
- Known hardware, known OS image, known keys
- Works fully offline

**Panel-based control:**

- Operator sees "Inverter Cluster 2" not "GPIO 17, 18, 22"
- Safety states visible: OFF → ARMED → ON → FAULT
- Dangerous operations require deliberate interaction (hold, confirm)
- Presets for common workflows

**workmeshd handles the hard parts:**

- Owns GPIO mapping and safety rules
- Enforces state machine transitions
- Logs every action with operator, timestamp, target, result
- Exposes clean API to Weaver UI

**Audit log example:**

```
2025-12-14 14:32:08
Operator: deck-01
Action: enable
Panel: inverter_cluster_2
From state: ARMED
To state: ON
Result: OK

2025-12-14 14:45:22
Operator: deck-01
Action: disable
Panel: inverter_cluster_2
From state: ON
To state: OFF
Result: OK
Reason: Scheduled maintenance
```

### Architecture Implications

This use case drove several key decisions:

1. **GUI/daemon separation** — Weaver is unprivileged, workmeshd owns hardware
2. **Panel abstraction** — UI never knows pin numbers
3. **State machines** — Not just ON/OFF, but proper transition logic
4. **Logging as first-class** — Not optional, not an afterthought
5. **Offline-first** — No cloud dependency in the critical path

---

## 2. Cyberdeck Development Platform

### The Problem

Cyberdeck builders need a desktop environment that:

- Runs on minimal hardware (Pi Zero, limited RAM)
- Provides GPIO/hardware control without external tools
- Looks intentional, not like a hacked-together Linux install
- Supports touchscreens as primary input
- Doesn't drain battery with background services

Existing options:

| Option | Problem |
|--------|---------|
| **Raspberry Pi OS Desktop** | 300MB+ RAM, not touch-optimized |
| **DietPi + X11** | Minimal but no DE, just window manager |
| **XFCE/LXDE** | Still 200MB+, no hardware integration |
| **Custom scripts** | No visual feedback, hard to demo |

### The Weaver Solution

**Reference build:**

- Raspberry Pi Zero W2
- 7" capacitive touchscreen
- GPIO breakout with demo relays
- 3.3V / 5V / 12V / 240V rails (for presentation)

**What Weaver provides:**

- Sub-50MB RAM footprint
- Touch-first UI with large targets
- Hardware control panels built-in
- Template system for different contexts
- Clean visual identity (not "Linux with a skin")

**Demo workflow:**

1. Boot cyberdeck (5 seconds to usable)
2. Show hardware panel with relay controls
3. Toggle 230V socket with visible safety confirmation
4. Show live feedback (status, duration, load)
5. Switch to "Presentation Mode" preset
6. Audience understands in 10 seconds

### Panel Configuration Example

```toml
[[panel]]
id = "desk_socket"
label = "Desk Power Socket"
type = "relay"
voltage = "230V"
danger = true

[panel.gpio]
control = 17
enable = 22
sense = 27

[panel.safety]
require_arm = true
hold_duration_ms = 2000
confirm_on_enable = true

[panel.feedback]
show_duration = true
show_load = true  # requires current sensor
```

---

## 3. Home Media Center / TV Interface

### The Problem

Living room TV needs a simple, remote-friendly interface:

- Large buttons, readable from couch distance
- Media controls (play, pause, volume)
- Quick access to streaming apps
- System status (storage, network)
- Maybe one hardware control (lamp, amplifier relay)
- Family-friendly (no terminal access visible)

### The Weaver Solution

**Same codebase, different template:**

```toml
[template]
name = "media_center"
base = "fullscreen"
navigation = "remote_friendly"

[layout]
primary_view = "media_launcher"
show_top_bar = false
show_dock = true
dock_position = "bottom"
dock_items = ["kodi", "firefox", "spotify", "settings", "power"]

[theme]
scale = 1.5  # larger for TV distance
button_min_size = 80
font_size_base = 18

[hardware]
[[hardware.panel]]
id = "room_lamp"
label = "Room Lamp"
type = "relay"
gpio = 17
show_in_dock = true
```

**Key insight:** The template system isn't just theming — it fundamentally reshapes what components exist and how they behave. Same binary, completely different experience.

---

## 4. Kiosk / Point of Sale

### The Problem

Kiosk deployments need:

- Locked-down interface (no escape to desktop)
- Single-purpose application focus
- Remote management capability
- Automatic recovery from crashes
- Minimal attack surface

### The Weaver Solution

**Kiosk template:**

```toml
[template]
name = "kiosk"
base = "locked"
escape_key = false
virtual_keyboard = true

[layout]
primary_view = "kiosk_app"
fullscreen = true
show_top_bar = false
show_dock = false

[security]
allow_settings = false
allow_terminal = false
require_pin_for_exit = true
exit_pin = "supervisor_pin"  # reference to secure storage

[recovery]
restart_on_crash = true
health_check_interval_sec = 30
```

**Remote management:**

- workmeshd accepts commands from authorized remote Weaver instance
- Update kiosk content without physical access
- Monitor health and restart remotely
- Collect logs for diagnostics

---

## 5. Maker Space Workstation

### The Problem

Shared workstations in maker spaces need:

- Quick reset between users
- Profile-based configurations (3D printing, electronics, woodworking)
- Prevent accidental system changes
- Hardware tool integration (soldering station, power supplies)
- Usage logging for billing/accountability

### The Weaver Solution

**Profile switching:**

```
┌─────────────────────────────────────┐
│  Select Workstation Profile         │
│                                     │
│  [ 🖨️ 3D Printing ]                 │
│  [ ⚡ Electronics Lab ]              │
│  [ 🪚 Woodworking ]                  │
│  [ 🔧 General Workshop ]             │
│                                     │
│  Current user: guest                │
│  Session started: 14:32             │
└─────────────────────────────────────┘
```

Each profile:

- Loads appropriate applications
- Configures hardware panels (power supplies, tools)
- Sets UI layout for the task
- Logs session start/end

---

## 6. Robot Control Node / Gateway

### The Problem

Robots and autonomous systems need a control/management layer that:

- **Provides a human interface** for monitoring and intervention
- **Manages multiple MCUs** (motor controllers, sensors, actuators)
- **Enables firmware updates** without physical access to each board
- **Bridges communication** between high-level control and low-level hardware
- **Logs telemetry** for debugging and analysis
- **Supports field updates** without disassembling the robot

Traditional approaches:

| Approach | Problem |
|----------|---------|
| **Direct MCU programming** | Requires physical access, no runtime monitoring |
| **ROS on full Linux** | Heavy, complex, overkill for many robots |
| **Custom embedded code** | No standard UI, hard to debug in field |
| **Cloud robotics** | Latency, connectivity dependency, privacy |

### The Weaver Solution

**Weaver as the robot's "brain interface":**

```
┌─────────────────────────────────────────────────────────┐
│  ROBOT CONTROL NODE                                     │
│  ┌─────────────────────────────────────────────────┐    │
│  │  Weaver Desktop (Pi 4/5 or CM4)                 │    │
│  │  • Visual dashboard for operator                │    │
│  │  • Firmware update interface                    │    │
│  │  • Telemetry visualization                      │    │
│  │  • Emergency stop UI                            │    │
│  └─────────────────────────────────────────────────┘    │
│                         │                               │
│                    workmeshd                            │
│                         │                               │
│         ┌───────────────┼───────────────┐               │
│         ▼               ▼               ▼               │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐          │
│   │ Tiny2040 │    │ Arduino  │    │ ESP32    │          │
│   │ Motors   │    │ Sensors  │    │ Comms    │          │
│   └──────────┘    └──────────┘    └──────────┘          │
└─────────────────────────────────────────────────────────┘
```

**Panel abstractions for robot subsystems:**

| Panel | What It Controls | Underlying Hardware |
|-------|------------------|---------------------|
| 🦿 Left Leg | Position, torque, status | Tiny2040 via UART |
| 🦿 Right Leg | Position, torque, status | Tiny2040 via UART |
| 👁️ Vision | Camera feed, detection | USB camera + GPIO trigger |
| 🔋 Power | Battery status, e-stop | I2C power monitor + GPIO relay |
| 📡 Comms | Network status, signal | ESP32 via SPI |

**Firmware update workflow:**

```
┌─────────────────────────────────────┐
│  🔧 Firmware Management             │
│                                     │
│  Motor Controller (Tiny2040)        │
│  Current: v1.2.3                    │
│  Available: v1.2.4                  │
│  [ VIEW CHANGELOG ]                 │
│  [ ⬆️ UPDATE FIRMWARE ]             │
│                                     │
│  Sensor Hub (Arduino)               │
│  Current: v2.0.1                    │
│  Status: ✅ Up to date              │
│                                     │
│  ⚠️ Update requires motor stop      │
└─────────────────────────────────────┘
```

**workmeshd capabilities:**

- **MCU communication** — UART, I2C, SPI to connected microcontrollers
- **Firmware flashing** — UF2, avrdude, esptool integration
- **Protocol bridging** — translate high-level commands to MCU protocols
- **Watchdog** — monitor MCU heartbeats, trigger safe mode on failure
- **Telemetry aggregation** — collect sensor data, buffer for UI

**Template for robot control:**

```toml
[template]
name = "robot_control"
base = "dashboard"
emergency_stop_visible = true

[layout]
primary_view = "subsystem_overview"
sidebar = true
sidebar_items = ["status", "motors", "sensors", "power", "firmware", "logs"]

[[panel]]
id = "emergency_stop"
type = "e_stop"
label = "EMERGENCY STOP"
position = "top_right"
always_visible = true
gpio = 4
active_low = true

[[panel]]
id = "left_leg"
type = "mcu_subsystem"
label = "Left Leg"
connection = "uart:/dev/ttyACM0:115200"
protocol = "weaver_mcu_v1"
firmware_type = "uf2"

[[panel]]
id = "battery"
type = "power_monitor"
label = "Main Battery"
i2c_address = 0x40
show_graph = true
warn_threshold = 20
critical_threshold = 10
```

### Architecture Implications

This use case adds:

1. **MCU communication protocols** — workmeshd speaks to microcontrollers
2. **Firmware management** — version tracking, safe update procedures
3. **Telemetry streaming** — real-time data from sensors to UI
4. **Emergency stop** — hardware-level safety with UI visibility
5. **Subsystem health** — watchdogs, heartbeats, fault detection

---

## 7. Vehicle / Fleet Dashboard

### The Problem

Small vehicle fleets (delivery robots, golf carts, agricultural equipment) need:

- Real-time status of multiple units
- Remote diagnostics
- Geolocation tracking
- Maintenance scheduling
- Driver/operator interface on each vehicle

### The Weaver Solution

**On-vehicle display:**

- Weaver running on vehicle's embedded display
- Shows operator-relevant panels (battery, speed, cargo)
- Touch controls for vehicle functions
- GPS position reporting to central node

**Central fleet view:**

- Weaver on control room workstation
- Aggregated status from all vehicles via WorkMesh
- Alert on anomalies (low battery, geofence breach)
- Remote intervention capability

**Panel examples:**

```
┌─────────────────────────────────────┐
│  🚛 Vehicle Status                  │
│                                     │
│  Speed: 12 km/h                     │
│  Battery: 78% (4.2h remaining)      │
│  Cargo: Loaded (45 kg)              │
│  GPS: 51.5074° N, 0.1278° W         │
│                                     │
│  [ 🛑 STOP ] [ 📍 NAVIGATE ] [ 📞 ] │
└─────────────────────────────────────┘
```

---

## 8. Home Automation Hub

### The Problem

DIY home automation often involves:

- Multiple protocols (Zigbee, Z-Wave, WiFi, RF433)
- Various device types (lights, sensors, locks, HVAC)
- Desire for local control (no cloud dependency)
- Family-friendly interface (not a tech dashboard)

Commercial solutions (Home Assistant, OpenHAB) are powerful but:

- Web-based (browser always open)
- Complex for simple needs
- Heavy on resources

### The Weaver Solution

**Dedicated home hub:**

- Pi with touchscreen mounted on wall or shelf
- Always-on display showing home status
- Touch to control lights, scenes, climate
- Physical trust (local network only)

**Panel examples:**

```
┌─────────────────────────────────────┐
│  🏠 Living Room                     │
│                                     │
│  💡 Ceiling Light    [ ████░░ 70% ] │
│  💡 Floor Lamp       [ ON  ]        │
│  🌡️ Temperature      22.4°C         │
│  🚪 Motion           (3m ago)       │
│                                     │
│  Scenes:                            │
│  [ 🌅 Morning ] [ 📺 Movie ] [ 🌙 ] │
└─────────────────────────────────────┘
```

**Integration via workmeshd:**

- Zigbee coordinator (CC2531, Conbee)
- MQTT bridge
- RF transmitter for legacy devices
- Sensor aggregation

---

## 9. Laboratory Instrument Controller

### The Problem

Research labs have instruments that need:

- Precise parameter control
- Data logging for experiments
- Reproducible configurations
- Safety interlocks
- Sometimes decades-old equipment with RS232

### The Weaver Solution

**Instrument control panel:**

```
┌─────────────────────────────────────┐
│  🔬 Thermal Cycler Control          │
│                                     │
│  Stage: DENATURE (2/30 cycles)      │
│  Temperature: 95.2°C (target: 95°C) │
│  Time remaining: 00:23              │
│                                     │
│  ┌─────────────────────────────┐    │
│  │  ▄▄██▄▄  ▄▄██▄▄  ▄▄██▄▄    │    │
│  │  temperature profile graph  │    │
│  └─────────────────────────────┘    │
│                                     │
│  [ ⏸️ PAUSE ] [ ⏹️ ABORT ]          │
│  ⚠️ Lid locked during operation     │
└─────────────────────────────────────┘
```

**workmeshd capabilities:**

- Serial communication to legacy instruments
- Temperature PID control
- Safety interlock enforcement
- Experiment logging with timestamps

---

## 10. Medical Device Interface

### The Problem

Medical devices (diagnostic equipment, therapy devices, patient monitors) need:

- **Reliable, responsive UI** — clinicians can't wait for loading screens
- **Clear status indication** — is the device ready? running? faulted?
- **Audit logging** — who did what, when, for which patient
- **Offline operation** — hospital networks fail, device must continue
- **Regulatory compliance** — IEC 62304, FDA 21 CFR Part 11

Traditional approaches:

| Approach | Problem |
|----------|---------|
| **Windows Embedded** | Heavy, expensive licensing, update nightmares |
| **Custom embedded** | Expensive development, hard to update |
| **Web-based UI** | Latency, browser crashes, network dependency |
| **Proprietary RTOS** | Vendor lock-in, expensive, limited UI capability |

### The Weaver Solution

**Weaver as medical device HMI:**

- Lightweight, fast boot (critical for emergency equipment)
- Touch-first design (gloved hands, sterile environments)
- Structured logging (IEC 62304 traceability)
- Offline-first (no network dependency for core function)
- Template system for different device contexts

**Example: Infusion Pump Interface**

```
┌─────────────────────────────────────┐
│  💉 Infusion Controller             │
│                                     │
│  Patient: ████████ (ID: 12345)      │
│  Drug: Saline 0.9%                  │
│  Rate: 125 mL/hr                    │
│  Volume Infused: 340 mL             │
│  Volume Remaining: 160 mL           │
│  Time Remaining: 1h 17m             │
│                                     │
│  Status: ✅ RUNNING                 │
│                                     │
│  [ ⏸️ PAUSE ]  [ ⚠️ BOLUS ]  [ ⏹️ ] │
│                                     │
│  ⚠️ Bolus requires confirmation     │
└─────────────────────────────────────┘
```

**Example: Diagnostic Device**

```
┌─────────────────────────────────────┐
│  🔬 Blood Analyzer                  │
│                                     │
│  Sample: Loaded ✅                  │
│  Calibration: Valid (expires 18:00) │
│  Quality Control: PASSED            │
│                                     │
│  [ ▶️ START ANALYSIS ]              │
│                                     │
│  Recent Results:                    │
│  • 14:32 - Sample 4521 - Complete   │
│  • 14:28 - Sample 4520 - Complete   │
│  • 14:15 - Sample 4519 - Complete   │
│                                     │
│  [ 📊 View Results ] [ 🖨️ Print ]   │
└─────────────────────────────────────┘
```

### Regulatory Considerations

**IEC 62304 (Medical Device Software Lifecycle):**

| Class | Risk | Weaver Role | Certification Effort |
|-------|------|-------------|---------------------|
| **Class A** | No injury possible | Full UI + control | Moderate documentation |
| **Class B** | Non-serious injury | UI + control with safeguards | Significant documentation |
| **Class C** | Death or serious injury | UI only, safety in certified subsystem | UI documentation only |

**Recommended architecture for Class B/C:**

```
┌─────────────────────────────────────────────────────────┐
│  Weaver Desktop (Class A software)                      │
│  • User interface only                                  │
│  • Displays status, accepts input                       │
│  • NOT in safety path                                   │
└─────────────────────────────┬───────────────────────────┘
                              │ Commands / Status
┌─────────────────────────────▼───────────────────────────┐
│  workmeshd (Class A/B software)                         │
│  • Command validation                                   │
│  • Audit logging                                        │
│  • Protocol translation                                 │
└─────────────────────────────┬───────────────────────────┘
                              │ Validated commands only
┌─────────────────────────────▼───────────────────────────┐
│  Certified Medical Controller (Class B/C)               │
│  • Actual therapy/measurement control                   │
│  • Hardware safety interlocks                           │
│  • IEC 62304 certified firmware                         │
└─────────────────────────────────────────────────────────┘
```

**Key insight:** Weaver doesn't need to be certified as Class C software if it's only the display layer. The safety-critical logic lives in certified firmware below. This is the same pattern as industrial: **Weaver as HMI, certified controller for safety.**

### FDA 21 CFR Part 11 Compliance

For audit trail requirements:

- [ ] Timestamped, append-only logs
- [ ] Operator identification (login/badge)
- [ ] Electronic signatures for critical actions
- [ ] Log integrity verification (hash chain)
- [ ] Log export for FDA inspection

**workmeshd already plans these features** — they serve both industrial and medical markets.

### Template for Medical Device

```toml
[template]
name = "medical_device"
base = "locked"
high_contrast = true
large_touch_targets = true

[layout]
primary_view = "device_status"
always_visible = ["status_bar", "emergency_stop"]
font_scale = 1.3

[safety]
require_confirmation = ["bolus", "stop", "override"]
timeout_to_lock_sec = 300
require_login = true

[audit]
log_all_touches = true
log_level = "verbose"
include_operator_id = true
sign_logs = true

[display]
high_visibility_colors = true
colorblind_safe = true
min_contrast_ratio = 4.5
```

### Market Positioning

| Segment | Weaver Fit | Notes |
|---------|------------|-------|
| **Point-of-care diagnostics** | ✅ Excellent | Simple UI, audit logging, offline |
| **Therapy devices (Class B)** | ✅ Good | HMI layer, safety in controller |
| **Patient monitors** | ✅ Good | Display + alarm management |
| **Implantable device programmers** | ⚠️ Possible | Would need Class C consideration |
| **Life support equipment** | ❌ No | Use fully certified stack |

---

## 11. Conference Room / Meeting Space Controller

### The Problem

Modern conference rooms have:

- Displays/projectors
- Video conferencing systems
- Lighting control
- Audio systems
- Booking systems

Usually controlled by expensive proprietary systems or confusing remotes.

### The Weaver Solution

**Wall-mounted touch panel:**

```
┌─────────────────────────────────────┐
│  📍 Conference Room B               │
│  Current: Available                 │
│  Next: Team Standup @ 10:00         │
│                                     │
│  [ 🖥️ Presentation Mode ]           │
│  [ 📹 Video Call Mode ]             │
│  [ 💡 Lights: Bright ]              │
│  [ 🔊 Audio: Muted ]                │
│                                     │
│  [ 🔌 All Off & Lock ]              │
└─────────────────────────────────────┘
```

**Presets:**

- "Presentation Mode" → dim lights, projector on, laptop HDMI active
- "Video Call Mode" → front lights bright, screen on, camera active
- "All Off" → everything off, room locked

---

## Architectural Patterns Across Use Cases

### Pattern 1: Panels Abstract Hardware

Every use case benefits from semantic hardware abstraction:

| Use Case | Raw Hardware | Panel Abstraction |
|----------|--------------|-------------------|
| Solar grid | GPIO 17, 18, 22 | Inverter Cluster 2 |
| Cyberdeck | PWM channel 0 | LED Strip Brightness |
| Media center | GPIO 17 | Room Lamp |
| Maker space | I2C 0x40, GPIO 5 | Soldering Station |
| Robot control | UART /dev/ttyACM0 | Left Leg Motor Controller |
| Vehicle dashboard | CAN bus + GPIO | Battery Status + E-Stop |
| Home automation | Zigbee coordinator | Living Room Ceiling Light |
| Lab instrument | RS232 /dev/ttyUSB0 | Thermal Cycler |
| Conference room | IR + relay + HDMI-CEC | Presentation Mode |

### Pattern 2: Templates Define Experience

The template system isn't cosmetic — it's structural:

| Layer | What Templates Control |
|-------|------------------------|
| **Components** | Which UI elements exist at all |
| **Layout** | Where components are placed |
| **Behavior** | How interactions work (touch, remote, keyboard) |
| **Safety** | What requires confirmation, what's hidden |
| **Theme** | Colors, fonts, spacing, scale |

### Pattern 3: workmeshd Owns Authority

Weaver Desktop never:

- Directly accesses GPIO
- Runs privileged operations
- Stores credentials
- Makes safety decisions

workmeshd always:

- Owns hardware abstraction
- Enforces safety rules
- Logs all actions
- Manages authentication

This separation enables:

- Security (small trusted codebase)
- Remote control (same commands, different target)
- Testability (mock workmeshd for UI development)
- Replacement (swap Weaver for different UI, same backend)

---

## Reference Implementation Priority

For demonstrating Weaver's value, implement in this order:

### Phase 1: Prove the Concept

1. **One relay panel** (done perfectly)
   - 230V socket with safety confirmation
   - Live status feedback
   - Duration tracking

2. **One PWM panel** (buttery smooth)
   - LED brightness with <50ms latency
   - Visual slider with numeric readout

3. **One preset button**
   - "All Off" or "Demo Mode"
   - Single tap → multiple state changes

### Phase 2: Validate Templates

4. **Cyberdeck template**
   - Hardware-focused layout
   - Touch-optimized panels

5. **Media center template**
   - Same hardware panels, different presentation
   - Remote-friendly sizing

### Phase 3: Real Deployment

6. **Solar grid demo**
   - Multiple panels with state machines
   - Audit logging
   - Remote monitoring

---

## Success Criteria

A use case is successfully implemented when:

1. **10-second understanding** — New viewer grasps purpose immediately
2. **No GPIO numbers visible** — All hardware is semantic
3. **Safety is obvious** — Dangerous operations look dangerous
4. **Feedback is live** — State changes visible within 100ms
5. **Template switching works** — Same device, different purpose
6. **Logs capture everything** — Full audit trail without configuration

---

*These use cases aren't aspirational — they're the lens through which every Weaver feature is evaluated.*

---

## Weaver in the SCADA Hierarchy

### Understanding the Terminology

SCADA (Supervisory Control and Data Acquisition) isn't a single product—it's an **architecture pattern** with distinct layers:

| Layer | Traditional Name | Function | Traditional Tech | Weaver Equivalent |
|-------|------------------|----------|------------------|-------------------|
| **Level 3** | SCADA Host / MTU | Central supervision, historian, analytics | Ignition, PowerON, Wonderware | WorkMesh Cloud |
| **Level 2** | HMI | Human interface at site | Industrial touchscreens, $2000+ | **Weaver Desktop** |
| **Level 1** | RTU / PLC | Local control, I/O, safety logic | Siemens, Allen-Bradley | **workmeshd** |
| **Level 0** | Field Devices | Sensors, actuators, relays | Standard hardware | GPIO, I2C, SPI, MCUs |

**Key insight:** You're not building *a* SCADA system. You're building **modern, open-source components for Levels 1 and 2** that can operate standalone OR integrate with Level 3 systems.

### The Spectrum of Deployment

```
STANDALONE                                                    FULL SCADA
    │                                                              │
    ▼                                                              ▼
┌────────┐      ┌────────────┐      ┌────────────┐      ┌──────────────┐
│ Single │      │  Multiple  │      │   Mesh     │      │   Cloud      │
│ Node   │ ──►  │   Nodes    │ ──►  │  Network   │ ──►  │  Connected   │
│        │      │  (Local)   │      │            │      │              │
└────────┘      └────────────┘      └────────────┘      └──────────────┘

 Weaver +        Weaver nodes        workmeshd nodes      WorkMesh cloud
 workmeshd       on same LAN         mesh together        provides:
 on one Pi       share state         peer-to-peer         • Central view
                                                          • Historian
                                                          • Remote access
                                                          • Fleet mgmt
```

### Mode 1: Standalone Field Terminal

**What it is:** Single Weaver + workmeshd controlling local hardware.

**Use cases:** Cyberdeck, kiosk, single solar installation, lab instrument.

**Architecture:**

```
┌─────────────────┐
│ Weaver Desktop  │  ← You touch this
└────────┬────────┘
         │
┌────────▼────────┐
│   workmeshd     │  ← This controls hardware
└────────┬────────┘
         │
┌────────▼────────┐
│    Hardware     │  ← Relays, sensors, MCUs
└─────────────────┘
```

**This is NOT SCADA** — it's a local HMI with integrated control. Think "smart thermostat" level.

### Mode 2: Local Mesh (Multi-Node)

**What it is:** Multiple Weaver/workmeshd nodes on same network, sharing state.

**Use cases:** Multi-room home automation, distributed solar installation, maker space with multiple stations.

**Architecture:**

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Node A    │     │   Node B    │     │   Node C    │
│ ┌─────────┐ │     │ ┌─────────┐ │     │ ┌─────────┐ │
│ │ Weaver  │ │     │ │ Weaver  │ │     │ │ Weaver  │ │
│ └────┬────┘ │     │ └────┬────┘ │     │ └────┬────┘ │
│ ┌────▼────┐ │     │ ┌────▼────┐ │     │ ┌────▼────┐ │
│ │workmeshd│◄├─────┼─►workmeshd│◄├─────┼─►workmeshd│ │
│ └────┬────┘ │     │ └────┬────┘ │     │ └────┬────┘ │
│ ┌────▼────┐ │     │ ┌────▼────┐ │     │ ┌────▼────┐ │
│ │Hardware │ │     │ │Hardware │ │     │ │Hardware │ │
│ └─────────┘ │     │ └─────────┘ │     │ └─────────┘ │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                   │
       └───────────────────┴───────────────────┘
              Peer-to-peer state sync
              (mDNS discovery, mesh protocol)
```

**This is approaching SCADA** — distributed control with coordination. But still no central authority.

### Mode 3: Cloud-Connected (Full SCADA)

**What it is:** Field nodes connect to WorkMesh cloud for central oversight.

**Use cases:** Fleet management, remote monitoring, enterprise deployment, regulatory compliance.

**Architecture:**

```
                    ┌─────────────────────────────────────┐
                    │          WorkMesh Cloud             │
                    │                                     │
                    │  • Central dashboard (fleet view)   │
                    │  • Historian (long-term storage)    │
                    │  • Alerting and notifications       │
                    │  • User management                  │
                    │  • Remote command relay             │
                    │  • Audit log aggregation            │
                    │  • Firmware update distribution     │
                    └──────────────────┬──────────────────┘
                                       │
                         ┌─────────────┼─────────────┐
                         │             │             │
                         ▼             ▼             ▼
                    ┌─────────┐   ┌─────────┐   ┌─────────┐
                    │ Site A  │   │ Site B  │   │ Site C  │
                    │ Weaver  │   │ Weaver  │   │ Weaver  │
                    │workmeshd│   │workmeshd│   │workmeshd│
                    │Hardware │   │Hardware │   │Hardware │
                    └─────────┘   └─────────┘   └─────────┘
                         │             │             │
                    (offline-capable, sync when connected)
```

**This IS SCADA** — supervisory control, data acquisition, central authority. Just built with modern, lightweight, open components instead of proprietary stacks.

### What Makes This Different from Traditional SCADA

| Aspect | Traditional SCADA | Weaver/WorkMesh Stack |
|--------|-------------------|----------------------|
| **Edge hardware** | $2000+ industrial PCs | $50 Raspberry Pi |
| **Licensing** | Per-seat, per-tag, expensive | Open source + cloud subscription |
| **Offline operation** | Often limited | First-class citizen |
| **Deployment** | Vendor professional services | Self-deployable |
| **Customization** | Limited, vendor-dependent | Full source access |
| **Update cycle** | Slow, expensive | Continuous, controlled |
| **Security model** | Often outdated | Modern (TLS, auth, audit) |

### The Business Model Connection

This hierarchy maps directly to revenue:

| Mode | Customer | Revenue Model |
|------|----------|---------------|
| **Standalone** | Makers, hobbyists, small deployments | Free / donations |
| **Local Mesh** | Professional installers, small business | Support contracts |
| **Cloud-Connected** | Enterprise, fleet operators, regulated industries | WorkMesh SaaS subscription |

**Weaver Desktop is free.** It drives adoption.
**WorkMesh Cloud is the product.** It's where complexity (historian, fleet management, compliance) lives.

### Where Certification Matters

```
                        ┌─────────────────┐
                        │ WorkMesh Cloud  │  ← SOC 2, ISO 27001
                        │ (your infra)    │     (trust & compliance)
                        └────────┬────────┘
                                 │
              ┌──────────────────┼──────────────────┐
              ▼                  ▼                  ▼
        ┌───────────┐      ┌───────────┐      ┌───────────┐
        │  Weaver   │      │  Weaver   │      │  Weaver   │  ← IEC 62443
        │  Desktop  │      │  Desktop  │      │  Desktop  │     (optional)
        └─────┬─────┘      └─────┬─────┘      └─────┬─────┘
              │                  │                  │
        ┌─────▼─────┐      ┌─────▼─────┐      ┌─────▼─────┐
        │ workmeshd │      │ workmeshd │      │ workmeshd │  ← IEC 62443
        └─────┬─────┘      └─────┬─────┘      └─────┬─────┘     (if needed)
              │                  │                  │
        ┌─────▼─────┐      ┌─────▼─────┐      ┌─────▼─────┐
        │ Hardware  │      │ Hardware  │      │ Hardware  │  ← CE, UL
        │ (safety)  │      │ (safety)  │      │ (safety)  │     (for HW)
        └───────────┘      └───────────┘      └───────────┘
```

**For most deployments:** No certification needed at Weaver level. CE-marked hardware + good engineering practices.

**For regulated industries:** Certify workmeshd + hardware integration. Weaver stays as "display layer" outside certification scope.

**For medical:** See Use Case 10. Same pattern—HMI outside certification, safety logic in certified controller.

### Summary: Yes, You're Building SCADA

But you're building it **from the bottom up**:

1. **Now:** Weaver Desktop — the touchscreen HMI ✅
2. **Next:** workmeshd — the local control daemon 🔜
3. **Then:** Mesh networking — peer-to-peer coordination
4. **Later:** WorkMesh Cloud — central supervision and revenue

Traditional SCADA vendors sell top-down: "Here's our platform, buy our terminals."
You're selling bottom-up: "Here's a great field terminal. Need more? Here's mesh. Need central control? Here's cloud."

**The field terminal is the wedge.** It gets deployed because it's good, affordable, and works offline. Once it's deployed, the upgrade path to full SCADA is natural.
