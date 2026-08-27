# Widget Fabric Design

**The widget model and the thin contract between application and UI toolkit.**

- **Status:** design — being implemented (see `HANDOFF.md` Story 01/02 on the feature branch)
- **Scope:** this is the durable home for the widget system's design intent, architecture and
  trajectories. Handoff details (story cards, current state, decisions log) live in
  `HANDOFF.md`, which is temporary and will be removed.

---

## 1. Design intent (why this exists)

The widget system is **transferable by design** — not a Weaver-internal implementation detail.
State + view model is meant to be **reused across projects and GUI libraries**, with only the
render backend swapped per target.

Design rationale (ReactJS-inspired): egui's immediate mode repaints every frame and owns no
state; the widget model therefore keeps **state out of the rendering pipeline**. The widget
tree persists across frames, rendering is a function of state, and layout is cached
(`CachedLayout` + dirty flag → no recompute unless rect/style/children changed).

The model, restated from the original intent (user-confirmed):

1. **State lives in the widget object — never in egui/backend objects.** egui owns nothing
   between frames; the widget keeps state across frames, separated from rendering.
2. **`render()` is React-inspired:** it has access to the object's state and **calls backend
   utilities** — egui utilities today, GTK utilities for HoverClock, whatever the web runtime
   needs later. It is a per-backend render function, **not** a pure tree description (VNode);
   the backend is called directly from `render()`.
3. **Dispatch = event objects.** Any dispatch is an event object — local or global. No ad-hoc
   closure plumbing (egui's per-frame lambdas); semantic events flow as objects.
4. **Composition = layout widgets + presentational widgets.** A widget is a standalone UI
   component holding layout widgets and presentational widgets — Java-Swing-like (configure
   layout, put controls into it). The layout model follows the HTML evolution: **flexbox
   (`flex` display) is the correct layout model** — padding/border/margin + flex axis/size/
   align/justify (the `Axis`/`Size`/`Align`/`Justify`/`Spacing`/`Overflow` types in
   `crates/weaver_desktop_shell/src/components/widget.rs`).

## 2. Architecture — application ↔ thin contract ↔ UI toolkit

```
[ application backend ] <-> [ thin contract ] <-> [ ui toolkit ]
```

The **thin contract** is the layer being built (Story 01 → Story 02, see `HANDOFF.md`). It owns:

1. **Application state (the model)** — not the UI toolkit's state, not per-widget UI state.
   The user does not want to manage toolkit state; the contract keeps the application state
   and the model.
2. **Typed event channel** — semantic events as objects, both directions: user actions
   (UI → application) and state/notifications (application → UI). The existing
   `CommandBus<AppCommand>` + `ExternalReceiver` in Weaver-Desktop is the prototype of this
   channel.
3. **Per-backend renderer** — the adapter that renders the model/UI state with the current
   toolkit (egui today, GTK for HoverClock, web runtime later) and translates toolkit input
   into event objects.

### Name

The contract's working name is **the Fabric** (crate `weaver_fabric`, Story 02) — "thin
contract" describes its *role*, "fabric" is the *name* (already in use across the docs:
`UI_FABRIC_PROPOSAL.md`, "Weaver Desktop fabric").

### Render as a translation layer (facade)

The per-backend renderer is a **facade** (per JigsawFlow rendering facade, §6.1): it translates
between the toolkit and the contract, keeping the widget's `render()` **thin**. Per-toolkit
renderers ship as **cargo features** (`gtk`/`egui`/`qt`/`web`), each implementing the same
facade for its toolkit — the widget's `render()` calls the facade, the facade does the
toolkit-specific work (egui immediate-mode paint, GTK retained widgets, etc.), including the
create/update/destroy reconciliation for retained toolkits. Widgets stay toolkit-agnostic;
only the facade knows the toolkit's idioms.

### Two-state rule

**UI state** (text-field value, open panel, clock time) belongs to widgets on the toolkit
side; **application state** belongs to the contract. Widgets never call business logic
directly — they emit event objects; the contract handles them and new state flows back down.
Business logic and data live behind the contract, not in widgets.

### Consequences

- **UI toolkit is replaceable** — swap egui for GTK (or a web runtime later) without touching
  the application backend; only the per-backend renderer changes.
- **Application is testable without UI** — drive the contract directly (state + events) in
  tests, no toolkit involved.
- **No lambdas-per-frame** — egui's per-frame closure callbacks are replaced by the typed
  event channel; business logic is not stuffed into render frames.
- **One seam, not many** — backend ↔ GUI interaction is narrowed to the contract's channel.
- **Contractual rendering across network** — the contract's channel is not bound to one
  process: the application backend can run as a service, a CLI script, a program, or a
  `workmeshd` P2P-mesh daemon, and render the GUI **on the consumer machine** (kiosk,
  WorkFlows desktop). Same contract, wire instead of in-process calls. See §3.

### Hard constraint (implied by remote rendering)

Event objects and application state must be **serializable plain data** (no closures, no
backend types/handles inside events) — they are future wire messages. The *shape* is decided
(events = data enums, state = data structures, renderer = the only backend-typed part); the
timing of adding serde is a Story 02 decision.

