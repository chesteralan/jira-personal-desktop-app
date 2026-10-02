
Jira Personal Desktop App

Objective

Build a lightweight cross-platform desktop application that provides a focused personal view of Jira.

The application should not attempt to recreate Jira’s full web interface. Its primary purpose is to help a developer quickly see, organize, and manage only the Jira issues relevant to them.

The application should support:

* Viewing issues assigned to the authenticated Jira user
* Selecting and saving specific Jira boards
* Filtering issues by board/project/status
* Personal “My Work” dashboard
* Quick issue actions
* Opening issues directly in Jira
* Automatic synchronization
* Local caching
* macOS desktop/menu-bar experience

The UI should be fast, minimal, keyboard-friendly, and optimized for someone who works with Jira throughout the day.

⸻

1. Technology Stack

Use the following stack unless there is a strong technical reason to change it.

Desktop

* Tauri 2.x
* Rust
* macOS
* Support Apple Silicon
* Prepare the architecture for Windows/Linux support later

Frontend

* React
* TypeScript
* Vite
* Tailwind CSS
* shadcn/ui or similarly lightweight accessible UI components

State

Prefer:

* Zustand

Avoid unnecessary global state.

Local Storage

Use SQLite through the Tauri/Rust layer.

Store:

* Jira connection configuration
* Current user information
* Selected boards
* Saved filters
* Cached issues
* Application preferences
* Last synchronization timestamp

Do not store the Jira password.

Jira Integration

Use the official Jira REST APIs.

The application must support Jira Cloud initially.

Keep the Jira integration behind an abstraction so Jira Server/Data Center support can be added later.

⸻

2. High-Level Architecture

┌─────────────────────────────────────────────┐
│                 macOS App                   │
│                                             │
│  ┌───────────────────────────────────────┐  │
│  │              React UI                 │  │
│  │                                       │  │
│  │ Dashboard                             │  │
│  │ My Tasks                              │  │
│  │ Boards                                │  │
│  │ Issue Details                         │  │
│  │ Settings                              │  │
│  └───────────────────┬───────────────────┘  │
│                      │ Tauri IPC             │
│  ┌───────────────────▼───────────────────┐  │
│  │              Rust Core                │  │
│  │                                       │  │
│  │ Jira API Client                       │  │
│  │ Authentication                        │  │
│  │ SQLite Repository                     │  │
│  │ Sync Engine                           │  │
│  │ Notifications                         │  │
│  └───────────────────┬───────────────────┘  │
│                      │                       │
│               ┌──────▼──────┐               │
│               │   SQLite    │               │
│               └─────────────┘               │
└──────────────────────┬──────────────────────┘
                       │ HTTPS
                       ▼
                Jira REST API

⸻

3. Core Product Philosophy

The application should answer one question:

“What Jira work do I need to care about right now?”

Avoid unnecessary Jira functionality.

Do not build:

* Full Jira project administration
* Full Jira board configuration
* Complex Jira reporting
* Jira project management
* User administration
* Workflow administration
* Full issue creation wizard
* Full Jira settings

The application should be a personal Jira command center.

⸻

4. Main Navigation

Use a sidebar on desktop.

MY WORK
├── Today
├── My Tasks
├── In Progress
├── Review
└── Done
BOARDS
├── E-commerce
├── Funnels
├── Presells
└── + Add Board
OTHER
├── Search
└── Settings

The sidebar should be collapsible.

Keyboard shortcut:

⌘ + B

to toggle the sidebar.

⸻

5. Dashboard

The default screen should be Today / My Work.

Example:

Good afternoon, Alchie
Last synced 2 minutes ago                         [Sync]
┌─────────────────────────────────────────────┐
│ Needs Attention                         3   │
├─────────────────────────────────────────────┤
│ 🔴 TPT-7042   Update Snowplow tracking      │
│ 🟠 PRCM-2579  Checkout component update     │
│ 🟠 TPT-7101   Fix mobile funnel issue       │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│ In Progress                              4  │
├─────────────────────────────────────────────┤
│ TPT-7001                                      │
│ PRCM-2540                                     │
│ TPT-6981                                      │
│ TPT-6950                                      │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│ Review                                    2 │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│ Due This Week                            5 │
└─────────────────────────────────────────────┘

⸻

6. My Tasks

Display only issues assigned to the current Jira user.

Default JQL:

assignee = currentUser()
AND resolution = Unresolved
ORDER BY priority DESC, updated DESC

Allow filtering by:

* Status
* Priority
* Project
* Board
* Sprint
* Due date
* Labels

Allow sorting by:

* Priority
* Updated
* Created
* Due date
* Status

⸻

7. Task Cards

Each task should display:

