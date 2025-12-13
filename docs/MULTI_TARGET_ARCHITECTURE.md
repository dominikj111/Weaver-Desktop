# Multi-Target Architecture

**Transparent Remote Control for SystemWeaver/Flow**

This document describes the architecture for controlling multiple machines (local and remote) through a single desktop environment instance, with seamless user experience regardless of which target is active.

---

## Overview

SystemWeaver operates as a **thin client** that renders locally but executes operations on the **active target**. The user can switch between targets (local machine, remote servers, embedded devices) and the experience remains identical—as if sitting directly in front of that machine.

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Master Machine                                  │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                   SystemWeaver DE (egui)                      │  │
│  │                                                               │  │
│  │   • Renders all UI locally                                    │  │
│  │   • Icons, themes, assets from local resources                │  │
│  │   • Dispatches commands to active target                      │  │
│  │   • Receives data/responses for display                       │  │
│  │                                                               │  │
│  │   ┌─────────────────────────────────────────────────────────┐ │  │
│  │   │  Target Selector: [Local] [Server A] [Raspberry Pi]     │ │  │
│  │   └─────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    Active Target Connection                   │  │
│  │         (local workmeshd OR remote workmeshd)                 │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                               │
          ┌────────────────────┼────────────────────┐
          ▼                    ▼                    ▼
   ┌─────────────┐      ┌─────────────┐      ┌─────────────┐
   │   Local     │      │  Server A   │      │ Raspberry   │
   │  workmeshd  │      │  workmeshd  │      │ Pi workmeshd│
   │             │      │             │      │             │
   │  • PTY      │      │  • PTY      │      │  • PTY      │
   │  • Files    │      │  • Files    │      │  • Files    │
   │  • Services │      │  • Services │      │  • GPIO     │
   │  • Packages │      │  • Packages │      │  • Services │
   └─────────────┘      └─────────────┘      └─────────────┘
```

---

## Core Principles

### 1. Transparent Operation

When the user opens a terminal, file browser, or runs any command:

- **No explicit SSH** - The connection is already established
- **No extra authentication** - Session is pre-authenticated
- **Same behavior as local** - User cannot distinguish local from remote

### 2. Data Flow Only

Between the DE and targets, only **data/commands** flow—never visual rendering:

- Commands: "open terminal", "list directory /home", "install package X"
- Responses: PTY output stream, file listings, operation results
- The DE renders everything locally using local resources (icons, themes, fonts)

### 3. Target Abstraction

All DE operations go through a **Target** abstraction:

```rust
pub trait Target {
    /// Open a PTY session, returns stream handle
    fn open_pty(&self) -> Result<PtyStream>;

    /// Execute a command, return output
    fn exec(&self, cmd: &str) -> Result<CommandOutput>;

    /// List directory contents
    fn list_dir(&self, path: &Path) -> Result<Vec<DirEntry>>;

    /// Read file contents
    fn read_file(&self, path: &Path) -> Result<Vec<u8>>;

    /// Write file contents
    fn write_file(&self, path: &Path, data: &[u8]) -> Result<()>;

    /// Service control
    fn service_control(&self, name: &str, action: ServiceAction) -> Result<()>;

    /// Package management
    fn install_package(&self, name: &str) -> Result<()>;

    // ... other operations
}
```

Two implementations:

- `LocalTarget` - Communicates with local `workmeshd` via Unix socket
- `RemoteTarget` - Communicates with remote `workmeshd` via network

---

## Connection & Transport

### Network Options

| Transport           | Use Case                | Notes                                  |
| ------------------- | ----------------------- | -------------------------------------- |
| **Unix Socket**     | Local target            | Fast, no network overhead              |
| **TCP/IP (LAN)**    | Trusted local network   | Simple, low latency                    |
| **UDP**             | Real-time PTY streaming | Lower latency, needs reliability layer |
| **TLS over TCP**    | Untrusted networks      | Encrypted, authenticated               |
| **P2P (encrypted)** | NAT traversal, mesh     | For distributed/mobile scenarios       |

### Protocol

The DE ↔ workmeshd protocol carries:

```
┌─────────────────────────────────────────────────────────────┐
│                      Message Frame                          │
├─────────────────────────────────────────────────────────────┤
│  Header: message_type, sequence_id, payload_length          │
│  Payload: command-specific data (msgpack/protobuf/custom)   │
└─────────────────────────────────────────────────────────────┘
```

Message types:

- `PtyOpen`, `PtyData`, `PtyResize`, `PtyClose`
- `FsListDir`, `FsReadFile`, `FsWriteFile`, `FsDelete`
- `ServiceList`, `ServiceStart`, `ServiceStop`, `ServiceStatus`
- `PackageInstall`, `PackageRemove`, `PackageList`
- `ExecCommand`, `ExecResult`
- `SystemInfo`, `HardwareControl`

---

## Terminal (PTY) Handling

### Transparent Remote Terminal

When user opens terminal on a remote target:

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  egui_term   │ ──▶ │   Target     │ ──▶ │  workmeshd   │
│  (renders)   │     │  (active)    │     │  (remote)    │
│              │ ◀── │              │ ◀── │              │
│  pty_output  │     │  PtyData     │     │  /bin/bash   │
└──────────────┘     └──────────────┘     └──────────────┘
```

1. User clicks "Terminal" in DE
2. DE calls `active_target.open_pty()`
3. If remote: workmeshd spawns shell, streams PTY I/O over network
4. egui_term receives and renders output locally
5. Keystrokes sent back through same channel

**User experience**: Identical to local terminal. No SSH prompt, no login, just a shell.

### PTY Stream Protocol

