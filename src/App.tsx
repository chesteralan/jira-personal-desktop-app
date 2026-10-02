import { useEffect } from "react";
import { Sidebar } from "@/components/sidebar";
import { TodayView } from "@/features/today/today-view";
import { TasksView } from "@/features/tasks/tasks-view";
import { BoardsView } from "@/features/boards/boards-view";
import { SettingsView } from "@/features/settings/settings-view";
import { useUiStore, type AppView } from "@/stores/ui-store";
import { useWorkspaceStore, initSyncListener } from "@/stores/workspace-store";
import { useIssueStore } from "@/stores/issue-store";

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
  const toggleSidebar = useUiStore((s) => s.toggleSidebar);
  const fetchInfo = useWorkspaceStore((s) => s.fetchInfo);
  const restoreSession = useWorkspaceStore((s) => s.restoreSession);
  const sync = useWorkspaceStore((s) => s.sync);
  const refresh = useIssueStore((s) => s.refresh);
  const info = useWorkspaceStore((s) => s.info);

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

  // Auto-sync on window focus when connected
  useEffect(() => {
    const handler = (): void => {
      if (info.connectionStatus === "connected") {
        void sync().then(() => refresh());
      }
    };
    window.addEventListener("focus", handler);
    return () => window.removeEventListener("focus", handler);
  }, [info.connectionStatus, sync, refresh]);

  // Refresh issue list when sync status changes to success
  const syncState = useWorkspaceStore((s) => s.syncState);
  useEffect(() => {
    if (syncState === "success") {
      void refresh();
      void fetchInfo();
    }
  }, [syncState, refresh, fetchInfo]);

  // Keyboard shortcut: Cmd+B to toggle sidebar
  useEffect(() => {
    const handler = (e: KeyboardEvent): void => {
      if ((e.metaKey || e.ctrlKey) && e.key === "b") {
        e.preventDefault();
        toggleSidebar();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [toggleSidebar]);

  return (
    <div className="grid min-h-screen grid-cols-[auto_1fr] bg-background text-foreground">
      <Sidebar />
      <ViewRouter view={activeView} />
    </div>
  );
}