┌──────────────────────────────────────────────┐
│ TPT-7042                           🔴 High   │
│                                              │
│ Update Snowplow tracking                     │
│                                              │
│ E-commerce · In Progress · Sprint 42        │
│                                              │
│ Updated 2h ago                    Due Oct 4  │
└──────────────────────────────────────────────┘

Show:

* Issue key
* Summary
* Priority
* Status
* Project
* Sprint
* Labels
* Due date
* Last updated

Avoid displaying excessive Jira metadata.

⸻

8. Issue Details

Clicking an issue opens a detail panel.

Example:

TPT-7042
Update Snowplow tracking
Status
[ In Progress ▼ ]
Priority
[ High ▼ ]
Sprint
Sprint 42
Labels
snowplow
tracking
Description
────────────────────────
...
Actions
[Add Comment]
[Change Status]
[Open in Jira]

The panel should preferably slide in from the right instead of navigating away from the current list.

⸻

9. Quick Actions

Support these actions directly from the application:

Status

Allow changing status when Jira permits the transition.

Example:

To Do
   ↓
In Progress
   ↓
Review
   ↓
Done

Do not assume these exact statuses exist.

Retrieve available transitions from Jira and display only valid transitions.

Priority

Allow changing issue priority if the authenticated Jira user has permission.

Comment

Provide a simple comment input:

Add a comment...
[Cancel] [Comment]

Open Jira

Open the issue’s Jira URL using the system browser.

⸻

10. Boards

Users should be able to add specific Jira boards.

Click:

+ Add Board

Then show available boards.

Search:

Search boards...

Example:

Select Boards
☑ E-commerce
☑ Funnels
☐ Presells
☐ Checkout
☐ QA

Save selected boards locally.

⸻

11. Board View

Selecting a board should show issues associated with that board, but prioritize the current user’s issues.

Example:

E-commerce
All        My Tasks        In Progress        Review
┌─────────────────────────────────────────────┐
│ TPT-7042                                    │
│ Update Snowplow tracking                    │
├─────────────────────────────────────────────┤
│ PRCM-2579                                   │
│ Checkout component update                   │
└─────────────────────────────────────────────┘

Provide an option:

☑ Only show my assigned issues

Default this to enabled.

⸻

12. Board Configuration

Each saved board should have:

Board Name
Board ID
Project
JQL
Enabled
Show only my tasks

Allow custom JQL.

Example:

project = TPT
AND assignee = currentUser()
AND resolution = Unresolved

Advanced users should be able to edit the JQL.

⸻

13. Search

Provide global keyboard-accessible search.

Shortcut:

⌘ + K

Search should support:

* Issue key
* Summary
* Project
* Labels

Example:

⌘ K
Search Jira...
> TPT-7042

Results:

TPT-7042
Update Snowplow tracking
In Progress · High

Press Enter to open the issue details.

Press:

⌘ + Enter

to open the issue directly in Jira.

⸻

14. Keyboard Navigation

Make the application keyboard-friendly.

Recommended shortcuts:

⌘ K       Global search
⌘ B       Toggle sidebar
⌘ R       Sync
⌘ ,       Settings
Esc       Close modal/panel
↑ / ↓     Navigate issues
Enter     Open issue
⌘ Enter   Open in Jira

Do not interfere with standard macOS shortcuts unnecessarily.

⸻

15. Synchronization

Implement background synchronization.

Initial synchronization:

Authenticate
     ↓
Get current user
     ↓
Get selected boards
     ↓
Get assigned issues
     ↓
Store locally
     ↓
Display UI

Subsequent synchronization:

Every 5 minutes

Also provide manual:

[ Sync ]

The UI should show:

Synced just now
Synced 2 minutes ago
Syncing...
Offline

⸻

16. Offline Support

The application should remain usable when Jira is temporarily unavailable.

If offline:

* Display cached tasks
* Display last synchronization time
* Disable Jira write operations
* Allow browsing cached data
* Clearly show:

Offline
Showing data from 12 minutes ago.

Do not silently show stale data as current.

⸻

17. SQLite Schema

Design a normalized local database.

Suggested tables:

users
boards
issues
issue_statuses
issue_priorities
issue_labels
issue_sprints
settings
sync_state

Example:

CREATE TABLE issues (
    id TEXT PRIMARY KEY,
    key TEXT NOT NULL,
    summary TEXT NOT NULL,
    description TEXT,
    status_id TEXT,
    status_name TEXT,
    priority_id TEXT,
    priority_name TEXT,
    project_id TEXT,
    project_key TEXT,
    project_name TEXT,
    assignee_id TEXT,
    assignee_name TEXT,
    due_date TEXT,
    updated_at TEXT,
    created_at TEXT,
    jira_url TEXT,
    raw_json TEXT,
    cached_at TEXT
);

