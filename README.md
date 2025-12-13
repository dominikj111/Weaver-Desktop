# SystemWeaver

A lightweight desktop environment for kiosks, cyberdecks, and resource-constrained systems.

*Note: SystemWeaver will evolve into "Flow" or "Weaver" (the desktop environment) as system management responsibilities migrate to dedicated Workmesh daemon services.*

## Purpose

SystemWeaver currently combines desktop environment and system management functionality in a single application. Traditional desktop environments bundle window management, system services, and user interface into monolithic packages consuming 300-600MB of RAM. SystemWeaver separates concerns differently: a lightweight GUI frontend (<50MB) handles both user interface and system management operations directly. This architecture enables touch-first operation, hardware control integration (GPIO/PWM/MCU), and profile-based system reproduction across devices.

**Evolution Path**: As the Workmesh ecosystem matures, system management responsibilities (package management, service control, hardware abstraction) will migrate to dedicated Workmesh daemon services. SystemWeaver will then evolve into "Flow" - a pure desktop environment that communicates with these backend daemons. This separation will create a cleaner architecture where Flow focuses exclusively on user interface and experience, while Workmesh handles privileged system operations.

## Vision

**Current Phase**: Efficient system management application that replaces heavy desktop environments with a 30-50MB footprint.

**Future Evolution**: Pure desktop environment ("Flow") focused exclusively on user interface, working with Workmesh daemon services for system operations.

## Future Vision

SystemWeaver's evolution reflects a broader architectural transition in the WorkFlows ecosystem. Currently, SystemWeaver handles both desktop environment and system management responsibilities to validate the integrated approach. As the platform matures, these concerns will separate: Workmesh daemon services will handle privileged system operations (package management, service control, hardware abstraction), while SystemWeaver evolves into "Flow" - a pure desktop environment.

This evolution enables targeting different market segments appropriately: Flow (the desktop environment) can pursue both consumer desktop computing and industrial/SBC applications with <100MB footprint, while Workmesh (system management) serves enterprise customers requiring Docker-style reproducibility and bare-metal orchestration. The current SystemWeaver validates both use cases in a single application before architectural separation optimizes each domain.

## Branding

- **Current Name:** SystemWeaver (system management + desktop environment)
- **Future Name:** Flow or Weaver (pure desktop environment after Workmesh separation)
- **Mascot:** Origami boat (purposeful movement, lightweight, flows with intention) or Baya weaver bird
- **Philosophy:** Smooth, adaptive, effortless interaction

## Status

Early development - private repository

## Documentation

- **[PROPOSAL.md](docs/PROPOSAL.md)** - Complete technical specification, architecture design, and feature roadmap. Defines Flow's core capabilities including profile-based system management, hardware control integration, and desktop environment evolution stages.

- **[MULTI_TARGET_ARCHITECTURE.md](docs/MULTI_TARGET_ARCHITECTURE.md)** - Multi-target remote control architecture. Describes how SystemWeaver operates as a thin client controlling local or remote machines transparently, with seamless target switching and no explicit SSH/authentication per operation.

- **[DESKTOP_COMPONENTS.md](docs/DESKTOP_COMPONENTS.md)** - Desktop environment component tracking. Lists all UI components, settings views, utilities, and their implementation status.

- **[STRATEGIC_ANALYSIS.md](docs/STRATEGIC_ANALYSIS.md)** - Market positioning analysis and competitive landscape assessment. Compares Flow against existing solutions and identifies unique value propositions in the resource-efficient GUI space for SBCs and compact devices.

- **[BUSINESS_STRATEGY.md](docs/BUSINESS_STRATEGY.md)** - Comprehensive go-to-market strategy and monetization plan. Outlines the path from open-source project to sustainable business through consulting, product tiers, and enterprise services.

---
*Part of the WorkFlows ecosystem*
