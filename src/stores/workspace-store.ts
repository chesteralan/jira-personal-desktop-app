import { create } from "zustand";
import {
  getWorkspaceInfo,
  jiraConnect,
  jiraDisconnect,
  jiraSync,
  jiraRestoreSession,
  seedMockData,
  type ConnectInput,
  type ConnectResult,
  type WorkspaceInfo,
} from "@/services/ipc";

interface WorkspaceState {
  info: WorkspaceInfo;
  loading: boolean;
  syncing: boolean;
  error: string | null;
  fetchInfo: () => Promise<void>;
  connect: (input: ConnectInput) => Promise<ConnectResult>;
  disconnect: () => Promise<void>;
  sync: () => Promise<number>;
  restoreSession: () => Promise<boolean>;
  seedData: () => Promise<string>;
  clearError: () => void;
}

const defaultInfo: WorkspaceInfo = {
  connectionStatus: "disconnected",
  lastSyncedAt: null,
  issueCount: 0,
  userDisplayName: null,
  jiraBaseUrl: null,
};

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  info: defaultInfo,
  loading: false,
  syncing: false,
  error: null,

  fetchInfo: async () => {
    set({ loading: true, error: null });
    try {
      const info = await getWorkspaceInfo();
      set({ info, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  connect: async (input: ConnectInput) => {
    set({ loading: true, error: null });
    try {
      const result = await jiraConnect(input);
      const info = await getWorkspaceInfo();
      set({ info, loading: false });
      return result;
    } catch (e) {
      set({ error: String(e), loading: false });
      throw e;
    }
  },

  disconnect: async () => {
    set({ loading: true, error: null });
    try {
      await jiraDisconnect();
      set({ info: defaultInfo, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  sync: async () => {
    set({ syncing: true, error: null });
    try {
      const count = await jiraSync();
      const info = await getWorkspaceInfo();
      set({ info, syncing: false });
      return count;
    } catch (e) {
      set({ error: String(e), syncing: false });
      throw e;
    }
  },

  restoreSession: async () => {
    try {
      const restored = await jiraRestoreSession();
      if (restored) {
        const info = await getWorkspaceInfo();
        set({ info });
      }
      return restored;
    } catch {
      return false;
    }
  },

  seedData: async () => {
    try {
      const result = await seedMockData();
      const info = await getWorkspaceInfo();
      set({ info });
      return result;
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },

  clearError: () => set({ error: null }),
}));
