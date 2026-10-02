import { useEffect } from "react";
import { Clock3, Database, RefreshCw } from "lucide-react";
import { Header } from "@/components/header";
import { IssueCard } from "@/components/issue-card";
import { useIssueStore } from "@/stores/issue-store";
import { useWorkspaceStore } from "@/stores/workspace-store";

export function TodayView(): React.JSX.Element {
  const issues = useIssueStore((s) => s.issues);
  const counts = useIssueStore((s) => s.counts);
  const loading = useIssueStore((s) => s.loading);
  const refresh = useIssueStore((s) => s.refresh);
  const info = useWorkspaceStore((s) => s.info);
  const fetchInfo = useWorkspaceStore((s) => s.fetchInfo);
  const seedData = useWorkspaceStore((s) => s.seedData);

  useEffect(() => {
    void refresh();
    void fetchInfo();
  }, [refresh, fetchInfo]);

  const handleSeed = async (): Promise<void> => {
    await seedData();
    await refresh();
  };

  const needsAttention = counts.todo + counts.review;

  return (
    <main className="min-w-0 px-8 py-7">
      <Header userName={info.userDisplayName} />

      {/* Sync status bar */}
      <section
        aria-label="Synchronization status"
        className="mt-7 flex items-center justify-between rounded-lg border bg-card px-4 py-3"
      >
        <div className="flex items-center gap-2 text-sm">
          <span className="size-2 rounded-full bg-success" />
          <span className="font-medium">
            {info.issueCount > 0 ? "Local cache ready" : "No issues cached"}
          </span>
          {info.lastSyncedAt ? (
            <span className="text-muted-foreground">
              &middot; Last synced{" "}
              {new Date(info.lastSyncedAt).toLocaleTimeString()}
            </span>
          ) : null}
        </div>
        <div className="flex items-center gap-2">
          {info.issueCount === 0 ? (
            <button
              className="flex items-center gap-2 rounded-md border bg-card px-3 py-1.5 text-sm font-medium text-foreground hover:bg-muted"
              onClick={() => void handleSeed()}
            >
              <Database size={14} />
              Seed Mock Data
            </button>
          ) : null}
          <button className="flex items-center gap-2 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground">
            <RefreshCw size={14} />
            Sync
          </button>
        </div>
      </section>

      {/* Summary cards */}
      <div className="mt-7 grid grid-cols-3 gap-4">
        <div className="rounded-lg border bg-card p-4">
          <p className="text-sm text-muted-foreground">Needs attention</p>
          <p className="mt-2 text-3xl font-semibold">{needsAttention}</p>
        </div>
        <div className="rounded-lg border bg-card p-4">
          <p className="text-sm text-muted-foreground">In progress</p>
          <p className="mt-2 text-3xl font-semibold">{counts.inProgress}</p>
        </div>
        <div className="rounded-lg border bg-card p-4">
          <p className="text-sm text-muted-foreground">Total</p>
          <p className="mt-2 text-3xl font-semibold">{counts.total}</p>
        </div>
      </div>

      {/* Issue list */}
      <section className="mt-8" aria-labelledby="active-work">
        <div className="mb-4 flex items-center justify-between">
          <div>
            <h2 id="active-work" className="text-lg font-semibold">
              Active work
            </h2>
            <p className="text-sm text-muted-foreground">
              {info.connectionStatus === "connected"
                ? "Your personal Jira queue from the local cache."
                : "Seed mock data or connect Jira in M3."}
            </p>
          </div>
          <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
            <Clock3 size={14} />
            {counts.total} issues
          </span>
        </div>
        {loading ? (
          <p className="py-12 text-center text-muted-foreground">Loading...</p>
        ) : issues.length === 0 ? (
          <div className="rounded-lg border bg-card p-12 text-center">
            <p className="text-muted-foreground">
              No issues yet. Seed mock data to preview the workspace.
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-4">
            {issues.map((issue) => (
              <IssueCard issue={issue} key={issue.id} />
            ))}
          </div>
        )}
      </section>
    </main>
  );
}
