# Hand-off & Work Plan — Widget Fabric Convergence (Weaver-Desktop ↔ HoverClock)

- **Date:** 2026-08-26 (exploration session; no code changed, nothing committed)
- **Repos:** `/development/Weaver-Desktop` (this repo), `/development/hover-clock`
- **Branches:** Weaver work continues on `feature/ux-ui-flex-layouting-app-design`
  (currently checked out: `main` — switch before starting Story 01); HoverClock work follows
  its own `roadmap/ROADMAP.md` (S12 → S05).
- **Conventions:** ICM/MWP guideline (§5 accept → process → handoff; §5.3 handover log
  contract), templates in `/development/llm_profiles/engineering/templates/`. The story
  cards below follow the template shape.

---

## 1. Purpose & strategy

The engineering workspace already commits to the convergence this plan executes
(`llm_profiles/engineering/projects/gtk-overlay-desktop.md` §Shell family, `weaver-desktop.md`
component/state model, hover-clock `docs/proposal.md` §11.4):

> **One widget model, renderer per target.** The fabric is shared: state + `view()` → tree →
> `RenderBackend`; cargo features `gtk`/`egui`/`iced` per artifact. Weaver-Desktop's component
> system *is* the egui renderer impl of the same fabric — the projects converge on one widget
> model, they do not compete as shells.

Weaver-Desktop's in-flight widget refactor (feature branch) is the **seed of that fabric**;
HoverClock's proposal §11 already describes the same shape (composite model, state/render
split, semantic events, swappable render contract). The work therefore contributes to **both
projects**:

