import { useEffect } from "react";
import { useUiStore } from "@/stores/ui-store";
import { useWorkspaceStore } from "@/stores/workspace-store";
import { useIssueStore } from "@/stores/issue-store";

/**
 * Register in-app keyboard shortcuts:
 *   Cmd/Ctrl+B   — toggle sidebar
 *   Cmd/Ctrl+1   — Today view
 *   Cmd/Ctrl+2   — My Tasks view
 *   Cmd/Ctrl+3   — Boards view
 *   Cmd/Ctrl+,   — Settings view
 *   Cmd/Ctrl+R   — Manual sync
 */
export function useKeyboardShortcuts(): void {
  const toggleSidebar = useUiStore((s) => s.toggleSidebar);
  const setActiveView = useUiStore((s) => s.setActiveView);
  const connectionStatus = useWorkspaceStore((s) => s.info.connectionStatus);
  const sync = useWorkspaceStore((s) => s.sync);
  const refresh = useIssueStore((s) => s.refresh);

  useEffect(() => {
    const handler = (e: KeyboardEvent): void => {
      const mod = e.metaKey || e.ctrlKey;
      if (!mod) return;

      switch (e.key) {
        case "b":
          e.preventDefault();
          toggleSidebar();
          break;
        case "1":
          e.preventDefault();
          setActiveView("today");
          break;
        case "2":
          e.preventDefault();
          setActiveView("tasks");
          break;
        case "3":
          e.preventDefault();
          setActiveView("boards");
          break;
        case ",":
          e.preventDefault();
          setActiveView("settings");
          break;
        case "r":
          if (!e.shiftKey && connectionStatus === "connected") {
            e.preventDefault();
            void sync().then(() => refresh());
          }
          break;
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [toggleSidebar, setActiveView, connectionStatus, sync, refresh]);
}
