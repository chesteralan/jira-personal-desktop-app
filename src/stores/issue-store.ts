import { create } from "zustand";
import {
  listIssues,
  getIssueCounts,
  type IssueView,
  type IssueCounts,
  type IssueFilter,
} from "@/services/ipc";

interface IssueState {
  issues: IssueView[];
  counts: IssueCounts;
  filter: IssueFilter;
  loading: boolean;
  error: string | null;
  setFilter: (filter: IssueFilter) => void;
  fetchIssues: () => Promise<void>;
  fetchCounts: () => Promise<void>;
  refresh: () => Promise<void>;
}

const emptyCounts: IssueCounts = {
  total: 0,
  todo: 0,
  inProgress: 0,
  review: 0,
  done: 0,
};

export const useIssueStore = create<IssueState>((set, get) => ({
  issues: [],
  counts: emptyCounts,
  filter: {},
  loading: false,
  error: null,

  setFilter: (filter: IssueFilter) => {
    set({ filter });
    void get().fetchIssues();
  },

  fetchIssues: async () => {
    set({ loading: true, error: null });
    try {
      const issues = await listIssues(get().filter);
      set({ issues, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  fetchCounts: async () => {
    try {
      const counts = await getIssueCounts();
      set({ counts });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  refresh: async () => {
    await Promise.all([get().fetchIssues(), get().fetchCounts()]);
  },
}));
