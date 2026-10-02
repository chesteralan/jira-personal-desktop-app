import {
  CheckCircle2,
  ChevronLeft,
  CircleDot,
  KanbanSquare,
  LayoutDashboard,
  Search,
  Settings2,
  UserRound,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { useUiStore, type AppView } from "@/stores/ui-store";
import { useWorkspaceStore } from "@/stores/workspace-store";

interface NavItemProps {
  icon: React.ReactNode;
  label: string;
  view: AppView;
  active: boolean;
  collapsed: boolean;
  shortcut?: string | undefined;
  onClick: () => void;
}

function NavItem({
  icon,
  label,
  view,
  active,
  collapsed,
  shortcut,
  onClick,
}: NavItemProps): React.JSX.Element {
  return (
    <button
      aria-current={active ? "page" : undefined}
      className={cn(
        "flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors",
        active
          ? "bg-primary/10 font-medium text-primary"
          : "text-muted-foreground hover:bg-muted hover:text-foreground",
        collapsed && "justify-center px-2",
      )}
      data-view={view}
      onClick={onClick}
      title={collapsed ? label : undefined}
    >
      {icon}
      {collapsed ? null : (
        <>
          <span className="flex-1 text-left">{label}</span>
          {shortcut ? (
            <kbd className="ml-auto text-[10px] text-muted-foreground/60">
              {shortcut}
            </kbd>
          ) : null}
        </>
      )}
    </button>
  );
}

export function Sidebar(): React.JSX.Element {
  const collapsed = useUiStore((s) => s.sidebarCollapsed);
  const activeView = useUiStore((s) => s.activeView);
  const setActiveView = useUiStore((s) => s.setActiveView);
  const toggleSidebar = useUiStore((s) => s.toggleSidebar);
  const info = useWorkspaceStore((s) => s.info);

  const navItems: Array<{
    icon: React.ReactNode;
    label: string;
    view: AppView;
    section: "work" | "other";
    shortcut?: string;
  }> = [
    {
      icon: <LayoutDashboard size={17} />,
      label: "Today",
      view: "today",
      section: "work",
      shortcut: "\u2318 1",
    },
    {
      icon: <CircleDot size={17} />,
      label: "My Tasks",
      view: "tasks",
      section: "work",
      shortcut: "\u2318 2",
    },
    {
      icon: <CheckCircle2 size={17} />,
      label: "Review",
      view: "review",
      section: "work",
    },
    {
      icon: <KanbanSquare size={17} />,
      label: "Boards",
      view: "boards",
      section: "work",
      shortcut: "\u2318 3",
    },
    {
      icon: <Search size={17} />,
      label: "Search",
      view: "search",
      section: "other",
    },
    {
      icon: <Settings2 size={17} />,
      label: "Settings",
      view: "settings",
      section: "other",
      shortcut: "\u2318 ,",
    },
  ];

  const workItems = navItems.filter((i) => i.section === "work");
  const otherItems = navItems.filter((i) => i.section === "other");

  return (
    <aside
      className={cn(
        "flex flex-col border-r bg-card transition-[width] duration-200",
        collapsed ? "w-[60px] px-2 py-5" : "w-[220px] px-3 py-5",
      )}
    >
      {/* Header */}
      <div
        className={cn(
          "flex items-center",
          collapsed ? "justify-center" : "justify-between gap-3 px-2",
        )}
      >
        {collapsed ? (
          <div className="grid size-9 place-items-center rounded-lg bg-primary text-sm font-bold text-primary-foreground">
            JP
          </div>
        ) : (
          <>
            <div className="flex items-center gap-3">
              <div className="grid size-9 place-items-center rounded-lg bg-primary text-sm font-bold text-primary-foreground">
                JP
              </div>
              <div>
                <p className="text-sm font-semibold">Jira Personal</p>
                <p className="text-xs text-muted-foreground">Local workspace</p>
              </div>
            </div>
            <button
              aria-label="Collapse sidebar"
              className="grid size-7 place-items-center rounded-md text-muted-foreground hover:bg-muted hover:text-foreground"
              onClick={toggleSidebar}
            >
              <ChevronLeft size={16} />
            </button>
          </>
        )}
      </div>

      {/* Navigation */}
      <nav aria-label="Primary" className="mt-8 space-y-1">
        {collapsed ? null : (
          <p className="px-2 pb-2 text-[11px] font-semibold tracking-widest text-muted-foreground">
            MY WORK
          </p>
        )}
        {workItems.map((item) => (
          <NavItem
            active={activeView === item.view}
            collapsed={collapsed}
            icon={item.icon}
            key={item.view}
            label={item.label}
            onClick={() => setActiveView(item.view)}
            shortcut={item.shortcut}
            view={item.view}
          />
        ))}

        {collapsed ? (
          <div className="my-4 border-t" />
        ) : (
          <p className="px-2 pt-6 pb-2 text-[11px] font-semibold tracking-widest text-muted-foreground">
            OTHER
          </p>
        )}
        {otherItems.map((item) => (
          <NavItem
            active={activeView === item.view}
            collapsed={collapsed}
            icon={item.icon}
            key={item.view}
            label={item.label}
            onClick={() => setActiveView(item.view)}
            shortcut={item.shortcut}
            view={item.view}
          />
        ))}
      </nav>

      {/* Workspace footer */}
      {collapsed ? null : (
        <div className="mt-auto rounded-lg border bg-muted/50 p-3">
          <div className="flex items-center gap-2 text-sm font-medium">
            <UserRound size={16} />
            {info.userDisplayName ?? "Not connected"}
          </div>
          <p className="mt-1 text-xs text-muted-foreground">
            {info.connectionStatus === "connected"
              ? `${info.issueCount} issues cached`
              : "Connect in Settings"}
          </p>
        </div>
      )}
    </aside>
  );
}
