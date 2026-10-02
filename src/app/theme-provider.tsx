import { useEffect, useMemo, useState, type ReactNode } from "react";
import { ThemeContext, type Theme, type ThemeContextValue } from "@/app/theme";

interface ThemeProviderProps {
  children: ReactNode;
  defaultTheme?: Theme;
  storageKey?: string;
}

function applyTheme(theme: Theme): void {
  const root = document.documentElement;
  const isDark =
    theme === "dark" ||
    (theme === "system" &&
      window.matchMedia("(prefers-color-scheme: dark)").matches);
  root.classList.toggle("dark", isDark);
}

export function ThemeProvider({
  children,
  defaultTheme = "system",
  storageKey = "jira-personal-theme",
}: ThemeProviderProps): React.JSX.Element {
  const [theme, setThemeState] = useState<Theme>(() => {
    const storedTheme = window.localStorage.getItem(storageKey);
    return storedTheme === "dark" ||
      storedTheme === "light" ||
      storedTheme === "system"
      ? storedTheme
      : defaultTheme;
  });

  useEffect(() => {
    applyTheme(theme);
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = (): void => applyTheme(theme);
    media.addEventListener("change", handleChange);
    return () => media.removeEventListener("change", handleChange);
  }, [theme]);

  const value = useMemo<ThemeContextValue>(
    () => ({
      theme,
      setTheme: (nextTheme): void => {
        window.localStorage.setItem(storageKey, nextTheme);
        setThemeState(nextTheme);
      },
    }),
    [storageKey, theme],
  );

  return (
    <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
  );
}
