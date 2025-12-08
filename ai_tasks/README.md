# AI Tasks

This directory contains structured task definitions for AI assistants to perform code quality checks, optimization reviews, and best practices validation.

## Purpose

- Ensure consistency across the codebase
- Validate SBC/low-resource optimization patterns
- Track technical debt and improvement opportunities
- Generate actionable reports

## Usage

1. Open a task file (e.g., `sbc_optimization_review.md`)
2. Ask the AI assistant to execute the task
3. The assistant will generate a report in the project root (e.g., `OPTIMIZATION_REPORT.md`)

## Task Structure

Each task file contains:

- **Objective**: What the task accomplishes
- **Subtasks**: Specific checks to perform
- **Output**: Expected report format and location
- **Context**: Background information for the assistant

## Available Tasks

| Task | Description | Output |
|------|-------------|--------|
| `sbc_optimization_review.md` | Full codebase review for SBC optimization | `OPTIMIZATION_REPORT.md` |
