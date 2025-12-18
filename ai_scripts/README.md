# AI Tasks

This directory contains structured task definitions for AI assistants to perform code quality checks, optimization reviews, and project maintenance tasks for Weaver Desktop.

## Purpose

Weaver Desktop targets resource-constrained SBCs and embedded devices. These tasks help:

- **Validate SBC optimization patterns** - Ensure zero-allocation principles are followed
- **Maintain documentation consistency** - Keep README, docs, and code in sync
- **Track technical debt** - Identify improvement opportunities
- **Enforce best practices** - Memory layout, reactive patterns, static data usage

## Usage

1. Open a task file in your editor (e.g., `sbc_optimization_review.md`)
2. Ask your AI assistant to "execute this task" or "run this audit"
3. The assistant will perform the analysis and generate a report
4. Review the output and address findings as needed

## Task Structure

Each task file contains:

| Section | Purpose |
|---------|---------|
| **Objective** | What the task accomplishes |
| **Context** | Background information and project constraints |
| **Subtasks** | Specific checks to perform with checkboxes |
| **Output** | Expected report format and location |

## Available Tasks

| Task | Description | When to Run |
|------|-------------|-------------|
| `AI_PAIR_PARTNER_PROMPT.md` | Pair programming context prompt for Weaver Desktop development | Start of new AI development sessions |
| `sbc_optimization_review.md` | Full codebase review for SBC/low-resource optimization | Before releases, after major features |
| `documentation_consistency.md` | Validate README, docs, and project structure alignment | After updating docs or project structure |

## Target Constraints Reference

When creating new tasks, keep these Weaver Desktop constraints in mind:

- **Memory**: 256MB - 512MB RAM targets
- **CPU**: Limited single-core to quad-core ARM
- **Display**: 7" touchscreens, kiosk displays
- **Footprint**: <50MB total application size

## Key Optimization Principles

- Prefer `&'static str` over `String`
- Use `fn(&T)` pointers over `Box<dyn Fn>` closures
- Stack-allocate with `ArrayString`/arrays where possible
- Cache computed values to reduce per-frame work
- Minimize syscalls in hot paths (render loops)
