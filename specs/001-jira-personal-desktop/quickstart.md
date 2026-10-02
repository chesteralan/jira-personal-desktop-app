# Validation Quickstart

## Prerequisites

- Current Node LTS with the repository's locked package manager
- Rust stable toolchain and Tauri 2 platform prerequisites
- SQLite tooling for inspection only
- Optional Jira Cloud sandbox with a test account, API token, and OAuth app

Exact commands are established by M1 in `package.json`, `Cargo.toml`, and CI; this guide is the required scenario set.

## Local validation

1. Install locked dependencies and run formatting, frontend lint/typecheck/tests, and Rust format/clippy/tests.
2. Start the Tauri development app with mock mode.
3. Verify dashboard, issue details, search, filters, theme, and keyboard-only navigation.
4. Seed 10,000 fixture issues and confirm launch/search targets.
5. Restart offline and confirm cached data plus freshness warning.

## Jira sandbox validation

1. Connect using API-token auth; verify identity and that SQLite/logs do not contain the token.
2. Sync assigned issues across multiple pages; interrupt a run and confirm prior cache integrity.
3. Save/remove a board and validate custom JQL failure recovery.
4. Execute one permitted transition and comment; verify denied actions remain unavailable or fail safely.
5. Log out, restart, and confirm the credential and authenticated session are gone.
6. Repeat connection/restart/logout using OAuth PKCE, including revoked-refresh-token recovery.

## Platform release validation

For macOS, Windows, and Linux: install a release candidate, launch, complete cached mock smoke flow, test deep links and supported native capabilities, upgrade from the previous released schema, verify data retention, and uninstall. Verify signatures/checksums and that no build artifact contains credentials.

## Expected result

All constitutional gates and success criteria in `spec.md` pass, failures produce sanitized actionable errors, and the release candidate matrix has no unresolved critical/high defects.