Store the original Jira response where useful so the schema can evolve without immediately requiring migrations.

⸻

18. Authentication

Use Jira’s recommended authentication mechanism for Jira Cloud.

The application should not request or store a user’s Jira password.

Authentication credentials/tokens must be stored securely using the operating system’s secure credential storage/keychain where appropriate.

Do not place secrets in:

* React source
* .env committed to git
* SQLite plaintext
* Logs
* Error messages

Provide a logout action that clears locally stored authentication credentials.

⸻

19. API Layer

Create a dedicated Jira API abstraction.

Suggested Rust structure:

src-tauri/
├── src/
│   ├── main.rs
│   ├── commands/
│   │   ├── auth.rs
│   │   ├── issues.rs
│   │   ├── boards.rs
│   │   ├── search.rs
│   │   └── settings.rs
│   │
│   ├── jira/
│   │   ├── client.rs
│   │   ├── auth.rs
│   │   ├── issues.rs
│   │   ├── boards.rs
│   │   └── users.rs
│   │
│   ├── database/
│   │   ├── mod.rs
│   │   ├── migrations.rs
│   │   ├── issues.rs
│   │   └── boards.rs
│   │
│   └── sync/
│       ├── mod.rs
│       └── scheduler.rs

React:

src/
├── components/
│   ├── layout/
│   ├── sidebar/
│   ├── dashboard/
│   ├── issues/
│   ├── boards/
│   ├── search/
│   └── settings/
│
├── pages/
│   ├── Dashboard.tsx
│   ├── MyTasks.tsx
│   ├── Board.tsx
│   └── Settings.tsx
│
├── stores/
│   ├── issueStore.ts
│   ├── boardStore.ts
│   ├── authStore.ts
│   └── settingsStore.ts
│
├── services/
│   └── tauri.ts
│
├── types/
│   └── jira.ts
│
└── App.tsx

⸻

20. Error Handling

Errors should be understandable.

Instead of:

Error: HTTP 401

Display:

Jira authentication expired.
Please reconnect your Jira account.

For API failures:

Unable to sync Jira.
Your cached tasks are still available.
[Retry]

For permission errors:

You don't have permission to modify this issue.
You can still open it in Jira.

Never expose authentication tokens or sensitive API responses in error messages.

⸻

21. Loading States

Use skeletons rather than blank screens.

Example:

┌─────────────────────────────────┐
│ ███████████████████             │
│ ███████████                     │
│ ████████                        │
└─────────────────────────────────┘

Avoid excessive loading spinners.

Because data is cached locally, the application should display cached data immediately and synchronize in the background.

⸻

22. macOS Menu Bar

Add a menu-bar application mode.

Example:

☑ Jira Personal

Clicking the icon displays:

My Work
In Progress     4
Review          2
Due Today       1
────────────────
Open Dashboard
Sync Now
Settings
Quit

Clicking an issue from the menu should open the application and display that issue.

⸻

23. Notifications

Optional but recommended.

Support notifications for:

* Assigned new issue
* Issue status changed
* Issue mentioned
* Due date approaching

Make notifications configurable.

Default:

Notifications: Enabled

Allow:

New assignments
Status changes
Mentions
Due dates

⸻

24. Appearance

Support:

* Light mode
* Dark mode
* System mode

Default:

System

The application should feel like a native macOS application rather than a web page inside a window.

Use:

* Compact spacing
* Subtle borders
* Native-feeling controls
* Keyboard navigation
* Smooth transitions
* Minimal visual noise

⸻

25. Performance Requirements

The application should feel instantaneous when opening.

Target:

Application launch
< 2 seconds

when local cache exists.

The dashboard should not wait for Jira API responses.

Use:

SQLite cache → render UI → background sync

rather than:

Jira API → wait → render UI

⸻

26. Security Requirements

Never log:

* Access tokens
* Refresh tokens
* API credentials
* Authorization headers
* Jira passwords

Sanitize Jira content before rendering HTML.

Be careful with:

* Jira descriptions
* Comments
* Attachments
* User-generated content

Do not allow arbitrary Jira content to execute JavaScript inside the application.

⸻

27. Testing

Implement tests for:

Rust

* Jira API client
* Authentication
* SQLite repository
* Issue synchronization
* Board synchronization
* Error handling

React

* Dashboard
* Issue list
* Issue filtering
* Board selection
* Search
* Settings

Integration

Test:

Login
  ↓
Retrieve user
  ↓
Select board
  ↓
Sync issues
  ↓
Display assigned issues
  ↓
Change status
  ↓
Refresh
  ↓
Verify local cache

⸻

28. Development Phases

