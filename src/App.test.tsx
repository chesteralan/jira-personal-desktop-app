import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { App } from "@/App";
import { ThemeProvider } from "@/app/theme-provider";

function renderApp(): void {
  render(
    <ThemeProvider>
      <App />
    </ThemeProvider>,
  );
}

describe("App", () => {
  it("renders the personal work preview", () => {
    renderApp();
    expect(
      screen.getByRole("heading", { name: "Good afternoon, Alchie" }),
    ).toBeInTheDocument();
    expect(screen.getByText("TPT-7042")).toBeInTheDocument();
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
});
