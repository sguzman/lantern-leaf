import { useEffect } from "react";

import { Alert, CircularProgress, Container, Snackbar, Stack } from "@mui/material";

import { ReaderShell } from "./components/ReaderShell";
import { StarterShell } from "./components/StarterShell";
import { useAppStore } from "./store/appStore";

export default function App() {
  const {
    bootstrap,
    loadingBootstrap,
    loadingRecents,
    busy,
    error,
    clearError,
    toast,
    dismissToast,
    bootstrapState,
    session,
    recents,
    openSourcePath,
    openClipboardText,
    deleteRecent,
    refreshRecents,
    returnToStarter
  } = useAppStore();

  useEffect(() => {
    void bootstrap();
  }, [bootstrap]);

  return (
    <main className="min-h-screen bg-slate-50 text-slate-900">
      <Container maxWidth="xl" className="py-8">
        <Stack spacing={2} alignItems="center">
          {loadingBootstrap ? <CircularProgress /> : null}

          {error ? (
            <Alert severity="error" onClose={clearError} className="w-full max-w-4xl">
              {error}
            </Alert>
          ) : null}

          {session && session.mode === "reader" ? (
            <ReaderShell session={session} busy={busy} onBack={returnToStarter} />
          ) : (
            <StarterShell
              bootstrap={bootstrapState}
              recents={recents}
              busy={busy}
              loadingRecents={loadingRecents}
              onOpenPath={openSourcePath}
              onOpenClipboardText={openClipboardText}
              onDeleteRecent={deleteRecent}
              onRefreshRecents={refreshRecents}
            />
          )}
        </Stack>
      </Container>

      <Snackbar
        key={toast?.id}
        open={Boolean(toast)}
        autoHideDuration={2800}
        onClose={dismissToast}
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      >
        {toast ? (
          <Alert severity={toast.severity} onClose={dismissToast} variant="filled">
            {toast.message}
          </Alert>
        ) : (
          <span />
        )}
      </Snackbar>
    </main>
  );
}
