import { useEffect } from "react";
import { Sidebar } from "@/components/sidebar";
import { TodayView } from "@/features/today/today-view";
import { TasksView } from "@/features/tasks/tasks-view";
import { BoardsView } from "@/features/boards/boards-view";
import { SettingsView } from "@/features/settings/settings-view";
import { useUiStore, type AppView } from "@/stores/ui-store";
import { useWorkspaceStore, initSyncListener } from "@/stores/workspace-store";
import { useIssueStore } from "@/stores/issue-store";
import { useKeyboardShortcuts } from "@/hooks/use-keyboard-shortcuts";

function ViewRouter({ view }: { view: AppView }): React.JSX.Element {
  switch (view) {
    case "today":
      return <TodayView />;
    case "tasks":
    case "review":
    case "search":
      return <TasksView />;
    case "boards":
      return <BoardsView />;
    case "settings":
      return <SettingsView />;
  }
}

export function App(): React.JSX.Element {
  const activeView = useUiStore((s) => s.activeView);
  const fetchInfo = useWorkspaceStore((s) => s.fetchInfo);
  const restoreSession = useWorkspaceStore((s) => s.restoreSession);
  const sync = useWorkspaceStore((s) => s.sync);
  const loading = useWorkspaceStore((s) => s.loading);
  const refresh = useIssueStore((s) => s.refresh);
  const info = useWorkspaceStore((s) => s.info);

  // Register in-app keyboard shortcuts
  useKeyboardShortcuts();

  // Restore session, init sync listener, and load workspace info on mount
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const init = async (): Promise<void> => {
      unlisten = await initSyncListener();
      await restoreSession();
      await fetchInfo();
    };

    void init();
    return () => unlisten?.();
  }, [restoreSession, fetchInfo]);

  // Auto-sync on window focus when connected (skip if a connection is in progress)
  useEffect(() => {
    const handler = (): void => {
      if (info.connectionStatus === "connected" && !loading) {
        void sync().then(() => refresh());
      }
    };
    window.addEventListener("focus", handler);
    return () => window.removeEventListener("focus", handler);
  }, [info.connectionStatus, loading, sync, refresh]);

  // Refresh issue list when sync status changes to success
  const syncState = useWorkspaceStore((s) => s.syncState);
  useEffect(() => {
    if (syncState === "success") {
      void refresh();
      void fetchInfo();
    }
  }, [syncState, refresh, fetchInfo]);

  return (
    <div className="grid min-h-screen grid-cols-[auto_1fr] bg-background text-foreground">
      <Sidebar />
      <ViewRouter view={activeView} />
    </div>
  );
}
