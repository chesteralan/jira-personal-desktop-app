import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

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

export async function getAppVersion(): Promise<string> {
  return invoke<string>("get_app_version");
}

// ── Auth commands ─────────────────────────────────────────────────────

export interface ConnectInput {
  baseUrl: string;
  email: string;
  apiToken: string;
}

export interface ConnectResult {
  displayName: string;
  email: string | null;
  issueCount: number;
}

export async function jiraConnect(input: ConnectInput): Promise<ConnectResult> {
  return invoke<ConnectResult>("jira_connect", { input });
}

export async function jiraDisconnect(): Promise<void> {
  return invoke<void>("jira_disconnect");
}

export async function jiraSync(): Promise<number> {
  return invoke<number>("jira_sync");
}

export async function jiraRestoreSession(): Promise<boolean> {
  return invoke<boolean>("jira_restore_session");
}

// ── Sync events ─────────────────────────────────────────────────────

export type SyncState = "idle" | "syncing" | "success" | "error" | "offline";

export interface SyncStatus {
  state: SyncState;
  lastSyncedAt: string | null;
  issueCount: number | null;
  error: string | null;
}

export async function onSyncStatus(
  handler: (status: SyncStatus) => void,
): Promise<UnlistenFn> {
  return listen<SyncStatus>("sync-status", (event) => {
    handler(event.payload);
  });
}

// ── Board types ─────────────────────────────────────────────────────

export interface Board {
  id: number;
  name: string;
  boardType: string;
  projectKey: string | null;
}

export interface SavedBoard {
  boardId: number;
  name: string;
  boardType: string;
  projectKey: string | null;
}

export interface IssueTransition {
  id: string;
  name: string;
}

// ── Board commands ──────────────────────────────────────────────────

export async function listBoards(): Promise<Board[]> {
  return invoke<Board[]>("list_boards");
}

export async function listSavedBoards(): Promise<SavedBoard[]> {
  return invoke<SavedBoard[]>("list_saved_boards");
}

export async function saveBoard(board: SavedBoard): Promise<void> {
  return invoke<void>("save_board", { board });
}

export async function unsaveBoard(boardId: number): Promise<boolean> {
  return invoke<boolean>("unsave_board", { boardId });
}

export async function getBoardIssues(boardId: number): Promise<IssueView[]> {
  return invoke<IssueView[]>("get_board_issues", { boardId });
}

// ── Quick action commands ───────────────────────────────────────────

export async function getTransitions(
  issueKey: string,
): Promise<IssueTransition[]> {
  return invoke<IssueTransition[]>("get_transitions", { issueKey });
}

export async function transitionIssue(
  issueKey: string,
  transitionId: string,
): Promise<void> {
  return invoke<void>("transition_issue", { issueKey, transitionId });
}

export async function addComment(
  issueKey: string,
  body: string,
): Promise<void> {
  return invoke<void>("add_comment", { issueKey, body });
}

// ── OAuth commands ──────────────────────────────────────────────────

export interface OAuthSetupInput {
  clientId: string;
  clientSecret: string;
}

export interface OAuthConnectResult {
  displayName: string;
  email: string | null;
  issueCount: number;
  siteUrl: string;
}

export async function oauthStart(
  input: OAuthSetupInput,
): Promise<OAuthConnectResult> {
  return invoke<OAuthConnectResult>("oauth_start", { input });
}

export async function oauthDisconnect(): Promise<void> {
  return invoke<void>("oauth_disconnect");
}

export async function oauthRefresh(): Promise<void> {
  return invoke<void>("oauth_refresh");
}
