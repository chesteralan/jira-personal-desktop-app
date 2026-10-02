# Tasks: Jira Personal Desktop App

**Input**: Design artifacts in `specs/001-jira-personal-desktop/`  
**Method**: Tests precede behavior; each milestone ends with a runnable, reviewed checkpoint.

## Milestone M1 — Foundation and Cross-Platform Shell

**Goal**: Reproducible Tauri app, mock data, quality gates, and CI builds.

- [ ] T001 Initialize the Tauri 2 React TypeScript project and lock toolchains in `package.json`, `pnpm-lock.yaml`, `Cargo.toml`, and `rust-toolchain.toml`
- [ ] T002 [P] Configure TypeScript, Vite aliases, and test environments in `tsconfig.json`, `vite.config.ts`, and `vitest.config.ts`
- [ ] T003 [P] Configure Rust formatting, linting, and test profiles in `rustfmt.toml`, `clippy.toml`, and `src-tauri/Cargo.toml`
- [ ] T004 [P] Configure Tailwind v4 and accessible component foundations in `src/styles/globals.css` and `components.json`
- [ ] T005 Define least-privilege Tauri capabilities and CSP in `src-tauri/capabilities/default.json` and `src-tauri/tauri.conf.json`
- [ ] T006 Create the React and Rust module structure from `plan.md` in `src/` and `src-tauri/src/`
- [ ] T007 [P] Add deterministic Jira fixture builders in `tests/fixtures/` and `src/test/fixtures.ts`
- [ ] T008 [P] Add CI jobs for frontend and Rust quality gates in `.github/workflows/ci.yml`
- [ ] T009 Add macOS, Windows, and Linux compile/package smoke jobs in `.github/workflows/platform-build.yml`
- [ ] T010 Record build, test, and release commands in `AGENTS.md` and baseline decisions in `docs/architecture.md`

**Exit gate**: Mock shell launches; format, lint, typecheck, unit tests, and platform compile jobs pass.

## Milestone M2 — Local-First Personal Workspace

**Goal**: Independently usable dashboard and issue browsing from local data.

- [ ] T011 [P] [US1] Write domain model tests for users, issues, freshness, and dashboard grouping in `src-tauri/src/domain/tests.rs`
- [ ] T012 [P] [US1] Write migration/repository integration tests in `src-tauri/tests/repository.rs`
- [ ] T013 [US1] Create initial normalized schema and indexes in `src-tauri/migrations/0001_initial.sql`
- [ ] T014 [US1] Implement SQLite connection, migration, and transaction management in `src-tauri/src/infrastructure/database/mod.rs`
- [ ] T015 [US1] Implement issue, user, preference, and sync-state repositories in `src-tauri/src/infrastructure/database/repositories/`
- [ ] T016 [US1] Implement provider-neutral domain entities and dashboard service in `src-tauri/src/domain/` and `src-tauri/src/application/dashboard.rs`
- [ ] T017 [P] [US1] Write IPC contract tests for dashboard and issue queries in `src-tauri/tests/ipc_read_contract.rs`
- [ ] T018 [US1] Implement typed read commands and safe error envelope in `src-tauri/src/commands/issues.rs` and `src-tauri/src/application/error.rs`
- [ ] T019 [P] [US1] Implement the typed invoke/event adapter in `src/services/tauri.ts` and `src/types/contracts.ts`
- [ ] T020 [P] [US1] Build accessible app shell, sidebar, theme, and routing in `src/app/` and `src/components/layout/`
- [ ] T021 [US1] Build Today dashboard and issue cards in `src/features/dashboard/`
- [ ] T022 [US1] Build My Tasks filtering/sorting and issue detail panel in `src/features/issues/`
- [ ] T023 [P] [US1] Build keyboard command handling and focus management in `src/app/keyboard.ts` and `src/components/command-menu/`
- [ ] T024 [P] [US1] Write React interaction/accessibility tests in `src/features/dashboard/dashboard.test.tsx` and `src/features/issues/issues.test.tsx`
- [ ] T025 [US1] Write offline launch and cache browser E2E tests in `tests/e2e/local-workspace.spec.ts`
- [ ] T026 [US1] Benchmark 10k-issue launch/search behavior in `src-tauri/benches/local_queries.rs` and `tests/e2e/performance.spec.ts`

**Exit gate**: Cache-only application satisfies US1, keyboard, accessibility, and performance targets.

## Milestone M3 — Secure API-Token Connection

**Goal**: First real Jira personal alpha with secure credential lifecycle.