Do not attempt to build everything at once.

Phase 1 — Application Shell

Build:

* Tauri application
* React application
* Sidebar
* Dashboard
* Dark/light/system themes
* Basic routing
* SQLite setup

Deliverable:

Working macOS application

with mock Jira data.

⸻

Phase 2 — Jira Authentication

Implement:

* Jira authentication
* Secure credential storage
* Current-user retrieval
* Logout
* Connection status

Deliverable:

Connected to Jira

⸻

Phase 3 — My Tasks

Implement:

* Assigned issues
* Issue cards
* Filtering
* Sorting
* Issue details
* Open in Jira

Deliverable:

My Tasks

working against real Jira data.

⸻

Phase 4 — Boards

Implement:

* Board discovery
* Board selection
* Saved boards
* Board-specific issue views
* Custom JQL

Deliverable:

My Boards

⸻

Phase 5 — Write Operations

Implement:

* Status changes
* Priority changes
* Comments
* Other safe quick actions

Only expose actions supported by the current user’s Jira permissions.

⸻

Phase 6 — Sync & Offline

Implement:

* Background sync
* Manual sync
* SQLite cache
* Offline mode
* Sync status
* Conflict/error handling

⸻

Phase 7 — macOS Integration

Implement:

* Menu-bar mode
* Notifications
* Deep links
* Global shortcuts where appropriate
* Auto-start option

⸻

Phase 8 — Polish

Improve:

* Animations
* Keyboard navigation
* Accessibility
* Empty states
* Error states
* Performance
* macOS visual consistency

⸻

29. Future Features

Do not implement these initially, but structure the architecture so they can be added later.

Potential features:

Personal Work Queue

Today
Tomorrow
This Week
Later

Focus Mode

Show only:

Current task

with:

Start Focus
Pause
Complete

Time Tracking

Optional integration with Jira worklogs.

Quick Issue Creation

⌘ N
Project
Summary
Description
Priority
[Create]

AI Assistant

Potential future commands:

"Show me everything I need to review."
"Which tasks are overdue?"
"Summarize my active tasks."
"What should I work on next?"

AI features should remain optional and should not send Jira data to external AI services without explicit user configuration/consent.

⸻

30. Important UX Principle

Do not make the user navigate through Jira terminology unnecessarily.

Instead of:

Project → Board → Filter → JQL → Sprint → Issue

the application should feel like:

My Work
   ↓
Task
   ↓
Do the work

Jira should primarily act as the backend/source of truth.

The desktop application should act as the user’s personal workspace.

⸻

31. Initial Mock Data

Before connecting to Jira, create mock data representing a realistic developer workload.

Example:

TPT-7042
Update Snowplow tracking
High
In Progress
PRCM-2579
Checkout component update
Medium
Review
TPT-7101
Fix mobile funnel issue
High
To Do
TPT-6981
Update product selector
Medium
In Progress

Use these to build and validate the UI before implementing the API.

⸻

32. Acceptance Criteria

The MVP is complete when a user can:

* Install the macOS application
* Connect their Jira account
* See their Jira identity
* View issues assigned to themselves
* Add Jira boards
* Remove boards
* View issues associated with selected boards
* Filter issues
* Search issues
* Open issue details
* Open an issue in Jira
* Change issue status
* Add comments
* Manually synchronize
* Automatically synchronize
* Use the application while temporarily offline
* Restart the application and retain configuration
* Use dark/light/system appearance
* Use keyboard navigation
* Access the application from the macOS menu bar

⸻

33. Development Instructions for the Coding Agent

Work incrementally.

Do not generate the entire application in one enormous implementation.

Follow this process:

1. Inspect the repository
2. Create/verify project structure
3. Implement Phase 1
4. Run type checking
5. Run tests
6. Run the application
7. Fix issues
8. Commit/record progress
9. Implement next phase

After each major phase:

- Run tests
- Run TypeScript checks
- Run Rust checks
- Run formatter
- Verify the application builds

Prefer small, understandable modules.

Avoid premature abstractions.

Do not introduce unnecessary dependencies.

Document architectural decisions in:

docs/architecture.md

Maintain a development log:

docs/development-log.md

⸻

34. Definition of Done

The project should ultimately produce:

Jira Personal.app

with:

* Native macOS desktop experience
* Jira authentication
* Personal task dashboard
* Saved Jira boards
* Local SQLite cache
* Background synchronization
* Offline support
* Quick issue actions
* Keyboard navigation
* Menu-bar integration
* Dark/light/system appearance
* Secure credential handling
* Automated tests
* Production build configuration

The application should be small, fast, focused, and personal.

The goal is not to replace Jira.

The goal is to create a much better interface for managing my own Jira workload.