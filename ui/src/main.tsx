import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { CssBaseline, ThemeProvider, createTheme } from "@mui/material";

import App from "./App";
import { ErrorBoundary } from "./components/ErrorBoundary";
import "./styles.css";

const theme = createTheme({
  palette: {
    mode: "light",
    primary: {
      main: "#0f766e"
    },
    secondary: {
      main: "#1d4ed8"
    },
    background: {
      default: "#f8fafc"
    }
  },
  shape: {
    borderRadius: 14
  },
  typography: {
    fontFamily: ["Lexend", "Noto Sans", "Segoe UI", "sans-serif"].join(",")
  }
});

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <ErrorBoundary>
        <App />
      </ErrorBoundary>
    </ThemeProvider>
  </StrictMode>
);
