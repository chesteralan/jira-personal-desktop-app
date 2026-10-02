import { useState } from "react";
import { ExternalLink, Loader2, Unplug } from "lucide-react";
import { cn } from "@/lib/utils";
import { useWorkspaceStore } from "@/stores/workspace-store";
import { useIssueStore } from "@/stores/issue-store";

export function ConnectForm(): React.JSX.Element {
  const info = useWorkspaceStore((s) => s.info);
  const loading = useWorkspaceStore((s) => s.loading);
  const error = useWorkspaceStore((s) => s.error);
  const connect = useWorkspaceStore((s) => s.connect);
  const disconnect = useWorkspaceStore((s) => s.disconnect);
  const clearError = useWorkspaceStore((s) => s.clearError);
  const refreshIssues = useIssueStore((s) => s.refresh);

  const [baseUrl, setBaseUrl] = useState("");
  const [email, setEmail] = useState("");
  const [apiToken, setApiToken] = useState("");

  const isConnected = info.connectionStatus === "connected";

  const handleConnect = async (e: React.FormEvent): Promise<void> => {
    e.preventDefault();
    clearError();
    try {
      await connect({ baseUrl, email, apiToken });
      await refreshIssues();
      setBaseUrl("");
      setEmail("");
      setApiToken("");
    } catch {
      // Error is already set in the store
    }
  };

  const handleDisconnect = async (): Promise<void> => {
    await disconnect();
    await refreshIssues();
  };

  if (isConnected) {
    return (
      <div className="rounded-lg border bg-card p-4">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-success">Connected</p>
            <p className="mt-1 text-sm text-muted-foreground">
              {info.userDisplayName} &middot; {info.jiraBaseUrl}
            </p>
            <p className="mt-1 text-sm text-muted-foreground">
              {info.issueCount} issues cached
            </p>
          </div>
          <button
            className="flex items-center gap-2 rounded-md border bg-card px-3 py-1.5 text-sm font-medium text-destructive hover:bg-destructive/10"
            onClick={() => void handleDisconnect()}
          >
            <Unplug size={14} />
            Disconnect
          </button>
        </div>
      </div>
    );
  }

  return (
    <form className="space-y-4" onSubmit={(e) => void handleConnect(e)}>
      {error ? (
        <div className="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          {error}
        </div>
      ) : null}

      <div>
        <label
          className="block text-sm font-medium text-foreground"
          htmlFor="baseUrl"
        >
          Jira Cloud URL
        </label>
        <input
          autoComplete="url"
          className="mt-1 w-full rounded-md border bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
          id="baseUrl"
          onChange={(e) => setBaseUrl(e.target.value)}
          placeholder="https://yourorg.atlassian.net"
          required
          type="url"
          value={baseUrl}
        />
      </div>

      <div>
        <label
          className="block text-sm font-medium text-foreground"
          htmlFor="email"
        >
          Email
        </label>
        <input
          autoComplete="email"
          className="mt-1 w-full rounded-md border bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
          id="email"
          onChange={(e) => setEmail(e.target.value)}
          placeholder="you@example.com"
          required
          type="email"
          value={email}
        />
      </div>

      <div>
        <label
          className="block text-sm font-medium text-foreground"
          htmlFor="apiToken"
        >
          API Token
        </label>
        <input
          autoComplete="off"
          className="mt-1 w-full rounded-md border bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
          id="apiToken"
          onChange={(e) => setApiToken(e.target.value)}
          placeholder="Your Jira API token"
          required
          type="password"
          value={apiToken}
        />
        <a
          className="mt-1 inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-primary"
          href="https://id.atlassian.com/manage-profile/security/api-tokens"
          rel="noopener noreferrer"
          target="_blank"
        >
          Generate an API token
          <ExternalLink size={10} />
        </a>
      </div>

      <button
        className={cn(
          "flex w-full items-center justify-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground",
          loading && "opacity-70",
        )}
        disabled={loading}
        type="submit"
      >
        {loading ? (
          <>
            <Loader2 className="animate-spin" size={16} />
            Connecting...
          </>
        ) : (
          "Connect to Jira"
        )}
      </button>
    </form>
  );
}
