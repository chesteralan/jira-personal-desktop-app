import { useEffect, useMemo, useState } from "react";
import { Filter, Search, X } from "lucide-react";
import { Header } from "@/components/header";
import { IssueCard } from "@/components/issue-card";
import { useIssueStore } from "@/stores/issue-store";
import { useWorkspaceStore } from "@/stores/workspace-store";
import type { IssueFilter, IssueStatus, IssuePriority } from "@/services/ipc";
import { cn } from "@/lib/utils";

const statusTabs: Array<{ label: string; value: IssueStatus | "all" }> = [
  { label: "All", value: "all" },
  { label: "To Do", value: "todo" },
  { label: "In Progress", value: "in_progress" },
  { label: "Review", value: "review" },
  { label: "Done", value: "done" },
];

const priorityOptions: Array<{ label: string; value: IssuePriority }> = [
  { label: "Highest", value: "highest" },
  { label: "High", value: "high" },
  { label: "Medium", value: "medium" },
  { label: "Low", value: "low" },
  { label: "Lowest", value: "lowest" },
];

/** Create a new filter with a specific key removed. */
function omitKey<K extends keyof IssueFilter>(
  f: IssueFilter,
  key: K,
): IssueFilter {
  return Object.fromEntries(
    Object.entries(f).filter(([k]) => k !== key),
  ) as unknown as IssueFilter;
}

