# Weaver Desktop: Industrial Roadmap

> **Purpose:** Document the path from MVP to industrial-grade deployment. This is the north star for reliability, not the immediate priority.

**Created:** December 2025  
**Status:** Vision document — revisit after MVP ships

---

## Strategic Position: Bottom-Up SCADA

**We are not competing with SCADA. We are building SCADA from the bottom up.**

Traditional SCADA vendors sell **top-down**: central platform first, then push expensive terminals to the edge. You buy their ecosystem or nothing.

Weaver/WorkMesh builds **bottom-up**: excellent field terminal first, then optional mesh coordination, then optional cloud supervision. Each layer works independently; each layer adds value.

```
TRADITIONAL SCADA (Top-Down)              WEAVER/WORKMESH (Bottom-Up)
                                          
┌─────────────────────┐                   ┌─────────────────────┐
│   Central Platform  │  ◄── Buy this    │   WorkMesh Cloud    │  ◄── Phase 4
│   ($50K-500K+)      │      first       │   (SaaS revenue)    │      (later)
└──────────┬──────────┘                   └──────────┬──────────┘
           │                                         │
           │ Forces you to buy                       │ Optional upgrade
           │ expensive terminals                     │ when you need it
           ▼                                         ▼
┌─────────────────────┐                   ┌─────────────────────┐
│   HMI Terminals     │                   │   Mesh Network      │  ◄── Phase 3
│   ($2K-5K each)     │                   │   (peer-to-peer)    │
└─────────────────────┘                   └──────────┬──────────┘
                                                     │
                                                     │ Optional coordination
                                                     │ when you have multiple nodes
                                                     ▼
                                          ┌─────────────────────┐
                                          │   workmeshd         │  ◄── Phase 2
                                          │   (local daemon)    │
                                          └──────────┬──────────┘
                                                     │
                                                     │ Required
                                                     ▼
                                          ┌─────────────────────┐
                                          │   Weaver Desktop    │  ◄── Phase 1
                                          │   (field terminal)  │      (NOW)
                                          └─────────────────────┘
```

### The Wedge Strategy

**Phase 1: Weaver Desktop** (free, open source)

- Excellent field terminal that works standalone
- Deployed because it's good, affordable, and offline-capable
- Builds user base and market presence

**Phase 2: workmeshd** (free, open source)

- Local control daemon with hardware abstraction
- Audit logging, state machines, safety rules
- Required for Weaver, but also usable headless

**Phase 3: Mesh Networking** (free, open source)

- Peer-to-peer coordination between nodes
- State synchronization, failover, load balancing
- Valuable for multi-node installations

**Phase 4: WorkMesh Cloud** (SaaS subscription)

- Central supervision, historian, fleet management
- Remote access, compliance reporting, user management
- **This is where revenue lives**

### Why This Works

| Advantage | How It Helps |
|-----------|--------------|
| **Low barrier to entry** | Try Weaver for free, on a $50 Pi |
| **Prove value first** | Customer uses it, loves it, then upgrades |
| **Offline-first** | Works without cloud; cloud adds value, not dependency |
| **Open source trust** | No vendor lock-in; you can always run without cloud |
| **Natural expansion** | One node → multiple nodes → central oversight |

### What This Means for Industrial

At the industrial level, Weaver + workmeshd + WorkMesh Cloud **is** a SCADA system:

| SCADA Layer | Traditional | Weaver Stack |
|-------------|-------------|--------------|
| **Level 3: Supervision** | Ignition, PowerON, Wonderware | WorkMesh Cloud |
| **Level 2: HMI** | Industrial touchscreens ($2K+) | Weaver Desktop ($50) |
| **Level 1: Control** | PLCs, RTUs ($500-5K) | workmeshd + MCUs |
| **Level 0: Field** | Sensors, actuators | Standard hardware |

We're building Levels 1-2 in open source, Level 3 as SaaS. The customer gets a complete SCADA stack without the traditional vendor lock-in or upfront cost.

