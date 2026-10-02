import {
  AlertCircle,
  ArrowUp,
  ArrowDown,
  Minus,
  ExternalLink,
} from "lucide-react";
import { cn } from "@/lib/utils";
import type { IssueView, IssuePriority, IssueStatus } from "@/services/ipc";

const statusLabels: Record<IssueStatus, string> = {
  todo: "To Do",
  in_progress: "In Progress",
  review: "Review",
  done: "Done",
  unknown: "Unknown",
};

const statusColors: Record<IssueStatus, string> = {
  todo: "bg-muted text-muted-foreground",
  in_progress: "bg-primary/15 text-primary",
  review: "bg-warning/15 text-warning",
  done: "bg-success/15 text-success",
  unknown: "bg-muted text-muted-foreground",
};

function PriorityIcon({
  priority,
}: {
  priority: IssuePriority;
}): React.JSX.Element {
  switch (priority) {
    case "highest":
      return <AlertCircle size={14} className="text-destructive" />;
    case "high":
      return <ArrowUp size={14} className="text-destructive/80" />;
    case "medium":
      return <Minus size={14} className="text-warning" />;
    case "low":
      return <ArrowDown size={14} className="text-success" />;
    case "lowest":
      return <ArrowDown size={14} className="text-muted-foreground" />;
    default:
      return <Minus size={14} className="text-muted-foreground" />;
  }
}

function formatUpdated(isoDate: string): string {
  const diff = Date.now() - new Date(isoDate).getTime();
  const minutes = Math.floor(diff / 60_000);
  if (minutes < 1) return "Just now";
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

function formatDueDate(dateStr: string): string | null {
  const due = new Date(dateStr + "T00:00:00");
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const diff = Math.floor(
    (due.getTime() - today.getTime()) / (1000 * 60 * 60 * 24),
  );
  if (diff < 0) return `${Math.abs(diff)}d overdue`;
  if (diff === 0) return "Today";
  if (diff === 1) return "Tomorrow";
  if (diff <= 7) return `${diff}d`;
  return null;
}

interface IssueCardProps {
  issue: IssueView;
}

export function IssueCard({ issue }: IssueCardProps): React.JSX.Element {
  const dueLabel = issue.dueDate ? formatDueDate(issue.dueDate) : null;

  return (
    <article className="group rounded-lg border bg-card p-4 text-card-foreground shadow-sm transition hover:border-primary/40 hover:shadow-md">
      <div className="flex items-start justify-between gap-4">
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <p className="text-xs font-semibold tracking-wide text-primary">
              {issue.key}
            </p>
            <PriorityIcon priority={issue.priority} />
          </div>
          <h3 className="mt-1 truncate font-semibold">{issue.summary}</h3>
        </div>
        <span
          className={cn(
            "shrink-0 rounded-full px-2.5 py-1 text-xs font-medium",
            statusColors[issue.status],
          )}
        >
          {statusLabels[issue.status]}
        </span>
      </div>
      <div className="mt-4 flex flex-wrap items-center gap-x-3 gap-y-2 text-xs text-muted-foreground">
        <span>{issue.projectName}</span>
        <span aria-hidden="true">&middot;</span>
        {issue.sprint ? (
          <>
            <span>{issue.sprint}</span>
            <span aria-hidden="true">&middot;</span>
          </>
        ) : null}
        <span>Updated {formatUpdated(issue.updatedAt)}</span>
        {dueLabel ? (
          <span
            className={cn(
              "font-medium",
              dueLabel.includes("overdue")
                ? "text-destructive"
                : "text-warning",
            )}
          >
            Due {dueLabel}
          </span>
        ) : null}
        {issue.webUrl ? (
          <a
            className="ml-auto opacity-0 transition group-hover:opacity-100"
            href={issue.webUrl}
            rel="noopener noreferrer"
            target="_blank"
            title="Open in Jira"
          >
            <ExternalLink size={14} />
          </a>
        ) : null}
      </div>
      {issue.labels.length > 0 ? (
        <div className="mt-3 flex flex-wrap gap-1.5">
          {issue.labels.map((label) => (
            <span
              className="rounded-md bg-secondary px-2 py-0.5 text-[11px] text-secondary-foreground"
              key={label}
            >
              {label}
            </span>
          ))}
        </div>
      ) : null}
    </article>
  );
}
