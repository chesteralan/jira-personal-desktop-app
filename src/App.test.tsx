import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { App } from "@/App";
import { ThemeProvider } from "@/app/theme-provider";

// Mock the Tauri IPC invoke function
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn((cmd: string) => {
    switch (cmd) {
      case "list_issues":
        return Promise.resolve([
          {
            id: "1",
            key: "TPT-7042",
            summary: "Update Snowplow tracking",
            status: "in_progress",
            priority: "high",
            projectKey: "TPT",
            projectName: "E-commerce",
            assignee: "Alchie",
            labels: ["analytics"],
            sprint: "Sprint 42",
            dueDate: null,
            updatedAt: new Date().toISOString(),
            webUrl: null,
          },
        ]);
      case "get_issue_counts":
        return Promise.resolve({
          total: 4,
          todo: 1,
          inProgress: 2,
          review: 1,
          done: 0,
        });
      case "get_workspace_info":
        return Promise.resolve({
          connectionStatus: "disconnected",
          lastSyncedAt: null,
          issueCount: 4,
          userDisplayName: "Alchie",
          jiraBaseUrl: null,
        });
      case "get_preferences":
        return Promise.resolve({
          theme: "system",
          sidebarCollapsed: false,
          syncIntervalSecs: 300,
          activeView: "today",
        });
      case "jira_restore_session":
        return Promise.resolve(false);
      default:
        return Promise.resolve(null);
    }
  }),
}));

// Mock the Tauri event listener
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

function renderApp(): void {
  render(
    <ThemeProvider>
      <App />
    </ThemeProvider>,
  );
}

describe("App", () => {
  it("renders the sidebar and today view", async () => {
    renderApp();
    expect(screen.getByText("Jira Personal")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("TPT-7042")).toBeInTheDocument();
    });
  });

  it("navigates between views via sidebar", async () => {
    const user = userEvent.setup();
    renderApp();

    const settingsButton = screen.getByRole("button", { name: "Settings" });
    await user.click(settingsButton);

    await waitFor(() => {
      expect(screen.getByText("Jira Connection")).toBeInTheDocument();
    });
  });

  it("cycles the theme", async () => {
    const user = userEvent.setup();
    renderApp();
    const themeButton = screen.getByRole("button", { name: "Theme: system" });
    await user.click(themeButton);
    expect(
      screen.getByRole("button", { name: "Theme: light" }),
    ).toBeInTheDocument();
  });

  it("shows connect form in settings when disconnected", async () => {
    const user = userEvent.setup();
    renderApp();

    const settingsButton = screen.getByRole("button", { name: "Settings" });
    await user.click(settingsButton);

    await waitFor(() => {
      expect(screen.getByLabelText("Jira Cloud URL")).toBeInTheDocument();
      expect(screen.getByLabelText("Email")).toBeInTheDocument();
      expect(screen.getByLabelText("API Token")).toBeInTheDocument();
      expect(
        screen.getByRole("button", { name: "Connect to Jira" }),
      ).toBeInTheDocument();
    });
  });
});
