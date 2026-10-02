# Implementation Plan: Jira Personal Desktop App

**Branch**: `001-jira-personal-desktop` | **Date**: 2026-10-02 | **Spec**: [spec.md](spec.md)

## Summary

Deliver a local-first Tauri desktop client that renders a personal Jira work cache immediately, synchronizes Jira Cloud through an isolated provider, supports API-token and OAuth authentication, and packages production releases for macOS, Windows, and Linux. Delivery proceeds through vertical milestones with mock-data UI first, then connectivity, synchronization, boards/actions, native integrations, and production hardening.

## Technical Context

**Language/Version**: Rust stable (MSRV recorded in `rust-toolchain.toml`); TypeScript 5.x on active Node LTS  
**Primary Dependencies**: Tauri 2, React, Vite, Tailwind CSS v4, shadcn/ui, Zustand, TanStack Query, Tokio, Reqwest, SQLx SQLite, Serde, tracing, keyring, OAuth2/PKCE support  
**Storage**: Versioned SQLite database in the platform app-data directory; secrets in Keychain/Credential Manager/Secret Service  
**Testing**: cargo test/nextest, Vitest, React Testing Library, Playwright, Tauri WebDriver/platform smoke tests, mocked Jira HTTP server  
**Target Platform**: macOS 13+ (Apple Silicon and Intel where CI permits), Windows 10/11 x64, Ubuntu 22.04+/equivalent supported Linux packaging target  
**Project Type**: Cross-platform desktop application  
**Performance Goals**: cached launch <2 s p95; local search <300 ms p95 at 10k issues; interactive list scrolling at 60 fps; sync never blocks UI  
**Constraints**: offline-capable reads, one active account, untrusted Jira content, no plaintext secrets, platform capability differences, Jira rate limits  
**Scale/Scope**: one user/site, up to 10k cached issues, tens of saved boards, six primary views, periodic polling

## Constitution Check

- **Personal-Work Focus**: PASS — administration, reporting, issue creation, AI, attachments, and time tracking are excluded.
- **Local-First Resilience**: PASS — SQLite is read first; sync is atomic and freshness is explicit.
- **Secure Boundaries**: PASS — credential references only, redacted telemetry, sanitized content, least-privilege OAuth scopes.
- **Contract-First Integration**: PASS — typed IPC contract and `JiraProvider` boundary are explicit deliverables.
- **Incremental Quality**: PASS — milestones are independently demonstrable and contain automated exit gates.

Post-design check: PASS. No constitutional exception is required.

## Architecture

```text
React presentation → feature services/stores → typed Tauri IPC
                                       ↓
Rust application services → JiraProvider / repositories / sync coordinator
                            ↓                    ↓
                  secure credential store     SQLite
```

The React process owns presentation and transient UI state, never credentials or direct Jira requests. Rust application services own authorization, networking, persistence, synchronization, URL opening, notifications, and platform capabilities. Domain types are provider-neutral. Database writes for each sync scope occur in transactions and preserve the prior committed snapshot on failure.

## Milestones

| Milestone | Outcome | Exit gate |
|---|---|---|
| M0 Specification | Approved scope, contracts, data model, threat assumptions | Spec checklist and constitution pass |
| M1 Foundation | CI-enabled Tauri shell runs on all targets with mock data | Format, lint, typecheck, unit tests, platform builds |
| M2 Local-first UI | Dashboard, tasks, details, search, filters, themes, keyboard flow on repository data | Offline/cache E2E and accessibility checks |
| M3 API-token connection | Secure personal connection, identity, logout, Jira client contract | Keyring, redaction, auth integration tests |
| M4 Sync engine | Atomic manual/background sync, freshness, retry/rate-limit handling | Interruption, migration, 10k-item performance tests |
| M5 Boards and actions | Board curation, custom JQL, transitions, priority, comments | Permission and contract tests against Jira sandbox |
| M6 OAuth 3LO | PKCE browser flow, deep-link callback, refresh, revocation | OAuth threat-model and restart tests |
| M7 Native integrations | Tray/menu bar, notifications, deep links, auto-start capability matrix | Platform-specific smoke and fallback tests |
| M8 Production release | Signed packages, updates, observability, docs, rollback and release automation | Full release candidate matrix and security review |

## Project Structure

```text
src/
├── app/                 # providers, router, app shell
├── components/          # shared accessible UI
├── features/            # dashboard, issues, boards, search, settings, auth
├── services/            # typed Tauri invoke/event adapter
├── stores/              # minimal client UI state
├── types/               # frontend contract types
└── test/

src-tauri/
├── migrations/
├── src/
│   ├── application/     # use cases and DTO mapping
│   ├── commands/        # thin Tauri IPC commands
│   ├── domain/          # provider-neutral entities and errors
│   ├── infrastructure/  # Jira Cloud, SQLite, keyring, platform adapters
│   ├── sync/            # scheduler and coordinator
│   └── lib.rs
└── tests/               # contract and integration tests

tests/
├── e2e/
├── fixtures/
└── platform/
```

**Structure Decision**: A single Tauri repository keeps web assets and Rust core independently testable while preserving one distributable. Feature-oriented React modules prevent global-store growth; ports/adapters in Rust isolate Jira and platform dependencies.

## Delivery Strategy

M1–M2 produce an offline demo; M3–M4 produce the first usable personal alpha; M5 forms the functional beta; M6–M7 complete product scope; M8 is the production gate. Each milestone uses a release-like artifact and records evidence in `docs/development-log.md`. Feature flags guard incomplete OAuth/native features without forking architecture.

## Risk Register

| Risk | Mitigation |
|---|---|
| OAuth desktop callback and refresh complexity | Authorization Code + PKCE, loopback/deep-link strategy per platform, state/nonce validation, threat-model tests |
| Jira API variability and permissions | Provider boundary, capability discovery, fixture corpus, sandbox contract suite |
| Cross-platform keyring/tray differences | Capability adapter, explicit fallback UX, per-platform CI smoke tests |
| Cache corruption or schema drift | Transactional sync, checksummed migrations, backup/recovery procedure, migration test matrix |
| Rate limits and large tenants | Pagination, bounded concurrency, conditional/incremental queries, backoff with jitter |
| Untrusted Jira rich text | Render a strict supported subset as text/sanitized nodes; never inject raw HTML |
| Release signing and update compromise | Protected CI environments, checksum/signature verification, least-privilege release credentials |

## Complexity Tracking

No constitution violations. TanStack Query is limited to server/IPC async state; Zustand is limited to durable UI preferences and navigation state to avoid overlapping state ownership.
