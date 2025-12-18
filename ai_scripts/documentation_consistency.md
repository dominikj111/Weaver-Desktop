# Task: Documentation Consistency Review

## Objective

Validate that the README, docs folder, and project structure are aligned and up-to-date. Ensure documentation accurately reflects the current codebase state.

---

## Context

Weaver Desktop maintains documentation across multiple files:

- `README.md` - Project overview and quick reference
- `docs/` - Detailed technical documentation
- `ai_scripts/` - AI assistant task definitions
- Inline code documentation and comments

**Key Principle:** Documentation should stay in sync with the codebase. Outdated docs create confusion and waste developer time.

---

## Subtasks

### 1. README Accuracy Check

Verify the main README reflects current project state:

- [ ] **Project Structure section** - All folders listed exist and descriptions are accurate
- [ ] **Documentation table** - All docs listed exist with correct descriptions
- [ ] **Status section** - Reflects actual development phase
- [ ] **Vision/Purpose sections** - Still accurate with current direction
- [ ] **Branding** - Names and concepts are consistent throughout

**Report:** List any discrepancies found.

---

### 2. Docs Folder Audit

Check each file in `docs/`:

- [ ] **PROPOSAL.md** - Core features match implementation plans
- [ ] **ARCHITECTURE_ROADMAP.md** - Component status table is current
- [ ] **TODO.md** - Task completion status is accurate
- [ ] **DESKTOP_COMPONENTS.md** - Component list matches actual crates
- [ ] **MULTI_TARGET_ARCHITECTURE.md** - Architecture still planned/valid
- [ ] **STRATEGIC_ANALYSIS.md** - Market analysis still relevant
- [ ] **BUSINESS_STRATEGY.md** - Strategy unchanged or needs update
- [ ] **GO_TO_MARKET_STRATEGY.md** - Timeline and targets current

**Report:** Flag outdated sections with severity (STALE/NEEDS_UPDATE/CRITICAL).

---

### 3. Crate Documentation

Verify crate-level documentation:

- [ ] `crates/weaver_lib/Cargo.toml` - Description is accurate
- [ ] `crates/weaver_desktop_shell/Cargo.toml` - Description is accurate
- [ ] `crates/weaver_lib/src/lib.rs` - Module-level docs present
- [ ] `crates/weaver_desktop_shell/src/lib.rs` - Module-level docs present
- [ ] Each `mod.rs` file has brief module documentation

**Report:** List crates/modules missing documentation.

---

### 4. Cross-Reference Validation

Check internal links and references:

- [ ] All links in README.md resolve to existing files
- [ ] All links in docs/*.md resolve correctly
- [ ] Code references in docs match actual module/function names
- [ ] File paths mentioned in docs are correct

**Report:** List broken links or incorrect references.

---

### 5. Terminology Consistency

Verify consistent naming across all documentation:

- [ ] "Weaver Desktop" vs "Weaver" usage (both acceptable, but consistent)
- [ ] "WorkMesh" capitalization (not "Workmesh" or "workmesh" except for `workmeshd`)
- [ ] "workmeshd" for the daemon (lowercase)
- [ ] Technical terms used consistently (SBC, GPIO, PWM, MCU, I2C, etc.)

**Report:** List inconsistent terminology with locations.

---

### 6. AI Tasks Alignment

Ensure ai_scripts folder is useful and current:

- [ ] `ai_tasks/README.md` - Accurately describes available tasks
- [ ] All listed tasks in README exist as files
- [ ] Task files follow consistent structure
- [ ] Task context reflects current project constraints

**Report:** List gaps or improvements needed.

---

## Output Format

Generate a `DOCUMENTATION_CONSISTENCY_REPORT.md` in the project root with:

```markdown
# Documentation Consistency Report

**Generated:** [Date]
**Status:** [PASS | NEEDS_ATTENTION | CRITICAL]

## Summary

- Total checks: X
- Passing: X
- Needs update: X
- Critical: X

## Findings

### README Issues
[List issues or "✅ No issues found"]

### Docs Folder Issues
[List by file]

### Crate Documentation
[List missing docs]

### Broken Links
[List with locations]

### Terminology Issues
[List inconsistencies]

### AI Tasks Issues
[List gaps]

## Recommended Actions

1. [Priority action 1]
2. [Priority action 2]
...
```

---

## When to Run

- After significant documentation updates
- Before releases or milestones
- When project structure changes (new crates, folders)
- Quarterly maintenance check
