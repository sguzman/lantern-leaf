import { Alert, Stack, Typography } from "@mui/material";
import { Component, type ReactNode } from "react";

interface ErrorBoundaryProps {
  children: ReactNode;
}

interface ErrorBoundaryState {
  hasError: boolean;
  message: string;
}

export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  state: ErrorBoundaryState = {
    hasError: false,
    message: ""
  };

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return {
      hasError: true,
      message: error.message
    };
  }

  override componentDidCatch(error: Error): void {
    console.error("UI render error", error);
  }

  override render(): ReactNode {
    if (!this.state.hasError) {
      return this.props.children;
    }

    return (
      <main className="min-h-screen bg-slate-50 text-slate-900">
        <Stack className="mx-auto max-w-3xl px-6 py-8" spacing={2}>
          <Typography variant="h5" fontWeight={700}>
            UI Runtime Error
          </Typography>
          <Alert severity="error">
            {this.state.message || "An unexpected UI render error occurred."}
          </Alert>
        </Stack>
      </main>
    );
  }
}
