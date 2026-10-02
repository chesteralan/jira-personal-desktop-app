import { useState } from "react";
import { ExternalLink, Globe, Key, Loader2, Unplug } from "lucide-react";
import { cn } from "@/lib/utils";
import { useWorkspaceStore } from "@/stores/workspace-store";
import { useIssueStore } from "@/stores/issue-store";

type AuthTab = "api-token" | "oauth";

export function ConnectForm(): React.JSX.Element {
  const info = useWorkspaceStore((s) => s.info);
  const loading = useWorkspaceStore((s) => s.loading);
  const error = useWorkspaceStore((s) => s.error);
  const connect = useWorkspaceStore((s) => s.connect);
  const oauthConnect = useWorkspaceStore((s) => s.oauthConnect);
  const disconnect = useWorkspaceStore((s) => s.disconnect);
  const clearError = useWorkspaceStore((s) => s.clearError);
  const refreshIssues = useIssueStore((s) => s.refresh);

  const [authTab, setAuthTab] = useState<AuthTab>("api-token");

  // API-token fields
  const [baseUrl, setBaseUrl] = useState("");
  const [email, setEmail] = useState("");
  const [apiToken, setApiToken] = useState("");

  // OAuth fields
  const [clientId, setClientId] = useState("");
  const [clientSecret, setClientSecret] = useState("");

  const isConnected = info.connectionStatus === "connected";

  const handleApiTokenConnect = async (e: React.FormEvent): Promise<void> => {
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

  const handleOAuthConnect = async (e: React.FormEvent): Promise<void> => {
    e.preventDefault();
    clearError();
    try {
      await oauthConnect({ clientId, clientSecret });
      await refreshIssues();
      setClientId("");
      setClientSecret("");
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
    <div className="space-y-4">
      {error ? (
        <div className="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          {error}
        </div>
      ) : null}

      {/* Auth method tabs */}
      <div className="flex gap-1 rounded-lg border bg-card p-1">
        <button
          className={cn(
            "flex items-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium transition-colors",
            authTab === "api-token"
              ? "bg-primary text-primary-foreground"
              : "text-muted-foreground hover:bg-muted hover:text-foreground",
          )}
          onClick={() => {
            setAuthTab("api-token");
            clearError();
          }}
          type="button"
        >
          <Key size={14} />
          API Token
        </button>
        <button
          className={cn(
            "flex items-center gap-2 rounded-md px-3 py-1.5 text-sm font-medium transition-colors",
            authTab === "oauth"
              ? "bg-primary text-primary-foreground"
              : "text-muted-foreground hover:bg-muted hover:text-foreground",
          )}
          onClick={() => {
            setAuthTab("oauth");
            clearError();
          }}
          type="button"
        >
          <Globe size={14} />
          OAuth 2.0
        </button>
      </div>

      {/* API Token form */}
      {authTab === "api-token" ? (
        <form
          className="space-y-4"
          onSubmit={(e) => void handleApiTokenConnect(e)}
        >
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
      ) : null}

      {/* OAuth form */}
      {authTab === "oauth" ? (
        <form
          className="space-y-4"
          onSubmit={(e) => void handleOAuthConnect(e)}
        >
          <p className="text-xs text-muted-foreground">
            Create an OAuth 2.0 (3LO) app in the{" "}
            <a
              className="text-primary hover:underline"
              href="https://developer.atlassian.com/console/myapps/"
              rel="noopener noreferrer"
              target="_blank"
            >
              Atlassian Developer Console
            </a>
            . Add a callback URL:{" "}
            <code className="rounded bg-muted px-1 py-0.5 text-[11px]">
              http://localhost:17042/callback
            </code>
          </p>

          <div>
            <label
              className="block text-sm font-medium text-foreground"
              htmlFor="clientId"
            >
              Client ID
            </label>
            <input
              autoComplete="off"
              className="mt-1 w-full rounded-md border bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
              id="clientId"
              onChange={(e) => setClientId(e.target.value)}
              placeholder="Your OAuth client ID"
              required
              value={clientId}
            />
          </div>

          <div>
            <label
              className="block text-sm font-medium text-foreground"
              htmlFor="clientSecret"
            >
              Client Secret
            </label>
            <input
              autoComplete="off"
              className="mt-1 w-full rounded-md border bg-card px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
              id="clientSecret"
              onChange={(e) => setClientSecret(e.target.value)}
              placeholder="Your OAuth client secret"
              required
              type="password"
              value={clientSecret}
            />
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
                Waiting for authorization...
              </>
            ) : (
              "Connect with OAuth"
            )}
          </button>
        </form>
      ) : null}
    </div>
  );
}