export function TasksView(): React.JSX.Element {
  const issues = useIssueStore((s) => s.issues);
  const filter = useIssueStore((s) => s.filter);
  const setFilter = useIssueStore((s) => s.setFilter);
  const refresh = useIssueStore((s) => s.refresh);
  const info = useWorkspaceStore((s) => s.info);

  const [showFilters, setShowFilters] = useState(false);
  const [searchQuery, setSearchQuery] = useState(filter.search ?? "");

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const activeTab =
    filter.status && filter.status.length === 1 ? filter.status[0] : "all";

  // Extract unique projects, sprints, and labels from current issues for filter dropdowns
  const { projects, sprints, labels } = useMemo(() => {
    const ps = new Set<string>();
    const ss = new Set<string>();
    const ls = new Set<string>();
    for (const issue of issues) {
      ps.add(issue.projectKey);
      if (issue.sprint) ss.add(issue.sprint);
      for (const l of issue.labels) ls.add(l);
    }
    return {
      projects: [...ps].sort(),
      sprints: [...ss].sort(),
      labels: [...ls].sort(),
    };
  }, [issues]);

  const activeFilterCount = [
    filter.priority,
    filter.projectKey,
    filter.sprint,
    filter.label,
    filter.search,
  ].filter(Boolean).length;

  const updateFilter = (partial: Partial<IssueFilter>): void => {
    setFilter({ ...filter, ...partial });
  };

  const clearAllFilters = (): void => {
    setSearchQuery("");
    setFilter({});
  };

  const handleSearch = (): void => {
    if (searchQuery.trim()) {
      updateFilter({ search: searchQuery.trim() });
    } else {
      setFilter(omitKey(filter, "search"));
    }
  };

  return (
    <main className="min-w-0 px-8 py-7">
      <Header userName={info.userDisplayName} />

      {/* Status tabs */}
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
                setFilter(omitKey(filter, "status"));
              } else {
                setFilter({ ...filter, status: [tab.value] });
              }
            }}
          >
            {tab.label}
          </button>
        ))}

        <div className="ml-auto flex items-center gap-2">
          {/* Search */}
          <div className="flex items-center gap-1 rounded-md border bg-background px-2">
            <Search size={14} className="text-muted-foreground" />
            <input
              className="w-32 border-0 bg-transparent py-1 text-sm focus:outline-none"
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSearch()}
              placeholder="Search..."
              value={searchQuery}
            />
            {searchQuery ? (
              <button
                className="text-muted-foreground hover:text-foreground"
                onClick={() => {
                  setSearchQuery("");
                  setFilter(omitKey(filter, "search"));
                }}
              >
                <X size={12} />
              </button>
            ) : null}
          </div>

          {/* Filter toggle */}
          <button
            className={cn(
              "flex items-center gap-1.5 rounded-md px-2.5 py-1.5 text-sm",
              showFilters || activeFilterCount > 0
                ? "bg-primary/10 text-primary"
                : "text-muted-foreground hover:text-foreground",
            )}
            onClick={() => setShowFilters((v) => !v)}
          >
            <Filter size={14} />
            Filters
            {activeFilterCount > 0 ? (
              <span className="ml-0.5 rounded-full bg-primary px-1.5 py-0.5 text-[10px] font-bold text-primary-foreground">
                {activeFilterCount}
              </span>
            ) : null}
          </button>
        </div>
      </div>

      {/* Advanced filters panel */}
      {showFilters ? (
        <div className="mt-3 rounded-lg border bg-card p-4">
          <div className="flex items-center justify-between">
            <p className="text-sm font-semibold">Filters</p>
            {activeFilterCount > 0 ? (
              <button
                className="text-xs text-muted-foreground hover:text-foreground"
                onClick={clearAllFilters}
              >
                Clear all
              </button>
            ) : null}
          </div>
          <div className="mt-3 flex flex-wrap gap-4">
            {/* Priority */}
            <div>
              <label className="mb-1 block text-xs font-medium text-muted-foreground">
                Priority
              </label>
              <div className="flex flex-wrap gap-1">
                {priorityOptions.map((p) => {
                  const active = filter.priority?.includes(p.value);
                  return (
                    <button
                      className={cn(
                        "rounded-md border px-2 py-1 text-xs transition",
                        active
                          ? "border-primary bg-primary/10 text-primary"
                          : "hover:border-primary/40",
                      )}
                      key={p.value}
                      onClick={() => {
                        if (active) {
                          const next =
                            filter.priority?.filter((v) => v !== p.value) ?? [];
                          if (next.length === 0) {
                            setFilter(omitKey(filter, "priority"));
                          } else {
                            updateFilter({ priority: next });
                          }
                        } else {
                          updateFilter({
                            priority: [...(filter.priority ?? []), p.value],
                          });
                        }
                      }}
                    >
                      {p.label}
                    </button>
                  );
                })}
              </div>
            </div>

            {/* Project */}
            {projects.length > 1 ? (
              <div>
                <label className="mb-1 block text-xs font-medium text-muted-foreground">
                  Project
                </label>
                <select
                  className="rounded-md border bg-background px-2 py-1 text-xs"
                  onChange={(e) => {
                    if (e.target.value === "") {
                      setFilter(omitKey(filter, "projectKey"));
                    } else {
                      updateFilter({ projectKey: e.target.value });
                    }
                  }}
                  value={filter.projectKey ?? ""}
                >
                  <option value="">All</option>
                  {projects.map((p) => (
                    <option key={p} value={p}>
                      {p}
                    </option>
                  ))}
                </select>
              </div>
            ) : null}

            {/* Sprint */}
            {sprints.length > 0 ? (
              <div>
                <label className="mb-1 block text-xs font-medium text-muted-foreground">
                  Sprint
                </label>
                <select
                  className="rounded-md border bg-background px-2 py-1 text-xs"
                  onChange={(e) => {
                    if (e.target.value === "") {
                      setFilter(omitKey(filter, "sprint"));
                    } else {
                      updateFilter({ sprint: e.target.value });
                    }
                  }}
                  value={filter.sprint ?? ""}
                >
                  <option value="">All</option>
                  {sprints.map((s) => (
                    <option key={s} value={s}>
                      {s}
                    </option>
                  ))}
                </select>
              </div>
            ) : null}

            {/* Label */}
            {labels.length > 0 ? (
              <div>
                <label className="mb-1 block text-xs font-medium text-muted-foreground">
                  Label
                </label>
                <select
                  className="rounded-md border bg-background px-2 py-1 text-xs"
                  onChange={(e) => {
                    if (e.target.value === "") {
                      setFilter(omitKey(filter, "label"));
                    } else {
                      updateFilter({ label: e.target.value });
                    }
                  }}
                  value={filter.label ?? ""}
                >
                  <option value="">All</option>
                  {labels.map((l) => (
                    <option key={l} value={l}>
                      {l}
                    </option>
                  ))}
                </select>
              </div>
            ) : null}
          </div>
        </div>
      ) : null}

      {/* Issue list */}
      <section className="mt-6">
        {issues.length === 0 ? (
          <div className="rounded-lg border bg-card p-12 text-center">
            <p className="text-muted-foreground">
              No issues match the current filter.
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