```
PtyOpen  { cols: u16, rows: u16, shell: Option<String> } → PtyHandle
PtyData  { handle: PtyHandle, data: Vec<u8> }            → bidirectional
PtyResize{ handle: PtyHandle, cols: u16, rows: u16 }     → ()
PtyClose { handle: PtyHandle }                           → ()
```

---

## Authentication & Security

### Session Authentication

Authentication happens **once** when connecting to a target, not per-operation:

```
┌─────────────┐                              ┌─────────────┐
│     DE      │                              │  workmeshd  │
│             │  ──── TLS Handshake ────▶    │   (remote)  │
│             │  ◀─── Certificate ──────     │             │
│             │                              │             │
│             │  ──── Auth Request ─────▶    │             │
│             │       (token/cert/creds)     │             │
│             │  ◀─── Session Granted ──     │             │
│             │       (session_id)           │             │
│             │                              │             │
│   [SESSION ESTABLISHED - NO MORE AUTH]    │             │
│             │                              │             │
│             │  ──── Commands ─────────▶    │             │
│             │  ◀─── Responses ────────     │             │
└─────────────┘                              └─────────────┘
```

### Authentication Methods

| Method                  | Use Case                | Notes                                 |
| ----------------------- | ----------------------- | ------------------------------------- |
| **mTLS (certificates)** | Machine-to-machine      | Pre-shared certs, no user interaction |
| **Token-based**         | Pre-authorized sessions | Token provisioned during setup        |
| **PAM passthrough**     | User credentials        | One-time login, then session          |

### Privilege Escalation (sudo)

Since workmeshd runs with elevated privileges:

- Most operations don't need sudo—workmeshd already has access
- For user-context operations, workmeshd can impersonate users
- If sudo is needed in terminal, it's handled normally (user types password in PTY)

### Security Model

```
┌─────────────────────────────────────────────────────────────┐
│                    Security Boundaries                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  DE (unprivileged)                                          │
│    │                                                        │
│    │ authenticated session                                  │
│    ▼                                                        │
│  workmeshd (privileged)                                     │
│    │                                                        │
│    ├── validates all requests                               │
│    ├── enforces access control                              │
│    ├── audits operations                                    │
│    └── can restrict per-user/per-session                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Target Switching

### User Experience

```
┌─────────────────────────────────────────────────────────────┐
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │  Local  │  │ Server  │  │  Pi TV  │  │Cyberdeck│        │
│  │   ●     │  │         │  │         │  │         │        │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │
│       ▲                                                     │
│       │ active                                              │
│       │                                                     │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                                                         ││
│  │   Terminal, File Browser, Settings, etc.                ││
│  │   All operations go to: Local                           ││
│  │                                                         ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

Clicking another target:

1. Current target's open sessions (terminals, etc.) are preserved or closed
2. Active target switches
3. All new operations go to new target
4. UI reflects new target's state (different files, services, etc.)

### State Management

```rust
pub struct TargetManager {
    /// All configured targets
    targets: HashMap<TargetId, TargetConfig>,

    /// Currently active target
    active: TargetId,

    /// Open connections (lazy, kept alive)
    connections: HashMap<TargetId, Box<dyn Target>>,

    /// Open PTY sessions per target
    pty_sessions: HashMap<TargetId, Vec<PtySession>>,
}

impl TargetManager {
    /// Switch active target
    pub fn switch_to(&mut self, id: TargetId) -> Result<()>;

    /// Get active target for operations
    pub fn active(&self) -> &dyn Target;

    /// List all available targets
    pub fn list(&self) -> Vec<&TargetConfig>;
}
```

---

## Example Scenarios

### Scenario 1: Open Terminal on Remote

1. User has "Server A" as active target
2. User clicks Terminal icon
3. DE calls `target_manager.active().open_pty()`
4. RemoteTarget sends `PtyOpen` to Server A's workmeshd
5. workmeshd spawns `/bin/bash`, returns PTY handle
6. egui_term renders shell prompt from Server A
7. User types commands—executed on Server A

**User sees**: A terminal. No indication it's remote (unless prompt shows hostname).

### Scenario 2: Install Firefox on Remote

1. Active target: Raspberry Pi
2. User opens Software Center, searches "Firefox"
3. Clicks Install
4. DE dispatches `InstallPackage("firefox")` to active target
5. Pi's workmeshd runs `apt install firefox`
6. Progress streamed back to DE
7. Toast: "Firefox installed"

**Result**: Firefox installed on Pi, not on master machine.

### Scenario 3: Switch Target Mid-Session

1. User working on Server A (terminal open, file browser showing /home)
2. Clicks "Local" in target selector
3. Terminal shows local shell (Server A terminal can stay open in background or close)
4. File browser now shows local /home
5. All operations now affect local machine

---

## Implementation Phases

### Phase 1: Local Target Only

- Implement `LocalTarget` via Unix socket to local workmeshd
- All current functionality works through Target abstraction
- No visible change to user

### Phase 2: Remote Target Support

- Implement `RemoteTarget` with TCP/TLS transport
- Add target configuration (host, port, auth)
- Target selector UI in top bar

### Phase 3: PTY Streaming

- Implement PTY protocol over network
- Integrate with egui_term
- Handle latency, reconnection

### Phase 4: Advanced Features

- P2P transport for NAT traversal
- Multi-target views (side-by-side)
- Target groups and batch operations

---

## Related Documents

- [PROPOSAL.md](./PROPOSAL.md) - Overall project vision
- [DESKTOP_COMPONENTS.md](./DESKTOP_COMPONENTS.md) - UI components
- workmeshd documentation (separate repository)

---

**Last Updated**: December 2024
