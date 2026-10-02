import { invoke } from "@tauri-apps/api/core";

// ── Domain types matching Rust serde output ─────────────────────────────

export type IssueStatus =
  | "todo"
  | "in_progress"
  | "review"
  | "done"
  | "unknown";

export type IssuePriority =
  | "highest"
  | "high"
  | "medium"
  | "low"
  | "lowest"
  | "unknown";

export type ConnectionStatus =
  | "connected"
  | "disconnected"
  | "syncing"
  | "error";

export interface IssueView {
  id: string;
  key: string;
  summary: string;
  status: IssueStatus;
  priority: IssuePriority;
  projectKey: string;
  projectName: string;
  assignee: string | null;
  labels: string[];
  sprint: string | null;
  dueDate: string | null;
  updatedAt: string;
  webUrl: string | null;
}

export interface IssueFilter {
  status?: IssueStatus[] | undefined;
  priority?: IssuePriority[] | undefined;
  projectKey?: string | undefined;
  sprint?: string | undefined;
  label?: string | undefined;
  search?: string | undefined;
}

export interface IssueCounts {
  total: number;
  todo: number;
  inProgress: number;
  review: number;
  done: number;
}

export interface Preferences {
  theme: "light" | "dark" | "system";
  sidebarCollapsed: boolean;
  syncIntervalSecs: number;
  activeView: string;
}

export interface WorkspaceInfo {
  connectionStatus: ConnectionStatus;
  lastSyncedAt: string | null;
  issueCount: number;
  userDisplayName: string | null;
  jiraBaseUrl: string | null;
}

// ── IPC commands ────────────────────────────────────────────────────────

export async function listIssues(filter?: IssueFilter): Promise<IssueView[]> {
  return invoke<IssueView[]>("list_issues", { filter: filter ?? null });
}

export async function getIssue(key: string): Promise<IssueView | null> {
  return invoke<IssueView | null>("get_issue", { key });
}

export async function getIssueCounts(): Promise<IssueCounts> {
  return invoke<IssueCounts>("get_issue_counts");
}

export async function getPreferences(): Promise<Preferences> {
  return invoke<Preferences>("get_preferences");
}

export async function savePreferences(prefs: Preferences): Promise<void> {
  return invoke<void>("save_preferences", { prefs });
}

export async function getWorkspaceInfo(): Promise<WorkspaceInfo> {
  return invoke<WorkspaceInfo>("get_workspace_info");
}

export async function seedMockData(): Promise<string> {
  return invoke<string>("seed_mock_data");
}
