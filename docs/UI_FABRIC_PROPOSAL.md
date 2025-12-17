# Weaver UI Fabric

**A Socket-Driven, Config-Shaped User Interface Runtime**

**Status**: Proposal  
**Date**: December 2025

---

## 1. Concept Overview

Weaver Desktop evolves from a static desktop environment into a **UI fabric** — a runtime system capable of materializing user interfaces on demand from configuration or live scripts, while maintaining clear authority, safety, and visual boundaries.

The key innovation is a **UI-over-socket interface**:

Any process — local or remote, human-written or AI-generated — can declare a UI surface through a Unix socket (or TCP), and Weaver will render it inside a constrained, governed container (window, widget, panel, or modal).

**Weaver remains the sole authority over:**

- Rendering
- Interaction semantics
- Permissions
- Execution of actions

**External processes describe UI — they do not draw pixels.**

---

## 2. Strategic Alignment

UI Fabric extends Weaver's existing architectural principles:

| Existing Principle | UI Fabric Extension |
|---|---|
| **GUI/daemon separation** (Weaver renders, workmeshd executes) | External processes declare UI, Weaver renders, workmeshd executes actions |
| **Panels, not primitives** (semantic abstractions) | Widget declarations, not pixel drawing |
| **Template-driven flexibility** | Socket sessions bound to template-defined slots |
| **Capability-based safety** | Per-session capability sets |
| **Audit by default** | Every UI-triggered action logged |

The socket-driven UI model applies the same philosophy outward — just as workmeshd abstracts hardware from Weaver, UI Fabric abstracts UI declaration from external processes.

---

## 3. Design Goals & Non-Goals

### Goals

| Goal | Description |
|------|-------------|
| **Shapeable UI by configuration** | The same system binary can manifest radically different interfaces |
| **Runtime UI materialization** | UI can appear, change, or disappear without restarting the desktop |
| **Language-agnostic UI declaration** | Any language capable of writing to a socket can create UI |
| **Strict visual and logical boundaries** | External UIs live inside constrained containers |
| **AI-compatible, AI-safe** | AI can propose interfaces, but never bypass authority |
| **Offline-first, local-first** | No browser, no cloud dependency required |

### Non-Goals

- ❌ This is **not** a web browser replacement
- ❌ This is **not** remote desktop
- ❌ This is **not** pixel streaming
- ❌ This is **not** arbitrary code execution

---

## 4. Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Weaver Desktop                       │
│                                                         │
│  ┌─────────────────┐   ┌────────────────────────────┐   │
│  │   UI Runtime    │◀──▶│   UI Socket Listener      │◀──┼── External Processes
│  │   (egui)        │   │   (Unix/TCP)               │   │
│  └─────────────────┘   └────────────────────────────┘   │
│          ▲                          │                   │
│          │                          ▼                   │
│  ┌─────────────────┐   ┌────────────────────────────┐   │
│  │   Config &      │◀──▶│   workmeshd (Actions)     │   │
│  │   Template      │   │   (Execution Authority)    │   │
│  └─────────────────┘   └────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Key Principle

**UI declaration is separate from execution.**

External processes declare:

- Structure
- State  
- Intent

Weaver decides:

- Placement
- Rendering
- Interaction handling
- Safety rules
- Whether actions are allowed

---

## 5. UI-over-Socket Model

### 5.1 Transport

| Transport | Use Case |
|-----------|----------|
| **Unix domain socket** | Primary, local processes |
| **TCP (authenticated)** | Remote/cloud backends |

- One socket connection = one UI session
- Each session has identity, lifetime, and assigned container

### 5.2 UI Scope & Boundaries

To maintain clarity and safety, socket-driven UI is **never global**.

Each session is explicitly bound to a UI container:

| Container Type | Description |
|----------------|-------------|
| `widget` | Embedded in panel/dashboard |
| `window` | Standard application window |
| `modal` | Blocking interaction, requires dismissal |
| `sheet` | Temporary overlay from screen edge |
| `panel_slot` | Reserved area defined by template |

**This is a hard constraint, not a suggestion.**

