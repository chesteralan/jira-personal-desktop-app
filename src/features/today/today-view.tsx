import { useEffect } from "react";
import { Clock3, Database, Loader2, RefreshCw, WifiOff } from "lucide-react";
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
  const syncing = useWorkspaceStore((s) => s.syncing);
  const syncState = useWorkspaceStore((s) => s.syncState);
  const error = useWorkspaceStore((s) => s.error);
  const fetchInfo = useWorkspaceStore((s) => s.fetchInfo);
  const sync = useWorkspaceStore((s) => s.sync);
  const seedData = useWorkspaceStore((s) => s.seedData);
  const clearError = useWorkspaceStore((s) => s.clearError);

  useEffect(() => {
    void refresh();
    void fetchInfo();
  }, [refresh, fetchInfo]);

  const handleSeed = async (): Promise<void> => {
    await seedData();
    await refresh();
  };

  const handleSync = async (): Promise<void> => {
    try {
      await sync();
      await refresh();
    } catch {
      // Error displayed via store
    }
  };

  const isConnected = info.connectionStatus === "connected";
  const isOffline = syncState === "offline";
  const needsAttention = counts.todo + counts.review;

  return (
    <main className="min-w-0 px-8 py-7">
      <Header userName={info.userDisplayName} />

      {/* Offline banner */}
      {isOffline ? (
        <div className="mt-4 flex items-center gap-2 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-4 py-3 text-sm text-yellow-700 dark:text-yellow-400">
          <WifiOff size={16} />
          <span>
            Offline — showing cached data. Sync will resume when connected.
          </span>
        </div>
      ) : null}

      {/* Error banner */}
      {error && !isOffline ? (
        <div className="mt-4 flex items-center justify-between rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          <span>{error}</span>
          <button className="text-xs underline" onClick={clearError}>
            Dismiss
          </button>
        </div>
      ) : null}

      {/* Sync status bar */}
      <section
        aria-label="Synchronization status"
        className="mt-4 flex items-center justify-between rounded-lg border bg-card px-4 py-3"
      >
        <div className="flex items-center gap-2 text-sm">
          <span
            className={`size-2 rounded-full ${
              isOffline
                ? "bg-yellow-500"
                : isConnected
                  ? "bg-success"
                  : "bg-muted-foreground"
            }`}
          />
          <span className="font-medium">
            {isOffline
              ? "Offline"
              : isConnected
                ? "Connected to Jira"
                : info.issueCount > 0
                  ? "Local cache ready"
                  : "Not connected"}
          </span>
          {info.lastSyncedAt ? (
            <span className="text-muted-foreground">
              &middot; Last synced{" "}
              {new Date(info.lastSyncedAt).toLocaleTimeString()}
            </span>
          ) : null}
        </div>
        <div className="flex items-center gap-2">
          {!isConnected && info.issueCount === 0 ? (
            <button
              className="flex items-center gap-2 rounded-md border bg-card px-3 py-1.5 text-sm font-medium text-foreground hover:bg-muted"
              onClick={() => void handleSeed()}
            >
              <Database size={14} />
              Seed Mock Data
            </button>
          ) : null}
          <button
            className="flex items-center gap-2 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground disabled:opacity-50"
            disabled={!isConnected || syncing}
            onClick={() => void handleSync()}
          >
            {syncing ? (
              <Loader2 className="animate-spin" size={14} />
            ) : (
              <RefreshCw size={14} />
            )}
            {syncing ? "Syncing..." : "Sync"}
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
              {isConnected
                ? "Your assigned Jira issues."
                : "Connect to Jira in Settings or seed mock data."}
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
              No issues yet. Connect to Jira or seed mock data to get started.
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-4">
            {issues.map((issue) => (
              <IssueCard
                issue={issue}
                key={issue.id}
                onActionComplete={() => void refresh()}
              />
            ))}
          </div>
        )}
      </section>
    </main>
  );
}
