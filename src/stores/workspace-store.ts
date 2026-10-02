import { create } from "zustand";
import {
  getWorkspaceInfo,
  seedMockData,
  type WorkspaceInfo,
} from "@/services/ipc";

interface WorkspaceState {
  info: WorkspaceInfo;
  loading: boolean;
  error: string | null;
  fetchInfo: () => Promise<void>;
  seedData: () => Promise<string>;
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

  seedData: async () => {
    try {
      const result = await seedMockData();
      // Re-fetch workspace info after seeding
      const info = await getWorkspaceInfo();
      set({ info });
      return result;
    } catch (e) {
      set({ error: String(e) });
      throw e;
    }
  },
}));