- [ ] T027 [P] [US2] Write credential-store and redaction tests in `src-tauri/tests/credential_security.rs`
- [ ] T028 [P] [US2] Write Jira Cloud fixture contract tests for identity and errors in `src-tauri/tests/jira_provider_contract.rs`
- [ ] T029 [US2] Define `JiraProvider` and provider error taxonomy in `src-tauri/src/domain/jira_provider.rs`
- [ ] T030 [US2] Implement hardened HTTP client, timeouts, user agent, and redaction in `src-tauri/src/infrastructure/jira/http.rs`
- [ ] T031 [US2] Implement API-token identity verification in `src-tauri/src/infrastructure/jira/cloud.rs`
- [ ] T032 [US2] Implement cross-platform secure credential adapter in `src-tauri/src/infrastructure/credentials/mod.rs`
- [ ] T033 [US2] Implement connect/status/logout commands and local-data removal transaction in `src-tauri/src/commands/auth.rs`
- [ ] T034 [P] [US2] Build connection onboarding and status settings in `src/features/auth/` and `src/features/settings/connection.tsx`
- [ ] T035 [US2] Add connect/restart/logout integration coverage in `tests/e2e/api-token-auth.spec.ts`
- [ ] T036 [US2] Add log/database/artifact secret scanning to `.github/workflows/security.yml`

**Exit gate**: API-token connection survives restart, logout removes credentials, and security tests find no secret leakage.

## Milestone M4 — Synchronization, Search, and Offline Recovery

**Goal**: Reliable assigned-work cache with manual/background synchronization.

- [ ] T037 [P] [US3] Write pagination, rate-limit, cancellation, and partial-failure provider tests in `src-tauri/tests/jira_search_contract.rs`
- [ ] T038 [P] [US3] Write single-flight and interrupted-transaction sync tests in `src-tauri/tests/sync_engine.rs`
- [ ] T039 [US3] Implement safe JQL builder and paginated assigned-issue retrieval in `src-tauri/src/infrastructure/jira/search.rs`
- [ ] T040 [US3] Implement provider-to-domain mapping and strict ADF conversion in `src-tauri/src/infrastructure/jira/mapping.rs`
- [ ] T041 [US3] Implement transactional sync coordinator and run records in `src-tauri/src/sync/coordinator.rs`
- [ ] T042 [US3] Implement scheduler, sleep/resume handling, retries, jitter, and offline detection in `src-tauri/src/sync/scheduler.rs`
- [ ] T043 [US3] Implement manual sync command and progress events in `src-tauri/src/commands/sync.rs`
- [ ] T044 [US3] Implement indexed local search and filters in `src-tauri/src/application/search.rs`
- [ ] T045 [P] [US3] Build global search, sync status, stale/offline banners, and retry UI in `src/features/search/` and `src/features/sync/`
- [ ] T046 [US3] Add restart, outage, rate-limit, and interrupted-sync E2E tests in `tests/e2e/synchronization.spec.ts`

**Exit gate**: Manual and scheduled sync are atomic; cached work survives all tested failure paths.

## Milestone M5 — Boards and Safe Quick Actions

**Goal**: Functional beta with curated boards and permission-aware writes.

- [ ] T047 [P] [US4] Write board discovery/JQL provider contract tests in `src-tauri/tests/boards_contract.rs`
- [ ] T048 [P] [US5] Write transitions, priority, and comment contract tests in `src-tauri/tests/actions_contract.rs`
- [ ] T049 [US4] Add board/filter schema migration in `src-tauri/migrations/0002_boards_filters.sql`
- [ ] T050 [US4] Implement board discovery, persistence, and JQL validation in `src-tauri/src/application/boards.rs`
- [ ] T051 [P] [US4] Build board picker, saved board navigation, and configuration in `src/features/boards/`
- [ ] T052 [US4] Add board persistence and invalid-JQL E2E tests in `tests/e2e/boards.spec.ts`
- [ ] T053 [US5] Implement transition/priority/comment provider operations without blind write retries in `src-tauri/src/infrastructure/jira/actions.rs`
- [ ] T054 [US5] Implement permission-aware action services and confirmed cache refresh in `src-tauri/src/application/issue_actions.rs`
- [ ] T055 [P] [US5] Build transition, priority, comment, and open-in-Jira controls in `src/features/issues/actions/`
- [ ] T056 [US5] Add permission, conflict, offline, and rollback E2E tests in `tests/e2e/issue-actions.spec.ts`

**Exit gate**: Boards persist and every quick action is capability-gated and reflects only confirmed Jira state.

## Milestone M6 — OAuth 2.0 3LO

**Goal**: Production-grade browser authorization and token refresh.

