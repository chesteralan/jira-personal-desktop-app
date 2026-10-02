import { useEffect, useMemo } from "react";
import { Header } from "@/components/header";
import { IssueCard } from "@/components/issue-card";
import { useIssueStore } from "@/stores/issue-store";
import { useWorkspaceStore } from "@/stores/workspace-store";
import type { IssueStatus } from "@/services/ipc";
import { cn } from "@/lib/utils";

const statusTabs: Array<{ label: string; value: IssueStatus | "all" }> = [
  { label: "All", value: "all" },
  { label: "To Do", value: "todo" },
  { label: "In Progress", value: "in_progress" },
  { label: "Review", value: "review" },
  { label: "Done", value: "done" },
];

export function TasksView(): React.JSX.Element {
  const issues = useIssueStore((s) => s.issues);
  const filter = useIssueStore((s) => s.filter);
  const setFilter = useIssueStore((s) => s.setFilter);
  const refresh = useIssueStore((s) => s.refresh);
  const info = useWorkspaceStore((s) => s.info);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const activeTab =
    filter.status && filter.status.length === 1 ? filter.status[0] : "all";

  const filteredIssues = useMemo(() => issues, [issues]);

  return (
    <main className="min-w-0 px-8 py-7">
      <Header userName={info.userDisplayName} />

      <div className="mt-7 flex items-center gap-1 rounded-lg border bg-card p-1">
        {statusTabs.map((tab) => (
          <button
            className={cn(
              "rounded-md px-3 py-1.5 text-sm font-medium transition-colors",
              activeTab === tab.value
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted hover:text-foreground",
            )}
            key={tab.value}
            onClick={() => {
              if (tab.value === "all") {
                setFilter({
                  priority: filter.priority,
                  projectKey: filter.projectKey,
                  sprint: filter.sprint,
                  label: filter.label,
                  search: filter.search,
                });
              } else {
                setFilter({ ...filter, status: [tab.value] });
              }
            }}
          >
            {tab.label}
          </button>
        ))}
      </div>

      <section className="mt-6">
        {filteredIssues.length === 0 ? (
          <div className="rounded-lg border bg-card p-12 text-center">
            <p className="text-muted-foreground">
              No issues match the current filter.
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-4">
            {filteredIssues.map((issue) => (
              <IssueCard issue={issue} key={issue.id} />
            ))}
          </div>
        )}
      </section>
    </main>
  );
}
