import { create } from "zustand";

export type AppView =
  | "today"
  | "tasks"
  | "review"
  | "boards"
  | "search"
  | "settings";

interface UiState {
  sidebarCollapsed: boolean;
  activeView: AppView;
  toggleSidebar: () => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
  setActiveView: (view: AppView) => void;
}

export const useUiStore = create<UiState>((set) => ({
  sidebarCollapsed: false,
  activeView: "today",

  toggleSidebar: () => {
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
  },

  setSidebarCollapsed: (collapsed: boolean) => {
    set({ sidebarCollapsed: collapsed });
  },

  setActiveView: (view: AppView) => {
    set({ activeView: view });
  },
}));
