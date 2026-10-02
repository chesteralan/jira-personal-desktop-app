# Architecture

## Baseline

Jira Personal is a local-first cross-platform desktop application. The React frontend presents cached work and sends typed commands through Tauri IPC. The Rust core owns Jira networking, credentials, persistence, synchronization, notifications, and platform integrations.

```text
React UI → typed IPC → Rust application services
                           ├── provider-neutral domain
                           ├── Jira Cloud adapter
                           ├── SQLite repositories
                           └── platform adapters
```

## Decisions

### Single Tauri repository

Frontend and core share one release lifecycle while remaining independently testable. Feature-oriented frontend modules and layered Rust modules avoid coupling presentation to infrastructure.

### Local-first ownership

SQLite becomes the durable local read model in M2. Cached data renders before synchronization and the last committed snapshot survives failed sync. Jira remains authoritative.

### Secure process boundary

React never receives credentials and never contacts Jira directly. Operating-system secure storage holds secrets; Rust exchanges only safe view models and structured errors over IPC.

### Tailwind v4 semantic theme

Theme variables are defined at root level, mapped through `@theme inline`, and consumed by semantic utilities. Light, dark, and system modes share component styles without hard-coded dark variants.

### Least-privilege shell

M1 grants only Tauri core window permissions and enforces a restrictive production CSP. Network, opener, keyring, notification, and deep-link capabilities are added only with the milestone that requires them.

## Module boundaries

- `src/app/`: composition, providers, keyboard and routing shell.
- `src/components/`: shared accessible UI primitives.
- `src/features/`: vertically organized product workflows.
- `src/services/`: typed Tauri IPC adapter.
- `src-tauri/src/domain/`: provider-neutral types and policies.
- `src-tauri/src/application/`: use cases and transaction boundaries.
- `src-tauri/src/commands/`: validation and IPC translation only.
- `src-tauri/src/infrastructure/`: Jira, SQLite, credentials, and OS adapters.
- `src-tauri/src/sync/`: single-flight scheduling and coordination.

Architectural changes affecting security, data ownership, portability, or these boundaries require this document and the implementation plan to be updated.