> **See also:** [USE_CASES.md](./USE_CASES.md#weaver-in-the-scada-hierarchy) for detailed SCADA hierarchy documentation.

---

## The Industrial Ladder

Software reliability exists on a spectrum. Understanding where Weaver sits today and where it can go helps prioritize the right features at the right time.

| Level | Description | Certification | Examples |
|-------|-------------|---------------|----------|
| **Maker/Hobbyist** | Works, looks good, personal use | None | Arduino projects, personal cyberdecks |
| **Professional** | Reliable, auditable, deployed to clients | None (contractual) | Small solar installations, custom kiosks |
| **Industrial** | Certified, redundant, documented compliance | IEC 62443, CE marking | Factory HMIs, building automation |
| **Safety-Critical** | Formally verified, redundant, fail-safe | IEC 61508 (SIL), DO-178C | Nuclear controls, railway signaling, medical devices, spacecraft |

### Weaver's Target Zone

```
                                    ┌─────────────────────┐
                                    │   Safety-Critical   │  ← Not our market
                                    │  (SIL 2-4, DO-178C) │    (dedicated hardware,
                                    └──────────┬──────────┘     formal methods)
                                               │
                              ┌────────────────▼────────────────┐
                              │           Industrial            │  ← Achievable goal
                              │    (IEC 62443, documented)      │    (Phase 2-3)
                              └────────────────┬────────────────┘
                                               │
                    ┌──────────────────────────▼──────────────────────────┐
                    │                    Professional                     │  ← Near-term target
                    │          (reliable, auditable, deployed)            │    (MVP++)
                    └──────────────────────────┬──────────────────────────┘
                                               │
        ┌──────────────────────────────────────▼──────────────────────────────────────┐
        │                              Maker/Hobbyist                                 │  ← Today
        │                        (works, demonstrates concept)                        │
        └─────────────────────────────────────────────────────────────────────────────┘
```

---

## What "Industrial" Actually Means

Industrial systems must be **predictable**, **auditable**, and **recoverable**. This isn't about fancy features — it's about trust.

### 1. Reliability & Fault Tolerance

The system must handle failures gracefully, not catastrophically.

| Requirement | Description | Implementation |
|-------------|-------------|----------------|
| **Watchdog timers** | Restart frozen processes automatically | systemd watchdog for workmeshd |
| **Graceful degradation** | UI works even if daemon unreachable | "OFFLINE" mode with last-known state |
| **Connection monitoring** | Clear indication when links are down | Visual "LINK DOWN" banner, not silent failure |
| **Automatic reconnection** | Recover from transient failures | Exponential backoff, state sync on reconnect |
| **Predictable memory** | No OOM crashes, bounded allocations | Memory limits, arena allocators if needed |

### 2. Audit & Compliance

Every action must be traceable to a person, time, and result.

| Requirement | Description | Implementation |
|-------------|-------------|----------------|
| **Structured logging** | Machine-parseable event records | JSON logs with schema |
| **Tamper evidence** | Logs cannot be silently modified | Hash chain (each entry includes hash of previous) |
| **User identity** | Who performed each action? | Operator ID per terminal, PIN or badge auth |
| **Role-based access** | Operator vs. Admin vs. Viewer | Capability sets per role |
| **Log export** | Auditors can review history | CSV/JSON export, date range filtering |

**Example audit log entry:**

```json
{
  "seq": 14523,
  "prev_hash": "a3f2c1...",
  "timestamp": "2025-12-14T14:32:08.442Z",
  "operator": "deck-01",
  "action": "panel.enable",
  "target": "inverter_cluster_2",
  "from_state": "ARMED",
  "to_state": "ON",
  "result": "OK",
  "hash": "b7e4d2..."
}
```

### 3. Alarm Management

Industrial systems distinguish between normal events and conditions requiring attention.

| Priority | Visual | Audio | Acknowledgment |
|----------|--------|-------|----------------|
| **Critical** | Red, flashing | Continuous tone | Required before clearing |
| **Warning** | Orange, solid | Single beep | Optional |
| **Info** | Blue/Gray | None | None |

**Alarm workflow:**

```
Event Occurs → Alarm Raised → Operator Acknowledges → Condition Clears → Alarm Clears
                    │                   │                    │
                    ▼                   ▼                    ▼
              Logged with          Logged with          Logged with
              timestamp            operator ID          duration
```

### 4. Deterministic Behavior

No surprises during operation.

| Requirement | Description |
|-------------|-------------|
| **No auto-updates** | Updates only during maintenance windows |
| **Defined edge cases** | Every error has documented behavior |
| **Bounded response time** | UI responds within X ms, always |
| **State persistence** | Survives power loss, resumes correctly |

### 5. Documentation & Traceability

For certification, you need paper trails.

| Document | Purpose |
|----------|---------|
| **System Design Document** | Architecture, data flows, security boundaries |
| **Test Evidence** | Test cases, results, coverage metrics |
| **Change Log** | Every change with rationale and approval |
| **Failure Mode Analysis** | What can go wrong, what happens when it does |
| **Operator Manual** | How to use the system safely |

---

## The Architecture That Enables This

Weaver's GUI/daemon separation is **correct for industrial use**:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Weaver Desktop (GUI)                        │
│                                                                 │
│  • Unprivileged process                                         │
│  • Can crash without losing hardware state                      │
│  • Displays what daemon tells it                                │
│  • Sends commands, doesn't execute them directly                │
└─────────────────────────────┬───────────────────────────────────┘
                              │ Unix socket / IPC
                              │ (well-defined protocol)
┌─────────────────────────────▼───────────────────────────────────┐
│                         workmeshd                               │
│                                                                 │
│  • Owns all hardware access                                     │
│  • Enforces state machine rules                                 │
│  • Validates every command                                      │
│  • Logs every action (audit trail)                              │
│  • Runs as privileged daemon (systemd managed)                  │
│  • Watchdog enabled — restarts if frozen                        │
└─────────────────────────────┬───────────────────────────────────┘
                              │ GPIO / SPI / I2C / USB
┌─────────────────────────────▼───────────────────────────────────┐
│                       Hardware Layer                            │
│                                                                 │
│  • Relays, sensors, MCUs                                        │
│  • For hard real-time: offload to RP2040/STM32                  │
│  • Fail-safe wiring (relay de-energized = safe state)           │
└─────────────────────────────────────────────────────────────────┘
```

**Why this matters:**

1. **GUI bugs can't break hardware** — daemon validates everything
2. **Audit happens at daemon level** — UI can't bypass logging
3. **Security boundary is clear** — socket protocol is the API
4. **Testable in isolation** — daemon can run headless for testing

---

## Phased Implementation

### Phase 1: Professional (MVP++)

*Target: Deployable to client solar installations with confidence.*

- [ ] Connection status indicator (daemon reachable? yes/no/reconnecting)
- [ ] Structured JSON logging in workmeshd
- [ ] Error states visible in UI (specific messages, not generic errors)
- [ ] Watchdog configuration for workmeshd (systemd)
- [ ] Last-known-state display when disconnected
- [ ] Basic operator identification (deck ID in logs)

**Exit criteria:** Colleague can deploy on solar installation, logs are reviewable, system recovers from crashes automatically.

### Phase 2: Industrial-Ready

*Target: Suitable for commercial deployments with compliance requirements.*

- [ ] User authentication (PIN per operator, or badge reader)
- [ ] Role-based access control (viewer / operator / admin)
- [ ] Alarm widget with prioritization and acknowledgment
- [ ] Hash-chained audit logs (tamper-evident)
- [ ] Log export (CSV/JSON with date filtering)
- [ ] Documented failure modes for each component
- [ ] Connection to external SCADA via MQTT/Modbus TCP

**Exit criteria:** Passes informal security review, logs acceptable to auditors, integrates with larger systems.

### Phase 3: Certifiable

*Target: Formal certification for regulated industries (if market demands).*

- [ ] IEC 62443 cybersecurity baseline compliance
- [ ] Formal test suite with coverage metrics (>80%)
- [ ] Hardware qualification document (specific Pi model + peripherals)
- [ ] Third-party security audit
- [ ] Penetration testing
- [ ] Formal operator training materials
- [ ] CE marking for hardware bundle (if selling complete units)

**Exit criteria:** Passes third-party audit, documentation complete for regulatory submission.

---

## Weaver's Position in Industrial Ecosystems

Weaver is not competing with full SCADA platforms — it's the foundation for building one from the ground up.

### What Weaver Is (Today)

| Role | Description |
|------|-------------|
| **Field Terminal** | Human interface at the edge, where operators physically are |
| **Local HMI** | Control surface for a single site or cluster |
| **Standalone Controller** | For small installations that don't need central SCADA |

### What Weaver Becomes (With WorkMesh)

| Role | Description |
|------|-------------|
| **SCADA Edge Node** | Field terminal that reports to central WorkMesh Cloud |
| **Remote HMI** | Controllable from anywhere via WorkMesh |
| **Fleet Member** | One of many nodes managed centrally |

### What Weaver Is Not (And Won't Be)

| Role | Why Not |
|------|---------|
| **Monolithic SCADA Platform** | We build bottom-up, not top-down |
| **Safety PLC** | Use certified PLCs (Siemens, Allen-Bradley) for safety interlocks |
| **Hard Real-Time Controller** | For microsecond timing, use dedicated MCUs |

### Example: Distributed Solar Installation

```
┌─────────────────────────────────────────────────────────────────┐
│                    Central SCADA (optional)                     │
│              (Ignition, PowerON, or custom system)              │
│                                                                 │
│  • Historian (long-term data storage)                           │
│  • Fleet overview (all sites on one map)                        │
│  • Reporting and analytics                                      │
└─────────────────────────────┬───────────────────────────────────┘
                              │ MQTT / Modbus TCP / REST
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
   ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
   │   Site A    │     │   Site B    │     │   Site C    │
   │             │     │             │     │             │
   │ ┌─────────┐ │     │ ┌─────────┐ │     │ ┌─────────┐ │
   │ │ Weaver  │ │     │ │ Weaver  │ │     │ │ Weaver  │ │
   │ │ Desktop │ │     │ │ Desktop │ │     │ │ Desktop │ │
   │ └────┬────┘ │     │ └────┬────┘ │     │ └────┬────┘ │
   │      │      │     │      │      │     │      │      │
   │ ┌────▼────┐ │     │ ┌────▼────┐ │     │ ┌────▼────┐ │
   │ │workmeshd│ │     │ │workmeshd│ │     │ │workmeshd│ │
   │ └────┬────┘ │     │ └────┬────┘ │     │ └────┬────┘ │
   │      │      │     │      │      │     │      │      │
   │ ┌────▼────┐ │     │ ┌────▼────┐ │     │ ┌────▼────┐ │
   │ │ Relays  │ │     │ │ Relays  │ │     │ │ Relays  │ │
   │ │ Sensors │ │     │ │ Sensors │ │     │ │ Sensors │ │
   │ └─────────┘ │     │ └─────────┘ │     │ └─────────┘ │
   └─────────────┘     └─────────────┘     └─────────────┘
```

**Weaver's value:** The operator at Site A has a local, responsive, offline-capable interface. They don't need internet to control their site. Central SCADA gets data when connectivity exists, but the site operates independently.

---

## Real-Time Considerations

### What Weaver Needs (Soft Real-Time)

| Requirement | Target | How |
|-------------|--------|-----|
| UI responsiveness | 60fps (16ms frames) | egui is fast enough, standard Linux is fine |
| Command response | <100ms | Unix socket IPC, trivial |
| Sensor polling | 1-10 Hz | Standard Linux timers |
| Relay switching | <50ms | GPIO is effectively instant |

**Standard Debian on Raspberry Pi achieves all of these.**

### Optional Hardening (PREEMPT_RT)

For tighter latency bounds (reducing worst-case from ~200ms to ~200μs):

```bash
# Install RT kernel (Raspberry Pi OS)
sudo apt install linux-image-rt-arm64

# Isolate a CPU core for workmeshd
# Add to /boot/cmdline.txt: isolcpus=3

# Run workmeshd with real-time priority
sudo chrt -f 50 ./workmeshd
```

**When to bother:** Only if you observe latency spikes causing issues. For relay control and monitoring, you won't.

### Hard Real-Time (If Ever Needed)

For microsecond-precision timing (motor control, precise waveform generation):

- Offload to **RP2040** (Raspberry Pi Pico) connected via USB/SPI
- workmeshd sends high-level commands, MCU handles timing
- This is the industrial pattern — Linux for UI/logic, MCU for timing-critical I/O

---

## Comparison: Weaver vs. Industrial HMIs

| Aspect | Traditional Industrial HMI | Weaver Desktop |
|--------|---------------------------|----------------|
| **Platform** | Windows CE, proprietary RTOS | Linux (Debian) |
| **Hardware** | Purpose-built industrial PC ($500-2000) | Raspberry Pi ($35-75) |
| **Licensing** | Per-seat, expensive | Open source |
| **Connectivity** | OPC-UA, Modbus, proprietary | MQTT, Modbus TCP, Unix sockets |
| **Certification** | Pre-certified | Self-certify or third-party |
| **Customization** | Limited, vendor-dependent | Full source access |
| **Offline operation** | Usually yes | Yes (core design principle) |
| **Touch interface** | Yes | Yes |
| **Update mechanism** | Vendor-controlled | Your control |

**Weaver's advantage:** Cost, flexibility, ownership. You're not locked into a vendor.

**Weaver's disadvantage:** You do the work that vendors have already done (but you own the result).

---

## The Top of the Iceberg: Safety-Critical

For completeness, here's what safety-critical (SIL 2-4, DO-178C) requires. This is **not Weaver's target market**, but understanding it clarifies the boundary.

| Requirement | What It Means | Cost |
|-------------|---------------|------|
| **Formal verification** | Mathematical proof of correctness | $$$, specialized tools |
| **Certified toolchain** | Compiler, linker, OS all certified | Proprietary, expensive |
| **Hardware redundancy** | Triple modular redundancy, voting | 3x hardware cost |
| **Independent verification** | Third-party reviews everything | Months of audits |
| **Failure mode coverage** | Every possible failure analyzed and documented | Massive documentation |
| **Change control** | Any change requires full re-certification | Slow, expensive |

**Total cost:** $100K-$10M+ depending on SIL level and complexity.

**Who does this:** Aerospace, nuclear, rail, medical device companies with dedicated safety teams.

**Weaver's role in safety-critical systems:** The non-safety HMI layer. The actual safety interlocks run on certified PLCs. Weaver shows status and allows operation, but the PLC prevents unsafe states regardless of what Weaver does.

---

## Extreme Constraint Challenge: Lichee Pi Zero

> **Status:** Post-MVP stress test — proving Weaver can run on absolute minimum hardware.

### The Challenge

If Weaver Desktop runs on a **Lichee Pi Zero**, it runs anywhere. This is the ultimate proof that our "lightweight" claim is real.

### Target Hardware: Lichee Pi Zero

| Spec | Value |
|------|-------|
| **SoC** | Allwinner V3s |
| **CPU** | ARM Cortex-A7 @ 1.2GHz (single core) |
| **RAM** | 64MB DDR2 (built into SoC!) |
| **Storage** | MicroSD, optional 16MB SPI flash |
| **Display** | RGB LCD interface (direct panel connection) |
| **Size** | 45×26mm (smaller than a credit card) |
| **Price** | ~$6-8 |
| **Power** | MicroUSB or 5V pins |

**Key constraint:** 64MB total RAM shared between kernel, userspace, and framebuffer.

### Why This Matters

| Comparison | RAM | Price | Weaver? |
|------------|-----|-------|---------|
| Traditional HMI Panel | 2-4GB | $2,000-5,000 | Overkill |
| Raspberry Pi 4 | 1-8GB | $35-80 | Comfortable |
| Raspberry Pi Zero | 512MB | $5-15 | Current target |
| **Lichee Pi Zero** | **64MB** | **$6-8** | **The challenge** |

If we achieve this, we can honestly claim: **"Weaver runs on $6 hardware with 64MB RAM."**

No other desktop environment can make that claim.

### Memory Budget (64MB Target)

```
64MB Total
├── Linux kernel (minimal)     ~8-12MB
├── Essential services         ~4-6MB
├── Framebuffer (800×480 RGB)  ~2-3MB
├── workmeshd (headless)       ~8-12MB
├── Weaver Desktop             ~20-30MB (stretch goal)
└── Headroom                   ~10-20MB
```

**Requirements for success:**

- Buildroot or custom minimal Linux (not full distro)
- Direct framebuffer rendering (no compositor)
- Aggressive binary size optimization (`opt-level = "z"`, LTO, strip)
- Lazy loading of UI components
- Minimal font subset

### Reference Hardware Setup

**Mini "laptop" configuration (~$25 total):**

| Component | Price | Notes |
|-----------|-------|-------|
| Lichee Pi Zero | ~$6-8 | Main board |
| 5" 800×480 LCD | ~$14 | Direct RGB connection |
| Li-ion battery | ~$4-5 | Optional, for portable |
| USB-TTL adapter | ~$3 | For development/debug |

**Assembly:** LCD connects directly to Lichee Pi Zero's RGB interface — no HDMI adapter needed. Board can be mounted on back of display.

**Reference build guide:** <https://www.instructables.com/Mini-laptop-Made-by-Lichee-Pi-Zero/>

### Available Linux Images

| Image | Contents | RAM Usage |
|-------|----------|-----------|
| `brmin_dd.tar.bz2` | Minimum Buildroot | Lowest |
| `minmin_dd.tar.bz2` | Minimum Debian (apt only) | Low |
| `minX_dd.tar.bz2` | Minimum + Xorg | Medium |
| `brpy_dd.tar.bz2` | Buildroot + Python | Medium |

**Recommended starting point:** `brmin_dd` (minimum Buildroot) or custom Buildroot.

### Development Resources

- **Documentation:** <https://www.licheepizero.us/>
- **Kernel source:** <https://github.com/Lichee-Pi/linux>
- **Hardware wiki:** <http://linux-sunxi.org/LicheePi_Zero>
- **Community forum:** <https://en.bbs.sipeed.com/c/lichee-pi-zero>
- **Schematics:** Available on licheepizero.us

### Success Criteria

| Milestone | Description |
|-----------|-------------|
| **Boot** | Weaver launches on Lichee Pi Zero |
| **Render** | UI displays on 800×480 LCD |
| **Interact** | Touch input works |
| **Stable** | Runs for 24h without crash/OOM |
| **Usable** | Can control a simple panel (GPIO relay) |

### When to Attempt This

**Prerequisites:**

1. ✅ MVP shipped on Raspberry Pi
2. ✅ Framebuffer backend working
3. ✅ Memory profiling tools in place
4. ✅ Binary size optimizations documented

**Timeline:** Post-first-release milestone. This is a stress test, not a blocker.

### The Payoff

Successfully running on Lichee Pi Zero proves:

- Weaver is genuinely lightweight, not "lightweight for a desktop"
- Industrial deployments can use absolute minimum hardware
- Cost per node drops to single digits
- Embedded/kiosk market becomes accessible

**Marketing claim unlocked:** *"The only desktop environment that runs on $6 hardware."*

---

## Summary

### Reliability Ladder

| Level | Weaver Status | Path |
|-------|---------------|------|
| Maker/Hobbyist | ✅ Today | — |
| Professional | 🟡 MVP++ | Logging, error handling, watchdogs |
| Industrial | 🎯 Achievable | Auth, alarms, audit, documentation |
| Safety-Critical | ⛔ Not target | Partner with certified PLCs |

### Bottom-Up SCADA Stack

| Component | Status | Revenue |
|-----------|--------|---------|
| Weaver Desktop (HMI) | 🔨 Building now | Free / open source |
| workmeshd (control) | 📋 Planned | Free / open source |
| Mesh networking | 📋 Future | Free / open source |
| WorkMesh Cloud (supervision) | 📋 Future | **SaaS subscription** |

### The Vision

Weaver Desktop becomes the go-to HMI for Raspberry Pi-based industrial and embedded applications — not by competing with big SCADA vendors, but by building SCADA from the bottom up.

**Traditional vendors say:** "Buy our platform, then buy our terminals."

**We say:** "Use our free terminal. Love it? Add mesh. Need central control? Subscribe to cloud."

The field terminal is the wedge. It gets deployed because it's good, affordable, and works offline. Once deployed, the upgrade path to full SCADA is natural — and that's where revenue lives.

**Next step:** Ship the MVP. Real deployments generate real requirements. Industrial features follow demand, not speculation.

---

*"Make it work, make it right, make it fast — in that order."*