### Scope guard

The contract stays **thin** — event objects, application state, per-backend renderer. Diffing
engines, lens systems, virtual DOMs etc. are only pulled in when a concrete backend forces
them (GTK's retained mode will force reconciliation questions in Story 03).

## 3. Trajectories (confirmed targets)

| Target | GUI backend | Where | When |
| --- | --- | --- | --- |
| Weaver Desktop | egui (immediate mode) | this repo | now |
| HoverClock | GTK (retained mode) | `../hover-clock/` (S05 calendar + clock) | Story 03 |
| Linux DE base (normal + kiosk) | egui (this system) | Weaver Desktop is the base for the DE — normal desktop as well as kiosk DEs; personal Linux distro (WorkFlows direction: `../businesses/WorkFlows/`) | the widget system is the foundation |
| Remote / contractual rendering | any backend (egui/GTK/web) | services, CLI scripts, programs and `workmeshd` P2P-mesh daemons render GUI **on the consumer machine** — kiosks and the WorkFlows desktop | future — via the thin contract's channel crossing the network |
| Operational Surface | web runtime (`ui-runtime-web`, JS/TS) | `../businesses/operational-surface/` | later — next abstraction once the widget system works |

**Related architecture docs in this repo:**

- `docs/UI_FABRIC_PROPOSAL.md` — the socket-driven UI runtime: external processes (local or
  remote, human or AI) declare UI over a Unix/TCP socket, Weaver renders it in governed
  containers with semantic events and action-by-name execution. This is the **external-UI
  path** (§Two paths below), distinct from the fabric's first-party draw-directly path.
- `docs/MULTI_TARGET_ARCHITECTURE.md` — transparent remote control of multiple machines
  (local + remote via workmeshd): the application backend running elsewhere, GUI rendered
  locally.
- `HANDOFF.md` — the execution plan (Story 01: land the widget refactor; Story 02: extract
  the thin contract as `crates/weaver_fabric`; Story 03: GTK adapter for HoverClock).

**Related repos:** `../workmeshd/` (P2P mesh daemon — remote control/orchestration, backend
infrastructure for Weaver; daemons are future contract consumers), `../ui-runtime-web/`
(back-end-controlled front-end runtime — web-side precedent for contractual rendering),
`../businesses/operational-surface/` (client portal on ui-runtime-web),
`../businesses/WorkFlows/` (reference Linux distro — kiosk/desktop consumer),
`../hover-clock/` (GTK consumer, Story 03).

### Two paths to remote UI (do not conflate)

- **First-party widgets** (compiled into the consumer — clock, kiosk, DE): **the fabric**.
  Draw-directly; only **state + events** cross the wire; the consumer runs its own `render()`
  against its local facade. **No tree serialization.**
- **External/unknown processes** (can't be compiled in — AI, third-party scripts): JSON UI
  declaration (`docs/UI_FABRIC_PROPOSAL.md`), materialized by Weaver under governance. A
  separate, serialized path.

The fabric is for *our* UI; the socket proposal is for *external* UI.

### Base component set (curated, small)

Streamed/declarative UI is far off, but the set of base UI components must stay **small and
curated** (~10 primitive kinds) so the translation layer does not balloon. The existing
proposal enumerates one such set (`docs/UI_FABRIC_PROPOSAL.md` §11.2: button, label, status,
progress, slider, toggle, text_input, select, image, separator, group). When external JSON
declarations arrive, they map onto these wrappers (button/text-field/…) implemented by the
per-toolkit facades.

## 4. Relationship to existing code

| Existing piece | Role in the design |
| --- | --- |
| `CommandBus<C>` + `ExternalReceiver<C>` (`weaver_lib::commands`) | Prototype of the typed event channel (UI → state after render; daemon/network → UI). The shell instantiates them as `CommandBus<AppCommand>` (`weaver_desktop_shell::commands`). |
| `Observable<T>` / `SignalFn<T>` / `SignalFnMulti` (`weaver_lib::reactive`) | Available state primitive — zero-allocation, subscription-based; reuse recommended for the fabric's state (decision at Story 02 kickoff) |
| `widget.rs` layout types (`Axis`/`Size`/`Align`/`Justify`/`Overflow`/`Spacing`) + `compute_child_rects` + `CachedLayout` | The flexbox layout engine — algorithm and types are renderer-neutral, but geometry (`Rect`/`Vec2`) is currently imported via `egui::` (egui re-exports `emath`, a standalone math crate). Story 02 re-points geometry imports to `emath` directly so `weaver_fabric` has no egui dependency. |
| `ImageSurface`, egui-coupled `ui(&mut egui::Ui, rect)` render-into | The egui-coupled parts that Story 02 moves behind the per-backend renderer |

## 5. Current status

- Widget refactor (trait-based `Widget` + `Container`) is written and coherent; the branch
  currently has 3 unresolved import errors (consumers still reference the old `WidgetStr`).
- Story 01 (migration-only): fix the errors, convert components, wire caching, cleanup —
  see `HANDOFF.md` §4.
- Story 02: extract `crates/weaver_fabric` — the thin contract (state + event channel +
  per-backend renderer), no egui dependency.
