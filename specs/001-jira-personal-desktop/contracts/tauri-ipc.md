# Tauri IPC Contract

All commands return a typed envelope equivalent to `Result<T, AppError>`. `AppError` contains stable `code`, safe `message`, `retryable`, optional `retry_after_ms`, and `correlation_id`; it never contains secrets or raw sensitive responses.

## Authentication

- `auth_connect_api_token({ siteUrl, email, token }) -> AccountView` (token is accepted only at the Rust boundary and immediately placed in secure storage)
- `auth_begin_oauth({ siteUrl }) -> { authorizationUrl, stateId }`
- `auth_complete_oauth({ callbackUrl }) -> AccountView`
- `auth_status() -> ConnectionStatus`
- `auth_logout({ removeLocalData }) -> void`

## Work and synchronization

- `dashboard_get() -> DashboardView`
- `issues_list(IssueQuery) -> Page<IssueSummary>`
- `issue_get({ key }) -> IssueDetail`
- `sync_start({ scope, reason }) -> SyncRunView`
- `sync_status() -> SyncStatusView`
- Events: `sync://state`, `sync://progress`, `auth://state`, `issue://changed`.

## Boards and search

- `boards_discover({ query, cursor }) -> Page<BoardSummary>`
- `boards_saved() -> BoardView[]`
- `board_save(BoardConfiguration) -> BoardView`
- `board_remove({ boardId }) -> void`
- `board_validate_jql({ jql }) -> ValidationResult`
- `search_issues({ query, limit }) -> IssueSummary[]`

## Actions and platform

- `issue_transitions({ key }) -> Transition[]`
- `issue_transition({ key, transitionId }) -> IssueDetail`
- `issue_set_priority({ key, priorityId }) -> IssueDetail`
- `issue_add_comment({ key, body }) -> CommentView`
- `open_external({ url }) -> void` accepts only validated HTTPS Jira URLs.
- `platform_capabilities() -> CapabilityMatrix`
- `preferences_get() / preferences_update(Patch) -> Preferences`

## Compatibility

Commands and DTOs are versioned with the app. Additive optional fields are backward compatible; removals/renames require coordinated frontend/core changes and migration tests. Runtime payload validation is required at the IPC boundary.
