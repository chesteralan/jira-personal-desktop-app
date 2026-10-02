# Research Decisions

## Jira provider boundary

**Decision**: Define a provider-neutral `JiraProvider` port in Rust with Jira Cloud as the first adapter.  
**Rationale**: Keeps Cloud authentication, pagination, REST versions, and Agile APIs out of application workflows.  
**Alternatives considered**: Direct calls in Tauri commands (fast initially, tightly coupled); frontend calls (exposes secrets and weakens offline ownership).

## Authentication sequence

**Decision**: Deliver API-token authentication first, then OAuth 2.0 Authorization Code with PKCE and browser-based consent. Keep a common credential-reference model.  
**Rationale**: API tokens unlock a personal alpha; OAuth provides the safer scalable production path.  
**Alternatives considered**: OAuth only (delays usable alpha); API token only (insufficient long-term product posture).

## Local persistence

**Decision**: Use SQLx with SQLite, bundled versioned migrations, WAL mode where supported, foreign keys, transactional snapshot updates, and raw response storage only when redacted and justified.  
**Rationale**: Compile-time query support and explicit Rust-owned persistence fit the security boundary.  
**Alternatives considered**: Frontend database plugin (weakens layering); ORM-heavy model (unneeded abstraction); JSON-only cache (poor querying/migrations).

## State ownership

**Decision**: SQLite/repositories are durable truth; TanStack Query manages asynchronous IPC results; Zustand manages small UI-only state.  
**Rationale**: Avoids duplicating issue data across global frontend stores.  
**Alternatives considered**: Zustand for all data (cache invalidation complexity); Redux (unnecessary ceremony).

## Synchronization

**Decision**: A single-flight coordinator performs paginated pulls with bounded concurrency, transactional upserts, explicit run records, cancellation, exponential backoff with jitter, and rate-limit awareness. Scheduler pauses during sleep/offline and triggers catch-up on resume.  
**Rationale**: Prevents overlap and partial cache presentation.  
**Alternatives considered**: Timer-driven independent fetches (race-prone); destructive full replacement (data-loss risk).

## Rich Jira content

**Decision**: Convert Atlassian Document Format to a strict internal display model; unsupported nodes fall back to plain text. Never render raw Jira HTML.  
**Rationale**: Jira content is untrusted and ADF support can evolve incrementally.  
**Alternatives considered**: `dangerouslySetInnerHTML` with sanitization (larger attack surface); plain text only forever (poor usability).

## Cross-platform distribution

**Decision**: Produce native Tauri bundles per platform in isolated CI jobs: signed/notarized macOS, signed Windows installer, and selected Linux AppImage/deb artifacts. Use a signed update manifest only after signing/recovery procedures are tested.  
**Rationale**: Platform signing toolchains and capabilities differ and cannot be validated from one runner.  
**Alternatives considered**: One universal job (not feasible); unsigned production packages (unacceptable trust and UX).

## Observability and privacy

**Decision**: Structured local logs with deny-by-default field policy, correlation IDs, bounded retention, user-triggered redacted diagnostic export, and opt-in crash reporting only.  
**Rationale**: Debuggability without leaking Jira content or credentials.  
**Alternatives considered**: Always-on remote telemetry (privacy mismatch); no diagnostics (operationally weak).

## Dependency policy

**Decision**: Lock all dependencies, require license and vulnerability scanning, document high-trust dependencies, and avoid releases younger than seven days unless a reviewed security fix requires an exception.  
**Rationale**: Desktop software ships privileged local code and needs supply-chain discipline.  
**Alternatives considered**: Floating versions (non-reproducible); blanket bans on dependencies (impractical).
