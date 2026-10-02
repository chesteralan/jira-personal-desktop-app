import { useEffect, useState } from "react";
import { Header } from "@/components/header";
import { useTheme, type Theme } from "@/app/theme";
import { useWorkspaceStore } from "@/stores/workspace-store";
import {
  getPreferences,
  savePreferences,
  type Preferences,
} from "@/services/ipc";
import { cn } from "@/lib/utils";

const themeOptions: Array<{ label: string; value: Theme }> = [
  { label: "Light", value: "light" },
  { label: "Dark", value: "dark" },
  { label: "System", value: "system" },
];

export function SettingsView(): React.JSX.Element {
  const { theme, setTheme } = useTheme();
  const info = useWorkspaceStore((s) => s.info);
  const [prefs, setPrefs] = useState<Preferences | null>(null);

  useEffect(() => {
    void getPreferences().then(setPrefs);
  }, []);

  const handleThemeChange = async (newTheme: Theme): Promise<void> => {
    setTheme(newTheme);
    if (prefs) {
      const updated = { ...prefs, theme: newTheme };
      setPrefs(updated);
      await savePreferences(updated);
    }
  };

  return (
    <main className="min-w-0 px-8 py-7">
      <Header userName={info.userDisplayName} />

      <div className="mt-8 max-w-2xl space-y-8">
        {/* Appearance */}
        <section>
          <h2 className="text-lg font-semibold">Appearance</h2>
          <p className="mt-1 text-sm text-muted-foreground">
            Choose your preferred theme.
          </p>
          <div className="mt-4 flex gap-2">
            {themeOptions.map((option) => (
              <button
                className={cn(
                  "rounded-md border px-4 py-2 text-sm font-medium transition-colors",
                  theme === option.value
                    ? "border-primary bg-primary/10 text-primary"
                    : "bg-card text-muted-foreground hover:bg-muted hover:text-foreground",
                )}
                key={option.value}
                onClick={() => void handleThemeChange(option.value)}
              >
                {option.label}
              </button>
            ))}
          </div>
        </section>

        {/* Workspace info */}
        <section>
          <h2 className="text-lg font-semibold">Workspace</h2>
          <p className="mt-1 text-sm text-muted-foreground">
            Connection and sync settings.
          </p>
          <div className="mt-4 rounded-lg border bg-card p-4">
            <dl className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <dt className="text-muted-foreground">Status</dt>
                <dd className="mt-1 font-medium capitalize">
                  {info.connectionStatus}
                </dd>
              </div>
              <div>
                <dt className="text-muted-foreground">Issues cached</dt>
                <dd className="mt-1 font-medium">{info.issueCount}</dd>
              </div>
              <div>
                <dt className="text-muted-foreground">User</dt>
                <dd className="mt-1 font-medium">
                  {info.userDisplayName ?? "Not connected"}
                </dd>
              </div>
              <div>
                <dt className="text-muted-foreground">Last synced</dt>
                <dd className="mt-1 font-medium">
                  {info.lastSyncedAt
                    ? new Date(info.lastSyncedAt).toLocaleString()
                    : "Never"}
                </dd>
              </div>
            </dl>
          </div>
        </section>

        {/* About */}
        <section>
          <h2 className="text-lg font-semibold">About</h2>
          <p className="mt-1 text-sm text-muted-foreground">
            Jira Personal v0.1.0 &middot; Local-first workspace for personal
            Jira work.
          </p>
        </section>
      </div>
    </main>
  );
}
