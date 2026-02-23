import { useEffect } from "react";

import { Alert, CircularProgress, Container, Snackbar, Stack } from "@mui/material";

import { ReaderShell } from "./components/ReaderShell";
import { StarterShell } from "./components/StarterShell";
import { useAppStore } from "./store/appStore";

function normalizeKey(key: string): string {
  if (key === " ") {
    return "space";
  }
  return key.toLowerCase();
}

function matchesShortcut(event: KeyboardEvent, shortcut: string): boolean {
  const parts = shortcut.toLowerCase().split("+").map((part) => part.trim());
  const target = parts[parts.length - 1];
  if (parts.includes("ctrl") !== event.ctrlKey) {
    return false;
  }
  if (parts.includes("shift") !== event.shiftKey) {
    return false;
  }
  if (parts.includes("alt") !== event.altKey) {
    return false;
  }
  return normalizeKey(event.key) === target;
}

export default function App() {
  const {
    bootstrap,
    loadingBootstrap,
    loadingRecents,
    loadingCalibre,
    busy,
    error,
    clearError,
    toast,
    dismissToast,
    bootstrapState,
    session,
    reader,
    recents,
    calibreBooks,
    openSourcePath,
    openClipboardText,
    deleteRecent,
    refreshRecents,
    loadCalibreBooks,
    openCalibreBook,
    closeReaderSession,
    readerNextPage,
    readerPrevPage,
    readerSetPage,
    readerSentenceClick,
    readerNextSentence,
    readerPrevSentence,
    readerToggleTextOnly,
    readerSearchSetQuery,
    readerSearchNext,
    readerSearchPrev,
    readerApplySettings,
    toggleSettingsPanel,
    toggleStatsPanel,
    toggleTtsPanel
  } = useAppStore();

  useEffect(() => {
    void bootstrap();
  }, [bootstrap]);

  useEffect(() => {
    if (!bootstrapState || !session || session.mode !== "reader") {
      return;
    }

    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      const typingInInput =
        target?.tagName === "INPUT" || target?.tagName === "TEXTAREA" || target?.isContentEditable;
      if (typingInInput) {
        return;
      }

      if (matchesShortcut(event, bootstrapState.config.key_toggle_settings)) {
        event.preventDefault();
        void toggleSettingsPanel();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_toggle_stats)) {
        event.preventDefault();
        void toggleStatsPanel();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_toggle_tts)) {
        event.preventDefault();
        void toggleTtsPanel();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_next_sentence)) {
        event.preventDefault();
        void readerNextSentence();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_prev_sentence)) {
        event.preventDefault();
        void readerPrevSentence();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_toggle_search)) {
        event.preventDefault();
        const searchInput = document.querySelector<HTMLInputElement>(
          'input[data-reader-search-input="1"]'
        );
        searchInput?.focus();
        searchInput?.select();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_safe_quit)) {
        event.preventDefault();
        void closeReaderSession();
      }
    };

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [
    bootstrapState,
    session,
    closeReaderSession,
    readerNextSentence,
    readerPrevSentence,
    toggleSettingsPanel,
    toggleStatsPanel,
    toggleTtsPanel
  ]);

  return (
    <main className="min-h-screen bg-slate-50 text-slate-900">
      <Container maxWidth={false} className="px-2 py-4 md:px-4 md:py-6">
        <Stack spacing={2} alignItems="center">
          {loadingBootstrap ? <CircularProgress /> : null}

          {error ? (
            <Alert severity="error" onClose={clearError} className="w-full max-w-5xl">
              {error}
            </Alert>
          ) : null}

          {session && session.mode === "reader" && reader ? (
            <ReaderShell
              reader={reader}
              busy={busy}
              onCloseSession={closeReaderSession}
              onPrevPage={readerPrevPage}
              onNextPage={readerNextPage}
              onPrevSentence={readerPrevSentence}
              onNextSentence={readerNextSentence}
              onSetPage={readerSetPage}
              onSentenceClick={readerSentenceClick}
              onToggleTextOnly={readerToggleTextOnly}
              onSearchQuery={readerSearchSetQuery}
              onSearchNext={readerSearchNext}
              onSearchPrev={readerSearchPrev}
              onToggleSettingsPanel={toggleSettingsPanel}
              onToggleStatsPanel={toggleStatsPanel}
              onToggleTtsPanel={toggleTtsPanel}
              onApplySettings={readerApplySettings}
            />
          ) : (
            <StarterShell
              bootstrap={bootstrapState}
              recents={recents}
              calibreBooks={calibreBooks}
              busy={busy}
              loadingRecents={loadingRecents}
              loadingCalibre={loadingCalibre}
              onOpenPath={openSourcePath}
              onOpenClipboardText={openClipboardText}
              onDeleteRecent={deleteRecent}
              onRefreshRecents={refreshRecents}
              onLoadCalibre={loadCalibreBooks}
              onOpenCalibreBook={openCalibreBook}
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
