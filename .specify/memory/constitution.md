<!--
Sync Impact Report
- Version change: template → 1.0.0
- Added principles: Personal-Work Focus, Local-First Resilience, Secure Boundaries, Contract-First Integration, Incremental Quality
- Added sections: Product and Platform Constraints; Delivery and Quality Gates
- Removed sections: none
- Deferred items: none
-->
# Jira Personal Desktop App Constitution

## Core Principles

### I. Personal-Work Focus
Every feature MUST help an individual answer what Jira work needs attention now. The product MUST remain a focused personal workspace rather than reproduce Jira administration, reporting, or project-management capabilities. New scope MUST demonstrate direct value to the authenticated user's daily workflow.

### II. Local-First Resilience
Cached data MUST render before network synchronization when available. Read workflows MUST remain usable during temporary Jira outages, stale data MUST be labeled with its age, and write actions MUST be disabled or safely deferred when their outcome cannot be confirmed. Jira remains the source of truth.

### III. Secure Boundaries (NON-NEGOTIABLE)
Passwords, access tokens, refresh tokens, API tokens, and authorization headers MUST NOT be stored in plaintext, committed, rendered, or logged. Secrets MUST be held by operating-system credential storage. Jira content MUST be treated as untrusted input. Least-privilege permissions, explicit logout/revocation behavior, and sanitized diagnostics are mandatory.

### IV. Contract-First Integration
Frontend-to-core commands, persisted data, and Jira provider behavior MUST have explicit typed contracts. Jira Cloud-specific behavior MUST remain behind a provider boundary so Server/Data Center support can be added without rewriting product workflows. Schema, command, and provider contract changes MUST include compatibility and migration decisions.

### V. Incremental, Test-First Quality
Work MUST ship as independently demonstrable vertical slices. For bugs and contract behavior, a failing test MUST precede the fix where test infrastructure exists. Each milestone MUST satisfy relevant unit, integration, end-to-end, accessibility, security, and packaging checks before it is considered complete. Complexity without an immediate requirement MUST be rejected.

## Product and Platform Constraints

The initial production architecture MUST use Tauri 2, Rust, React, TypeScript, Vite, Tailwind CSS, accessible lightweight components, and SQLite through the Rust layer unless an architecture decision record justifies a change. Production distribution targets macOS, Windows, and Linux. API-token authentication MUST provide the first usable personal connection flow; OAuth 2.0 3LO MUST be added as the production-grade alternative. Platform-specific capabilities MUST degrade explicitly when unavailable.

Launch with cache SHOULD complete within two seconds on supported hardware. The UI MUST be keyboard accessible, support system/light/dark appearance, and avoid blocking first render on Jira. Dependencies MUST be necessary, maintained, license-compatible, pinned through lockfiles, and selected with supply-chain risk in mind.

## Delivery and Quality Gates

Every feature follows specification, clarification, planning, tasks, implementation, and consistency analysis. Architectural decisions that affect security, data ownership, portability, or operability MUST be recorded. Database changes MUST be versioned and migration-tested. External and IPC boundaries MUST have contract tests; synchronization and authentication MUST have integration tests; critical user journeys MUST have end-to-end tests.

A milestone is complete only when formatting, linting, type checking, Rust checks, automated tests, security checks, and supported-platform builds pass; documentation and acceptance evidence are current; and no unresolved critical or high-severity defect remains. Releases MUST be reproducible, signed where the platform supports it, accompanied by rollback/recovery notes, and must not expose secrets in artifacts or logs.

## Governance

This constitution supersedes conflicting implementation conventions. Amendments require a documented rationale, impact analysis, migration plan when applicable, and semantic version update. MAJOR versions remove or redefine a principle incompatibly, MINOR versions add or materially expand governance, and PATCH versions clarify without changing obligations.

Plans and pull-request reviews MUST explicitly check constitutional compliance. Any exception MUST identify the violated rule, business necessity, risk owner, mitigation, and expiry milestone. Compliance is reviewed at every milestone exit and before each production release.

**Version**: 1.0.0 | **Ratified**: 2026-10-02 | **Last Amended**: 2026-10-02
