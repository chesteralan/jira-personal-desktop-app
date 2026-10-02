# Data Model

## Storage rules

All timestamps use UTC RFC 3339 text; Jira identifiers remain opaque text; secrets are represented only by credential-store references. Migrations are forward-only in production with tested backup/recovery. Foreign keys are enabled. Sync commits are transactional.

## Entities

### accounts
`id`, `site_url`, `cloud_id`, `auth_method`, `credential_ref`, `active`, `created_at`, `updated_at`, `last_verified_at`. One active account is enforced.

### users
`account_id`, `jira_account_id`, `display_name`, `email`, `avatar_url`, `active`, `synced_at`. Unique on account/Jira user.

### projects
`account_id`, `jira_id`, `key`, `name`, `project_type`, `synced_at`.

### boards
`account_id`, `jira_id`, `name`, `board_type`, `project_id`, `location_json`, `saved`, `enabled`, `only_my_issues`, `custom_jql`, `synced_at`.

### issues
`account_id`, `jira_id`, `key`, `summary`, `description_json`, `status_id`, `priority_id`, `project_id`, `assignee_id`, `due_date`, `created_at`, `updated_at`, `jira_url`, `raw_json`, `cached_at`, `deleted_at`. Indexes cover assignee/status/project/due/updated and case-insensitive key/summary search; FTS may be added after benchmark evidence.

### statuses / priorities / sprints / labels
Provider metadata linked to issues through `issue_sprints` and `issue_labels` join tables. Names are presentation metadata, not stable identifiers.

### saved_filters
`id`, `account_id`, `name`, `scope`, `criteria_json`, `sort_json`, `custom_jql`, `created_at`, `updated_at`.

### sync_runs
`id`, `account_id`, `trigger`, `scope`, `state`, `started_at`, `finished_at`, `cursor`, `received_count`, `changed_count`, `error_kind`, `retry_after`, `correlation_id`. Error detail is sanitized.

### sync_state
`account_id`, `scope`, `watermark`, `last_success_at`, `last_attempt_at`, `next_attempt_at`, `consecutive_failures`, `schema_version`.

### preferences
`key`, `value_json`, `updated_at`; validated against typed application defaults.

### notification_events
`id`, `account_id`, `issue_id`, `kind`, `fingerprint`, `observed_at`, `notified_at`; fingerprint prevents duplicate notifications.

## State transitions

- **Account**: disconnected → connecting → connected → reauth-required → disconnected.
- **Sync run**: queued → running → succeeded | failed | cancelled. Only one running run per account.
- **Issue write**: idle → submitting → confirmed | rejected. Rejected writes restore the last confirmed representation.
- **Migration**: pending → backed-up → applying → verified | recovery-required.

## Retention

Keep current issue/cache rows until Jira confirms deletion/inaccessibility or the account is removed. Keep recent sync summaries and bounded redacted logs; purge raw JSON when no longer needed for compatibility. Account removal deletes local domain data and credential references after explicit confirmation.