1. **Story 01** — land the Weaver widget refactor to a green build (resume `story_01/`).
2. **Story 02** — extract the renderer-neutral **fabric crate** (`crates/weaver_fabric`),
   per the workspace decoupling rule (*"modules are extractable — any module is a candidate
   for standalone publication; extraction opportunities are executed on sight"*, WORKSPACE.md).
3. **Story 03** — implement HoverClock's composite widget model (M4/S05 calendar) **on the
   fabric** with a GTK renderer adapter.

The crates/ split (weaver_lib + weaver_desktop_shell) is the decoupling vehicle; Story 02
extends it by moving the widget model out of the egui-coupled shell crate.

### 1.1 Design intent & architecture — see `docs/WIDGET_FABRIC_DESIGN.md`

The durable design (widget model, thin-contract architecture, trajectories, related repos)
now lives in **`docs/WIDGET_FABRIC_DESIGN.md`** — this handoff only carries the execution plan.

Summary for the reader: `[ application backend ] <-> [ thin contract ] <-> [ ui toolkit ]`;
the contract owns application state + typed event channel + per-backend renderer; widgets
hold UI state and `render()` into the toolkit (React-inspired), dispatch via event objects;
trajectories: egui (now) → GTK/HoverClock (Story 03) → remote/contractual rendering
(`docs/UI_FABRIC_PROPOSAL.md`, workmeshd, kiosks/WorkFlows) → ui-runtime-web
(operational-surface).

---

## 2. Verified current state (2026-08-26 — re-verify before trusting, per ICM/MWP §4)

### 2.1 Weaver-Desktop

- **Repo:** fresh clone, local checkout on `main` (clean). Only two remote branches:
  `main` and `feature/ux-ui-flex-layouting-app-design` — **no other ongoing work exists**.
- **Feature branch:** 5 commits on top of main, linear after the 2026-08-26 rebase onto
  `85ff0d7` (egui 0.36.1); tip `b0c6242` (handoff doc). The widget refactor is the first
  commit of the series ("trait-based composition with layout caching").
- **New widget system** — `crates/weaver_desktop_shell/src/components/widget.rs` (740 lines),
  written and coherent, compiles standalone:
  - Layout types: `Axis`, `Size` (Fixed/Flex/Content), `Align`, `Justify`, `Overflow`, `Spacing`
  - `Widget` trait: `id()`, `style()`, `style_mut()`, `min_size()`, `ui(&mut Ui, Rect)`,
    `compute_layout(Rect)`, `is_visible/is_disabled/set_visible/set_disabled`
  - `Style` struct: layout props + `background: Option<ImageSurface>` + `border_radius`
  - `Container`: `row()`/`column()`, builder chain, 3-pass flexbox (`compute_child_rects`),
    margin→widget rect / padding→content rect, overflow Clip/Visible/Scroll(stub),
    clip-rect inheritance, disabled overlay
  - Layout caching: `CachedLayout` + `layout_dirty`, `needs_layout()`/`invalidate_layout()`
  - ⚠️ **Renderer-coupled:** `ui()` takes `&mut egui::Ui` + `egui::Rect`; background renders via
    egui `ImageSurface`. Not yet renderer-neutral.
- **Build state (verified by `cargo check`):** exactly **3 errors, all `E0432` unresolved
  imports** — the old API (`WidgetStr`, `WidgetContent`, `Label`, `Spacer`) was deleted from
  widget.rs (parked in `old_widget_str.rs`, 1061 lines) but consumers were never migrated:
  1. `crates/weaver_desktop_shell/src/components/desktop_shell.rs:28` — imports
     `Label, WidgetStr, WidgetContent`; `DesktopShell` + `ClockWidget`, `DateWidget`,
     `MenuButton`, `StatusText`, `VersionLabel`, `XpTaskbar` still `impl WidgetContent` (old trait)
  2. `crates/weaver_desktop_shell/src/components/modal.rs:25` — `WidgetStr` content type
  3. `crates/weaver_desktop_shell/src/components/mod.rs:23` — re-exports the dead symbols
     (also `lib.rs` re-exports them)
  - 3 warnings (unused imports/doc comments). Nothing else.
- **Prerequisites for any build:** none — `egui-toast` now comes from crates.io (0.22);
  `egui_term` is vendored at `forks/egui_term` (committed path dep, egui 0.36.1). The old
  `forks/egui-toast` submodule is unused but still registered in `.gitmodules` (kept as-is
  per user instruction).
- **Missing:** leaf widgets as `Widget` impls (`Label`, `Spacer`, `Icon`, `Image`); migration
  of consumers (see errors). Story cards define the work: `story_01/STORY.md` (DoD + tasks
  01–05), `task_01.md` (fix imports + Label/Spacer), `task_xx.md` (cleanup).
- **Design doc divergence:** `docs/WIDGET_REFACTORING_DESIGN.md` planned **incremental**
  migration (keep `WidgetStr` alive, add trait alongside, migrate sites one by one, delete old
  last). Implementation did the opposite — big-bang swap — which is why the build broke.
- **Second, older widget layer:** `weaver_lib` has its own reactive widgets (`Button` with
  `Observable` + `InteractableHandlers`, `Calendar`, `Component` trait) — untouched by the
  refactor, coexists today. Out of scope unless a consumer forces the issue.
- **Docs present:** `docs/PROPOSAL.md`, `UI_FABRIC_PROPOSAL.md`, `ARCHITECTURE_ROADMAP.md`,
  `THEME_ARCHITECTURE.md`, `MULTI_TARGET_ARCHITECTURE.md`, `INDUSTRIAL_ROADMAP.md`,
  `DESKTOP_COMPONENTS.md`, `USE_CASES.md`, `TODO.md`, `schemas/`. No `roadmap/` or
  `handoffs/` directory exists — `story_01/` is the working-artifact convention.

### 2.2 HoverClock

- **Widget system is designed, not implemented.** Proposal §11 defines the composite model
  (layout/leaf/compound kinds, semantic events), §11.4 the render contract (state + `view()` →
  tree → `RenderBackend` adapter; `GtkRenderBackend` today, `EguiRenderBackend` possible later),
  §11.5 the future `WidgetProvider` data plane.
- **M4 calendar (S05) is the card that lands the composite model** — ⬜ backlog, next after
  S12 (`--upgrade` CLI, also ⬜). S05 goal/acceptance (roadmap/ROADMAP.md):
  *"a minimal month calendar whose only job is to show which day of the month today is; grows
  the widget model to widgets containing widgets (layout containers + leaf widgets, Weaver
  Desktop fabric model, §11.1)"* — acceptance: overlay shows current month with today visually
  distinct; composite model exercised (a `layout` containing leaf widgets); no pointer handlers;
  no activation changes.
- **Current code:** no widget module in `src/` — the clock is a hand-written GTK composite
  (`ClockWidget` in `src/main.rs`: hours/minutes/seconds/day/date labels + version label +
  upgrade button), composed in a vertical stack, styled by bundled CSS. M7 merged, v2.0.1 out.
- **Operational (for Story 03):** dev guard — `build.rs` aborts debug builds without swap
  state; use `scripts/swap-to-dev.sh` before, `scripts/swap-to-prod.sh` after any session;
  Wayland smoke via labwc (`docs/WAYLAND_TESTING.md`); formatting pinned to `cargo +1.92.0 fmt`.

### 2.3 What this means

The hard 80% of the Weaver refactor (trait, Style, Container, flexbox, caching) exists and
compiles; what remains is **mechanical migration** (3 files + component conversions) plus the
leaf suite. The design target (model-first fabric) is already documented in both projects'
briefs — the remaining decisions are sequencing and contract shape (see §7).

---

## 3. Story map

| Story | Repo / branch | Status | Summary |
| --- | --- | --- | --- |
| S01 — Land widget refactor to green build | Weaver, `feature/ux-ui-flex-layouting-app-design` | 🔄 resume (`story_01/`) | Fix 3 import errors, convert components to `Widget` trait, wire caching, cleanup |
| S02 — Extract renderer-neutral fabric crate | Weaver, new branch off feature | ⬜ | `crates/weaver_fabric`: widget model + layout engine, no egui types; egui adapter in shell |
| S03 — HoverClock M4/S05 composite widget model | hover-clock, own roadmap | ⬜ | Consume fabric via GTK adapter; calendar + clock as compound widgets; S05 → ✅ |

Ordering: S01 → S02 → S03. S01 is needed first (stable baseline + the migration decisions
inform the fabric contract). S03 can start only after S02 lands a tag (or during co-dev with a
path dep).

---

## 4. Story 01 — Land the Weaver widget refactor to a green build

- **Status:** 🔄 in progress — mid Task 01 of `story_01/` (build errors are the entry point).
- **Goal:** the refactor commit becomes a compiling, working baseline that seeds the fabric.
- **Scope decision (recommended):** **migration-only.** Fix the 3 import errors by converting
  the `WidgetContent` implementors to the new `Widget` trait mechanically (their `ui(&mut Ui)`
  bodies carry over; `id`/`style`/`compute_layout`/state are boilerplate). Implement **only the
  leaves consumers require** (desktop_shell.rs uses `Label` — a minimal `Widget` impl rendering
  text via `ui.painter()` suffices). **Defer the full leaf suite** (Spacer/Icon/Image) to Story
  02: in the fabric (docs/WIDGET_FABRIC_DESIGN.md §1) leaves are presentational widgets whose render calls come from the
  backend adapter, so egui-specific leaf `Widget` impls built now would be reworked. (Alternative — full leaves now
  per `story_01/task_02.md`: more work, largely throwaway; not recommended.)
- **Tasks:**
  1. `components/mod.rs` — fix re-exports (drop `WidgetStr`/`WidgetContent`/`Spacer`; keep what
     exists), clear the 3 warnings (unused imports).
  2. `widget.rs` — add default trait methods where they make conversion mechanical
     (`compute_layout` no-op default, state defaults); minimal `Label` `Widget` impl.
  3. `desktop_shell.rs` — convert `ClockWidget`, `DateWidget`, `MenuButton`, `StatusText`,
     `VersionLabel`, `XpTaskbar` + `DesktopShell` from `impl WidgetContent` to `impl Widget`;
     `modal.rs` — `WidgetStr` content → new type.
  4. Wire layout caching into the shell frame loop (`needs_layout()` → `compute_layout()` →
     `ui()`) so `CachedLayout` is actually exercised.
  5. Cleanup (per `story_01/task_xx.md`): delete `old_widget_str.rs`; delete `temporals/`;
     archive `docs/WIDGET_REFACTORING_DESIGN.md` (mark superseded — its incremental plan was not
     followed); update `AGENTS.md` §Key architectures (still describes `WidgetStr`),
     `lib.rs` re-exports, **and the stale `widget.rs` module doc** (still says "WidgetStr — the
     universal building block" and shows the dead `WidgetStr::column(...).leaf(...)` API —
     contradicts the actual `Widget`/`Container` implementation).
- **Acceptance:**
  - `cargo build` + `cargo clippy` clean (zero warnings), `cargo test` green
  - shell renders via the `Container` tree with caching active (no flicker on updates)
  - `grep -r "WidgetStr\|WidgetContent" crates/ src/` → empty; `old_widget_str.rs` deleted
- **Design refs:** `docs/WIDGET_REFACTORING_DESIGN.md`, `story_01/STORY.md` (+ task_01/xx),
  AGENTS.md.
- **Hand-off:** pending → this document §7; write `story_01` handoff when done.

---

## 5. Story 02 — Extract the renderer-neutral fabric crate (`crates/weaver_fabric`)

- **Status:** ⬜ backlog.
- **Goal:** the widget model becomes renderer-neutral so one core serves egui (Weaver) and GTK
  (HoverClock) — the documented convergence target (gtk-overlay-desktop.md §Shell family,
  weaver-desktop.md component/state model, hover-clock proposal §11.4, JigsawFlow rendering
  facade §6.1). This is the **thin contract** of docs/WIDGET_FABRIC_DESIGN.md §2: application state + typed event channel
  + per-backend renderer, extracted into `crates/weaver_fabric`.
- **Architectural decision — resolved by user (2026-08-26, docs/WIDGET_FABRIC_DESIGN.md §1):** the render contract is
  **stateful widgets whose `render()` calls backend utilities** — the earlier handoff
  dichotomy ("model-first `update`/`view` → tree" vs "neutralized render-into") is superseded.
  - Fabric core (`crates/weaver_fabric`, **no egui dependency**): widget model — state in the
    widget, layout widgets + presentational widgets (layout/leaf/compound kinds), the pure
    flexbox layout engine (already pure: `Axis`/`Size`/`Align`/`Justify`/`Overflow`/`Spacing` +
    `compute_child_rects` + `CachedLayout`), and **event objects** (local/global dispatch,
    semantic events like `activated`).
  - Render contract: `render()` reads widget state and calls backend utilities. Each backend
    provides those utilities: egui adapter (painting primitives for label/spacer/icon/image,
    input → event objects), GTK adapter later (Story 03).
  - `Observable`/`SignalFn` from `weaver_lib` are candidates for the state primitive — decide
    at Story 02 kickoff (§7 Q6).
- **Tasks:**
  1. Scaffold `crates/weaver_fabric` — **no egui dependency**; own geometry (fabric `Rect`/
     `Vec2`-equivalent) with conversion at adapter boundaries; `Event` + semantic events.
  2. Port the layout engine: `Axis`/`Size`/`Align`/`Justify`/`Overflow`/`Spacing` +
     `compute_child_rects` + `CachedLayout` — renderer-neutral.
  3. `Widget` trait (state + render contract per docs/WIDGET_FABRIC_DESIGN.md), layout/leaf/compound kinds, event
     objects (local/global) + semantic events.
  4. **egui adapter** (in `weaver_desktop_shell` or sibling crate): the backend utilities
     `render()` calls — painting for label/spacer/icon/image, input → event objects.
  5. Migrate Story 01's components onto the fabric; shell renders identically.
- **Acceptance:**
  - `grep egui crates/weaver_fabric` → empty (gate)
  - `weaver_lib` + shell green (`cargo build`, `clippy`, `test`)
  - layout caching verified (no flicker); a minimal tree renders identically before/after
  - fabric crate is path-dep of the shell; **publication deferred** (brief: extraction after
    widget model matures — post-M4); HoverClock consumes it via git dep pinned to a tag
    (MIT → BSD-3 compatible) or path dep during co-dev
- **Design refs:** `gtk-overlay-desktop.md` (§Shell family, Widget Model),
  `weaver-desktop.md`, hover-clock `docs/proposal.md` §11.1/§11.4, jigsawflow §6.1.
- **Hand-off:** pending.

---

## 6. Story 03 — HoverClock M4/S05: composite widget model on the fabric

- **Status:** ⬜ backlog — roadmap position is **after S12** (keep order unless user says
  otherwise, §7).
- **Goal:** S05 acceptance met: calendar compound widget + composite model exercised, overlay
  becomes a tree, not a flat list (§11.1).
- **Tasks:**
  1. Add `weaver_fabric` dep (git, pinned tag — or path dep during co-dev) + **GTK backend
     utilities**: the calls `render()` uses (`GtkBox`/`GtkLabel`/`GtkGrid`), semantic events →
     GTK signals; fabric↔Gdk rect conversion.
  2. Refactor `ClockWidget` → compound widget (state/view split per §11); version label +
     upgrade button as leaves with semantic `activated`.
  3. Calendar compound widget per S05 card: month grid `layout` of day-cell `label`s + weekday
     header; today via `.calendar-today` CSS class; composed next to the clock in the vertical
     stack; non-interactive (no pointer handlers).
- **Acceptance:** S05 card acceptance (overlay shows current month, today visually distinct,
  composite model exercised, no pointer handlers, no activation changes); S05 → ✅; §11.1
  confirmed implemented; smoke on X (xfwm4) + Wayland (labwc) per hover-clock AGENTS.md.
- **Ops:** swap-to-dev / swap-to-prod discipline (build.rs dev guard); `cargo +1.92.0 fmt`;
  `docs/WAYLAND_TESTING.md` for the labwc session.
- **Design refs:** hover-clock `docs/proposal.md` §8.1, §11; `roadmap/ROADMAP.md` S05.
- **Hand-off:** pending.

---

## 7. Hand-off log (this session)

Per ICM/MWP §5.3 contract:

| Section | Contents |
| --- | --- |
| **Task** | None — exploration + planning only. No code changed, nothing committed. Roadmap state: unchanged (Weaver has no roadmap; hover-clock roadmap untouched). |
| **What was done** | Explored `feature/ux-ui-flex-layouting-app-design` (4 commits, tip 2026-06-04); verified the build state via `cargo check` in a worktree (3 E0432 errors, 3 warnings; submodule `forks/egui-toast` needed init); read `widget.rs`, `story_01/*`, `docs/WIDGET_REFACTORING_DESIGN.md`, AGENTS.md, `.mwp/topology.md`; read engineering briefs (`weaver-desktop.md`, `gtk-overlay-desktop.md`) and ICM/MWP guidelines; read hover-clock proposal §11 + roadmap S05. Produced this plan. |
| **What was done differently** | none (no implementation attempted). |
| **Verification** | `cargo check` on the feature branch in a worktree at `/tmp/weaver-fb` (CARGO_TARGET_DIR reused): errors exactly as listed in §2.1. Branch/repo facts from `git log`, `git branch -r`, `git reflog`. |
| **Open questions (user decisions)** | 1. **Fabric contract shape — ANSWERED** (2026-08-26, docs/WIDGET_FABRIC_DESIGN.md): stateful widgets, `render()` calls backend utilities, dispatch via event objects, flexbox layout model. The earlier `update`/`view` → tree dichotomy is superseded. 2. **S01 scope:** migration-only (recommended) vs full leaf suite per `story_01`. 3. **HoverClock ordering:** keep S12 → S05 (recommended) vs pull S05 forward. 4. **Branch:** continue `feature/ux-ui-flex-layouting-app-design` (recommended) vs new branch off it. 5. **Fabric crate name:** `weaver_fabric` (recommended — docs already say "Weaver Desktop fabric"). 6. **State primitive — ANSWERED:** reuse `weaver_lib`'s `Observable`/`SignalFn` (user confirmed 2026-08-26, recommended direction). 7. **ui-runtime-web reading — ANSWERED** (correct; next abstraction once the widget system works; widget system also becomes the base for normal + kiosk Linux DEs, docs/WIDGET_FABRIC_DESIGN.md §3). 8. **Serde / remote rendering — ANSWERED:** **deferred** (user decision 2026-08-26). No serde, no wire shaping in Story 02; fabric stays local-first. Remote/contractual rendering becomes a future DelfinFlow asset (likely paid feature, open protocol) — recorded in docs/WIDGET_FABRIC_DESIGN.md §2. |
| **Next step** | Story 01 (migration-only scope confirmed): fix the 3 errors + warnings, convert components, wire caching, cleanup, verify per §4 acceptance, write the story_01 handoff. Then Story 02 extracts the thin contract (docs/WIDGET_FABRIC_DESIGN.md §2). |

---

## 8. Verification commands (next session)

```bash
# Weaver (Story 01/02)
cd /development/Weaver-Desktop
git checkout feature/ux-ui-flex-layouting-app-design
cargo build && cargo clippy && cargo test        # target: zero warnings, green

# HoverClock (Story 03)
cd /development/hover-clock
./scripts/swap-to-dev.sh                          # dev guard: NEVER bare `cargo run`
# ... build/test/smoke ...
./scripts/swap-to-prod.sh                         # restore + restart daemon after session
```

---

## 9. Reading order for the next session

1. This document (the handoff is the starting context — do not re-explore §2).
2. `docs/WIDGET_FABRIC_DESIGN.md` (design intent + thin contract + trajectories).
3. `story_01/STORY.md` + `task_01.md` (Story 01 task detail).
4. `crates/weaver_desktop_shell/src/components/widget.rs` (the new system to build on).
5. On Story 02/03: `gtk-overlay-desktop.md` + `weaver-desktop.md` briefs, hover-clock
   `docs/proposal.md` §11.
6. Verify references (§2 claims) before reasoning — the handoff's claims are assumptions
   until re-checked (ICM/MWP §4).
