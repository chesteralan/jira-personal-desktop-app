# Jira Personal Desktop App — Implementation Documentation

The initial concept in [INITIAL.md](INITIAL.md) has been converted into a Spec Kit implementation package covering product scope through production release.

## Canonical artifacts

| Artifact | Purpose |
|---|---|
| [Constitution](../.specify/memory/constitution.md) | Non-negotiable product, security, architecture, and quality rules |
| [Product specification](../specs/001-jira-personal-desktop/spec.md) | User stories, requirements, edge cases, assumptions, and success criteria |
| [Implementation plan](../specs/001-jira-personal-desktop/plan.md) | Architecture, technology decisions, milestones, risks, and delivery strategy |
| [Research](../specs/001-jira-personal-desktop/research.md) | Decisions and rejected alternatives |
| [Data model](../specs/001-jira-personal-desktop/data-model.md) | Durable entities, relationships, lifecycle, and retention |
| [Tauri IPC contract](../specs/001-jira-personal-desktop/contracts/tauri-ipc.md) | Frontend/core command and error contract |
| [Jira provider contract](../specs/001-jira-personal-desktop/contracts/jira-provider.md) | Jira abstraction invariants and verification scope |
| [Task backlog](../specs/001-jira-personal-desktop/tasks.md) | 83 dependency-ordered implementation tasks across eight milestones |
| [Validation quickstart](../specs/001-jira-personal-desktop/quickstart.md) | Local, Jira sandbox, and platform release validation |
| [Requirements checklist](../specs/001-jira-personal-desktop/checklists/requirements.md) | Spec readiness evidence |

## Delivery sequence

1. **M1 Foundation** — reproducible cross-platform Tauri shell and CI.
2. **M2 Local-first UI** — offline dashboard, issue workflows, search, and accessibility.
3. **M3 API-token connection** — secure first real Jira connection.
4. **M4 Synchronization** — atomic cache, scheduling, rate limits, and recovery.
5. **M5 Boards and actions** — board curation and permission-aware writes.
6. **M6 OAuth 3LO** — PKCE, callbacks, refresh, revocation, and threat review.
7. **M7 Native integrations** — tray/menu bar, notifications, deep links, and auto-start.
8. **M8 Production release** — security, performance, signing, packaging, updates, docs, and release evidence.

Implement tasks in milestone order and stop at each exit gate. The first useful release cut is the M1–M2 offline demo; the first connected alpha is M1–M4. Production requires all M8 gates and publication task T083.

## Using Spec Kit

The repository is initialized with Spec Kit under `.specify/`. Devin-compatible generic command definitions are under `.devin/commands/`. Continue refinement with the `speckit.clarify`, `speckit.plan`, `speckit.tasks`, and `speckit.analyze` workflows; implementation should only begin after changed artifacts remain mutually consistent.
