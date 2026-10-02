import { create } from "zustand";
import {
  getWorkspaceInfo,
  jiraConnect,
  jiraDisconnect,
  jiraSync,
  jiraRestoreSession,
  oauthStart,
  oauthDisconnect,
  onSyncStatus,
  seedMockData,
  type ConnectInput,
  type ConnectResult,
  type OAuthSetupInput,
  type OAuthConnectResult,
  type SyncState,
  type SyncStatus,
  type WorkspaceInfo,
} from "@/services/ipc";

interface WorkspaceState {
  info: WorkspaceInfo;
  loading: boolean;
  syncing: boolean;
  syncState: SyncState;
  error: string | null;
  fetchInfo: () => Promise<void>;
  connect: (input: ConnectInput) => Promise<ConnectResult>;
  oauthConnect: (input: OAuthSetupInput) => Promise<OAuthConnectResult>;
  disconnect: () => Promise<void>;
  sync: () => Promise<number>;
  restoreSession: () => Promise<boolean>;
  seedData: () => Promise<string>;
  handleSyncEvent: (status: SyncStatus) => void;
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
  syncState: "idle" as SyncState,
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

  oauthConnect: async (input: OAuthSetupInput) => {
    set({ loading: true, error: null });
    try {
      const result = await oauthStart(input);
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
      // Also clear OAuth if that was the method.
      try {
        await oauthDisconnect();
      } catch {
        // Ignore — may not have been OAuth.
      }
      set({ info: defaultInfo, loading: false, syncState: "idle" });
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

  handleSyncEvent: (status: SyncStatus) => {
    set((prev) => {
      const updates: Partial<WorkspaceState> = {
        syncState: status.state,
        syncing: status.state === "syncing",
      };

      if (status.state === "success") {
        updates.info = {
          ...prev.info,
          lastSyncedAt: status.lastSyncedAt ?? prev.info.lastSyncedAt,
          issueCount: status.issueCount ?? prev.info.issueCount,
        };
        updates.error = null;
      } else if (status.state === "error") {
        updates.error = status.error ?? "Sync failed";
      } else if (status.state === "offline") {
        updates.error = status.error ?? "Network unavailable";
      }

      return updates as WorkspaceState;
    });
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

/// Initialize the sync event listener. Call once from the app root.
export async function initSyncListener(): Promise<() => void> {
  const { handleSyncEvent } = useWorkspaceStore.getState();
  const unlisten = await onSyncStatus(handleSyncEvent);
  return unlisten;
}