- [ ] T057 [P] [US2] Write PKCE, state validation, refresh rotation, expiry, and revocation tests in `src-tauri/tests/oauth.rs`
- [ ] T058 [US2] Implement OAuth configuration validation and least-privilege scopes in `src-tauri/src/infrastructure/oauth/config.rs`
- [ ] T059 [US2] Implement browser authorization with PKCE and state-bound callback in `src-tauri/src/infrastructure/oauth/flow.rs`
- [ ] T060 [US2] Implement platform callback/deep-link handling in `src-tauri/src/infrastructure/platform/deep_links.rs`
- [ ] T061 [US2] Implement secure refresh-token rotation and reconnect state in `src-tauri/src/infrastructure/oauth/tokens.rs`
- [ ] T062 [P] [US2] Add OAuth method selection and reconnect UX in `src/features/auth/oauth.tsx`
- [ ] T063 [US2] Add OAuth browser/restart/revocation E2E coverage in `tests/e2e/oauth.spec.ts`
- [ ] T064 [US2] Complete OAuth threat-model review in `docs/security/oauth-threat-model.md`

**Exit gate**: OAuth passes restart, refresh, callback-tampering, revocation, and secret-leakage tests.

## Milestone M7 — Native Desktop Integrations

**Goal**: Supported native convenience features with explicit fallbacks.

- [ ] T065 [P] [US6] Define and test platform capability matrix in `src-tauri/src/infrastructure/platform/capabilities.rs` and `tests/platform/capabilities.rs`
- [ ] T066 [US6] Implement tray/menu-bar view and issue activation in `src-tauri/src/infrastructure/platform/tray.rs`
- [ ] T067 [US6] Implement deduplicated configurable notifications in `src-tauri/src/infrastructure/platform/notifications.rs`
- [ ] T068 [US6] Implement auto-start preference with unsupported-state fallback in `src-tauri/src/infrastructure/platform/autostart.rs`
- [ ] T069 [P] [US6] Build native integration preferences in `src/features/settings/integrations.tsx`
- [ ] T070 [US6] Add per-platform tray, notification, deep-link, and auto-start smoke tests in `tests/platform/`

**Exit gate**: Capability matrix is accurate and missing platform features never break core work flows.

## Milestone M8 — Production Hardening and Release

**Goal**: Reproducible, secure, supportable public production release.

- [ ] T071 Add migration backup, verification, and recovery tests in `src-tauri/tests/migration_recovery.rs`
- [ ] T072 [P] Add dependency license, vulnerability, provenance, and age-policy checks in `.github/workflows/supply-chain.yml`
- [ ] T073 [P] Add bounded structured logging and redacted diagnostic export in `src-tauri/src/infrastructure/diagnostics/`
- [ ] T074 Complete WCAG 2.2 AA audit and regression suite in `tests/e2e/accessibility.spec.ts`
- [ ] T075 Complete performance profiling and budgets in `docs/performance.md` and `.github/workflows/performance.yml`
- [ ] T076 Configure macOS signing/notarization in `src-tauri/tauri.macos.conf.json` and `.github/workflows/release-macos.yml`
- [ ] T077 Configure Windows signing/installer in `src-tauri/tauri.windows.conf.json` and `.github/workflows/release-windows.yml`
- [ ] T078 Configure Linux AppImage/deb packaging in `src-tauri/tauri.linux.conf.json` and `.github/workflows/release-linux.yml`
- [ ] T079 Configure signed update manifests and rollback controls in `src-tauri/tauri.conf.json` and `.github/workflows/release.yml`
- [ ] T080 [P] Write installation, connection, privacy, troubleshooting, and recovery docs in `docs/user/` and `docs/operations/`
- [ ] T081 Run the full quickstart and release-candidate matrix, recording evidence in `docs/release-readiness.md`
- [ ] T082 Perform security review, verify zero critical/high findings, and record accepted residual risks in `docs/security/review.md`
- [ ] T083 Tag and publish the production release only after all milestone gates pass using `.github/workflows/release.yml`

**Exit gate**: Signed/checksummed artifacts install, upgrade, launch, pass critical journeys, and can be safely supported and rolled back.

## Dependencies and Execution Order

`M0 → M1 → M2`; M3 depends on M1 and integrates with M2; M4 depends on M2+M3; M5 depends on M4; M6 depends on M3 and may run alongside M5; M7 depends on the stable M2 shell and may run alongside M5/M6; M8 depends on all release-scope milestones.

Within each milestone: failing tests → domain/storage → services/provider → IPC → UI → E2E → exit gate. Tasks marked `[P]` touch independent files and may run concurrently.

## Recommended Release Cuts

1. **Offline demo**: M1–M2.
2. **Personal alpha**: M3–M4 with API-token auth.
3. **Functional beta**: M5 plus early platform smoke coverage.
4. **Release candidate**: M6–M7 and all M8 gates except publication.
5. **Production**: T083 after signed release-candidate approval.
