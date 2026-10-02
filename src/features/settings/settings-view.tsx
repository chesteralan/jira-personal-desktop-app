import { useEffect, useState } from "react";
import { Header } from "@/components/header";
import { ConnectForm } from "@/features/settings/connect-form";
import { useTheme, type Theme } from "@/app/theme";
import { useWorkspaceStore } from "@/stores/workspace-store";
import {
  getAppVersion,
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

const shortcuts: Array<{ keys: string; description: string }> = [
  { keys: "\u2318 1", description: "Today" },
  { keys: "\u2318 2", description: "My Tasks" },
  { keys: "\u2318 3", description: "Boards" },
  { keys: "\u2318 ,", description: "Settings" },
  { keys: "\u2318 B", description: "Toggle sidebar" },
  { keys: "\u2318 R", description: "Sync now" },
  { keys: "\u2318\u21E7 J", description: "Toggle window (global)" },
];

export function SettingsView(): React.JSX.Element {
  const { theme, setTheme } = useTheme();
  const info = useWorkspaceStore((s) => s.info);
  const [prefs, setPrefs] = useState<Preferences | null>(null);
  const [version, setVersion] = useState("0.1.0");

  useEffect(() => {
    void getPreferences().then(setPrefs);
    void getAppVersion().then(setVersion);
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
        {/* Jira Connection */}
        <section>
          <h2 className="text-lg font-semibold">Jira Connection</h2>
          <p className="mt-1 text-sm text-muted-foreground">
            Connect to your Jira Cloud instance using an API token or OAuth 2.0.
          </p>
          <div className="mt-4">
            <ConnectForm />
          </div>
        </section>

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

        {/* Keyboard shortcuts */}
        <section>
          <h2 className="text-lg font-semibold">Keyboard Shortcuts</h2>
          <div className="mt-4 rounded-lg border bg-card">
            <div className="divide-y">
              {shortcuts.map((s) => (
                <div
                  className="flex items-center justify-between px-4 py-2.5 text-sm"
                  key={s.keys}
                >
                  <span className="text-muted-foreground">{s.description}</span>
                  <kbd className="rounded border bg-muted px-2 py-0.5 font-mono text-xs">
                    {s.keys}
                  </kbd>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* Workspace info */}
        <section>
          <h2 className="text-lg font-semibold">Workspace</h2>
          <p className="mt-1 text-sm text-muted-foreground">
            Local database and sync status.
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
          <div className="mt-4 rounded-lg border bg-card p-4 text-sm">
            <p className="font-medium">Jira Personal v{version}</p>
            <p className="mt-1 text-muted-foreground">
              A fast, local-first desktop workspace for personal Jira work.
            </p>
            <p className="mt-3 text-xs text-muted-foreground">
              Built with Tauri, React, and Rust.
            </p>
          </div>
        </section>
      </div>
    </main>
  );
}
