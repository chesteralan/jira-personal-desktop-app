import {
  Bell,
  CheckCircle2,
  CircleDot,
  Clock3,
  Command,
  LayoutDashboard,
  Moon,
  RefreshCw,
  Search,
  Settings2,
  Sun,
  UserRound,
} from "lucide-react";
import { useTheme, type Theme } from "@/app/theme";
import { mockIssues, type MockIssue } from "@/test/fixtures";

const themeIcon: Record<Theme, React.JSX.Element> = {
  light: <Sun aria-hidden="true" size={16} />,
  dark: <Moon aria-hidden="true" size={16} />,
  system: <Settings2 aria-hidden="true" size={16} />,
};

interface IssueCardProps {
  issue: MockIssue;
}

function IssueCard({ issue }: IssueCardProps): React.JSX.Element {
  return (
    <article className="rounded-lg border bg-card p-4 text-card-foreground shadow-sm transition hover:border-primary/40 hover:shadow-md">
      <div className="flex items-start justify-between gap-4">
        <div>
          <p className="text-xs font-semibold tracking-wide text-primary">
            {issue.key}
          </p>
          <h3 className="mt-1 font-semibold">{issue.summary}</h3>
        </div>
        <span className="rounded-full bg-secondary px-2.5 py-1 text-xs font-medium text-secondary-foreground">
          {issue.priority}
        </span>
      </div>
      <div className="mt-4 flex flex-wrap items-center gap-x-3 gap-y-2 text-xs text-muted-foreground">
        <span>{issue.project}</span>
        <span aria-hidden="true">·</span>
        <span>{issue.status}</span>
        <span className="ml-auto">Updated {issue.updatedMinutesAgo}m ago</span>
        {issue.dueLabel === undefined ? null : (
          <span className="font-medium text-warning">Due {issue.dueLabel}</span>
        )}
      </div>
    </article>
  );
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
export function App(): React.JSX.Element {
  const { theme, setTheme } = useTheme();
  const nextTheme: Record<Theme, Theme> = {
    system: "light",
    light: "dark",
    dark: "system",
  };

  return (
    <div className="grid min-h-screen grid-cols-[220px_1fr] bg-background text-foreground">
      <aside className="flex flex-col border-r bg-card px-3 py-5">
        <div className="flex items-center gap-3 px-2">
          <div className="grid size-9 place-items-center rounded-lg bg-primary text-sm font-bold text-primary-foreground">
            JP
          </div>
          <div>
            <p className="text-sm font-semibold">Jira Personal</p>
            <p className="text-xs text-muted-foreground">Local workspace</p>
          </div>
        </div>
        <nav aria-label="Primary" className="mt-8 space-y-1">
          <p className="px-2 pb-2 text-[11px] font-semibold tracking-widest text-muted-foreground">
            MY WORK
          </p>
          <a
            className="flex items-center gap-3 rounded-md bg-primary/10 px-3 py-2 text-sm font-medium text-primary"
            href="#today"
          >
            <LayoutDashboard size={17} />
            Today
          </a>
          <a
            className="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:bg-muted hover:text-foreground"
            href="#tasks"
          >
            <CircleDot size={17} />
            My Tasks
          </a>
          <a
            className="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:bg-muted hover:text-foreground"
            href="#review"
          >
            <CheckCircle2 size={17} />
            Review
          </a>
          <p className="px-2 pt-6 pb-2 text-[11px] font-semibold tracking-widest text-muted-foreground">
            OTHER
          </p>
          <a
            className="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:bg-muted hover:text-foreground"
            href="#search"
          >
            <Search size={17} />
            Search
          </a>
          <a
            className="flex items-center gap-3 rounded-md px-3 py-2 text-sm text-muted-foreground hover:bg-muted hover:text-foreground"
            href="#settings"
          >
            <Settings2 size={17} />
            Settings
          </a>
        </nav>
        <div className="mt-auto rounded-lg border bg-muted/50 p-3">
          <div className="flex items-center gap-2 text-sm font-medium">
            <UserRound size={16} />
            Mock workspace
          </div>
          <p className="mt-1 text-xs text-muted-foreground">
            Jira connection arrives in M3.
          </p>
        </div>
      </aside>
      <main id="today" className="min-w-0 px-8 py-7">
        <header className="flex items-center justify-between gap-4">
          <div>
            <p className="text-sm text-muted-foreground">Friday, October 2</p>
            <h1 className="mt-1 text-2xl font-semibold tracking-tight">
              Good afternoon, Alchie
            </h1>
          </div>
          <div className="flex items-center gap-2">
            <button
              aria-label="Open command menu"
              className="flex h-9 items-center gap-2 rounded-md border bg-card px-3 text-sm text-muted-foreground"
            >
              <Search size={15} />
              Search
              <span className="ml-5 flex items-center gap-1 rounded border bg-muted px-1.5 py-0.5 text-[11px]">
                <Command size={10} />K
              </span>
            </button>
            <button
              aria-label={`Theme: ${theme}`}
              className="grid size-9 place-items-center rounded-md border bg-card"
              onClick={() => setTheme(nextTheme[theme])}
            >
              {themeIcon[theme]}
            </button>
            <button
              aria-label="Notifications"
              className="grid size-9 place-items-center rounded-md border bg-card"
            >
              <Bell size={16} />
            </button>
          </div>
        </header>
        <section
          aria-label="Synchronization status"
          className="mt-7 flex items-center justify-between rounded-lg border bg-card px-4 py-3"
        >
          <div className="flex items-center gap-2 text-sm">
            <span className="size-2 rounded-full bg-success" />
            <span className="font-medium">Mock data ready</span>
            <span className="text-muted-foreground">· Local cache preview</span>
          </div>
          <button className="flex items-center gap-2 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground">
            <RefreshCw size={14} />
            Sync
          </button>
        </section>
        <div className="mt-7 grid grid-cols-3 gap-4">
          <div className="rounded-lg border bg-card p-4">
            <p className="text-sm text-muted-foreground">Needs attention</p>
            <p className="mt-2 text-3xl font-semibold">3</p>
          </div>
          <div className="rounded-lg border bg-card p-4">
            <p className="text-sm text-muted-foreground">In progress</p>
            <p className="mt-2 text-3xl font-semibold">2</p>
          </div>
          <div className="rounded-lg border bg-card p-4">
            <p className="text-sm text-muted-foreground">Due this week</p>
            <p className="mt-2 text-3xl font-semibold">2</p>
          </div>
        </div>
        <section className="mt-8" aria-labelledby="active-work">
          <div className="mb-4 flex items-center justify-between">
            <div>
              <h2 id="active-work" className="text-lg font-semibold">
                Active work
              </h2>
              <p className="text-sm text-muted-foreground">
                A deterministic preview of your personal Jira queue.
              </p>
            </div>
            <span className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <Clock3 size={14} />4 issues
            </span>
          </div>
          <div className="grid grid-cols-2 gap-4">
            {mockIssues.map((issue) => (
              <IssueCard issue={issue} key={issue.key} />
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}
