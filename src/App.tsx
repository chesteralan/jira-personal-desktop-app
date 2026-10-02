import { useEffect } from "react";
import { Sidebar } from "@/components/sidebar";
import { TodayView } from "@/features/today/today-view";
import { TasksView } from "@/features/tasks/tasks-view";
import { SettingsView } from "@/features/settings/settings-view";
import { useUiStore, type AppView } from "@/stores/ui-store";
import { useWorkspaceStore } from "@/stores/workspace-store";

function ViewRouter({ view }: { view: AppView }): React.JSX.Element {
  switch (view) {
    case "today":
      return <TodayView />;
    case "tasks":
    case "review":
    case "search":
      return <TasksView />;
    case "settings":
      return <SettingsView />;
  }
}

export function App(): React.JSX.Element {
  const activeView = useUiStore((s) => s.activeView);
  const toggleSidebar = useUiStore((s) => s.toggleSidebar);
  const fetchInfo = useWorkspaceStore((s) => s.fetchInfo);

  // Load workspace info on mount
  useEffect(() => {
    void fetchInfo();
  }, [fetchInfo]);

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
