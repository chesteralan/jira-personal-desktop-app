import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "@/App";
import { ThemeProvider } from "@/app/theme-provider";
import { ErrorBoundary } from "@/components/error-boundary";
import "@/styles/globals.css";

const root = document.getElementById("root");
if (root === null) {
  throw new Error("Root element not found");
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <ErrorBoundary>
      <ThemeProvider>
        <App />
      </ThemeProvider>
    </ErrorBoundary>
  </React.StrictMode>,
);