External processes **cannot**:

- Spawn arbitrary windows
- Alter desktop layout
- Override system UI

### 5.3 UI Declaration Model

UI is described declaratively via structured messages (JSON / MessagePack / CBOR).

**Example (simplified):**

```json
{
  "type": "ui.define",
  "container": {
    "kind": "modal",
    "title": "Coffee Order"
  },
  "layout": {
    "direction": "column"
  },
  "widgets": [
    {
      "id": "espresso",
      "type": "button",
      "label": "Espresso",
      "action": "order.espresso"
    },
    {
      "type": "status",
      "source": "machine.temperature"
    }
  ]
}
```

**Weaver:**

- Validates schema
- Maps widgets to native egui components
- Enforces safety constraints
- Renders within assigned container

### 5.4 Event Flow

```
External Process ──► UI Declare
         Weaver ───► Render
           User ───► Interaction
         Weaver ───► Event (to process)
External Process ──► State Update
         Weaver ───► Re-render
```

External processes **never receive raw input events**, only semantic events:

```json
{
  "type": "event",
  "source": "espresso",
  "event": "activated"
}
```

---

## 6. Config-Shaped Desktop Model

Weaver's core strength is that the desktop itself is a template.

### 6.1 Templates Define

- Screen layout
- Available container slots
- Allowed UI types per slot
- Default safety policies
- Visual identity

**Example template configuration:**

```toml
[slots.left_panel]
allowed = ["widget"]
max_sessions = 3

[slots.main]
allowed = ["window", "modal"]
requires_focus = true

[slots.hardware]
allowed = ["widget"]
safety_level = "critical"
```

External UI sessions must bind to an existing slot.

### 6.2 Profiles Shape Reality

Profiles determine:

- Which sockets exist
- Who can connect
- Which UI primitives are available

A coffee machine profile might:

- Expose only `button`, `status`, `progress`
- Disable text input
- Restrict actions to predefined commands

**Same binary — entirely different system.**

---

## 7. Security, Authority, and Safety

This is where Weaver is fundamentally different from web UI.

### 7.1 Capability-Based UI

Each session receives a capability set:

```json
{
  "widgets": ["button", "status"],
  "actions": ["order.espresso", "order.cancel"],
  "containers": ["modal"]
}
```

Requests outside this set are **rejected**.

### 7.2 Action Execution Boundary

UI triggers actions **by name — not code**.

```json
{ "action": "order.espresso" }
```

Only **workmeshd**:

- Validates the action
- Logs the request
- Executes (if permitted)
- Enforces safety rules

### 7.3 Audit by Default

Every UI-triggered action is logged with:

- Session ID
- Source widget
- Timestamp
- Outcome

This makes the system:

- Debuggable
- Certifiable
- Trustworthy

---

## 8. AI Integration (Controlled)

AI agents become **UI authors, not operators**.

### Allowed

- Propose layouts
- Assemble widgets
- Suggest workflows

### Forbidden

- Direct system access
- Bypassing confirmation dialogs
- Privilege escalation

AI-generated UI is still:

- Validated
- Constrained
- Rendered by Weaver

**This prevents:** "AI clicked something dangerous"

---

## 9. Use Cases Enabled

### 9.1 Cloud Applications Without Browsers

Backend logic runs in cloud, UI materializes locally:

```
┌─────────────────┐         ┌─────────────────┐
│  Cloud Backend  │◀──TCP──▶│  Weaver Desktop │
│  (Python/Node)  │         │  (Local UI)     │
└─────────────────┘         └─────────────────┘
```

- No Electron
- No browser tab
- Local authority over actions
- Offline fallback possible

### 9.2 AI-Driven Interfaces

```
User: "I need to order coffee"

AI Agent ──► Declares modal with coffee options
  Weaver ──► Renders modal
    User ──► Taps "Espresso"
  Weaver ──► Sends event to AI
AI Agent ──► Calls backend API
AI Agent ──► Updates UI with confirmation
```

AI never has direct system access.

### 9.3 Industrial Device Dashboards

