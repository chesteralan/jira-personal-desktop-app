import { Bell, Command, Moon, Search, Settings2, Sun } from "lucide-react";
import { useTheme, type Theme } from "@/app/theme";

const themeIcon: Record<Theme, React.JSX.Element> = {
  light: <Sun aria-hidden="true" size={16} />,
  dark: <Moon aria-hidden="true" size={16} />,
  system: <Settings2 aria-hidden="true" size={16} />,
};

const nextTheme: Record<Theme, Theme> = {
  system: "light",
  light: "dark",
  dark: "system",
};

function getGreeting(): string {
  const hour = new Date().getHours();
  if (hour < 12) return "Good morning";
  if (hour < 18) return "Good afternoon";
  return "Good evening";
}

interface HeaderProps {
  userName: string | null;
}

export function Header({ userName }: HeaderProps): React.JSX.Element {
  const { theme, setTheme } = useTheme();
  const today = new Date();
  const dateStr = today.toLocaleDateString("en-US", {
    weekday: "long",
    month: "long",
    day: "numeric",
  });

  return (
    <header className="flex items-center justify-between gap-4">
      <div>
        <p className="text-sm text-muted-foreground">{dateStr}</p>
        <h1 className="mt-1 text-2xl font-semibold tracking-tight">
          {getGreeting()}, {userName ?? "there"}
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
  );
}