Each device connects via socket and declares its control panel:

- Temperature controller declares sliders and status
- Conveyor declares start/stop/speed
- All rendered consistently by Weaver
- All actions logged and audited

### 9.4 Dynamic Kiosk Deployments

Remote management server updates kiosk UI without:

- Reflashing device
- Restarting desktop
- Physical access

Just push new UI declaration over socket.

---

## 10. Monetization Opportunities

| Opportunity | Description |
|-------------|-------------|
| **Cloud UI Hosting** | Backend logic in cloud, UI materializes locally |
| **AI Application Marketplace** | AI-driven apps running safely within Weaver governance |
| **Enterprise Integration Licensing** | Connect legacy systems to modern touch UIs via socket protocol |
| **Developer SDK** | Libraries for Python, Node, Rust to declare UI over socket |
| **Managed Fabric Service** | Remote UI deployment and management for fleets |

---

## 11. Protocol Specification (Draft)

### 11.1 Message Types

| Message Type | Direction | Description |
|--------------|-----------|-------------|
| `session.init` | Client → Weaver | Request session with capabilities |
| `session.granted` | Weaver → Client | Confirm session with assigned capabilities |
| `session.denied` | Weaver → Client | Reject session with reason |
| `ui.define` | Client → Weaver | Declare/update UI structure |
| `ui.update` | Client → Weaver | Partial update to existing widgets |
| `ui.clear` | Client → Weaver | Remove all UI for session |
| `event` | Weaver → Client | User interaction event |
| `action.result` | Weaver → Client | Result of action execution |
| `session.close` | Either | End session |

### 11.2 Widget Types (Initial Set)

| Widget | Description |
|--------|-------------|
| `button` | Clickable action trigger |
| `label` | Static text display |
| `status` | Dynamic status with data binding |
| `progress` | Progress bar (determinate/indeterminate) |
| `slider` | Numeric value control |
| `toggle` | Boolean switch |
| `text_input` | Text entry field |
| `select` | Dropdown/radio selection |
| `image` | Static or dynamic image |
| `separator` | Visual divider |
| `group` | Container for layout |

### 11.3 Layout Properties

```json
{
  "layout": {
    "direction": "column" | "row",
    "spacing": 8,
    "padding": [16, 16, 16, 16],
    "align": "start" | "center" | "end" | "stretch"
  }
}
```

---

## 12. Implementation Roadmap

### Phase 1: Foundation

- [ ] Define wire protocol (JSON over Unix socket)
- [ ] Implement socket listener in weaver_lib
- [ ] Basic session management
- [ ] Render simple widgets (button, label, status)

### Phase 2: Core Widgets

- [ ] Full widget set implementation
- [ ] Layout engine for declared UIs
- [ ] Container binding (modal, window, widget slot)
- [ ] Event dispatch to clients

### Phase 3: Security & Capabilities

- [ ] Capability negotiation
- [ ] Action routing to workmeshd
- [ ] Audit logging integration
- [ ] Template slot enforcement

### Phase 4: SDK & Developer Experience

- [ ] Python SDK for UI declaration
- [ ] Node.js SDK
- [ ] Rust client library
- [ ] Documentation and examples

### Phase 5: Cloud & AI Integration

- [ ] TCP transport with authentication
- [ ] AI agent integration patterns
- [ ] Cloud backend examples
- [ ] Managed deployment tools

---

## 13. Strategic Value (Summary)

This system enables:

| Capability | Value |
|------------|-------|
| **Cloud logic, local authority** | SaaS without browser, local data sovereignty |
| **AI assistance without AI control** | Safe AI integration by design |
| **Dynamic applications without browsers** | No Electron, no web complexity |
| **One binary, infinite interfaces** | Template-driven flexibility |
| **Human-readable, inspectable interaction** | Full audit trail |

**Weaver becomes:** The execution-safe UI substrate for the post-application era.

---

## 14. Final Summary

> Weaver UI Fabric turns the desktop into a governed runtime where humans, scripts, and AI can manifest constrained, meaningful interfaces — without surrendering control, safety, or sovereignty.
