import { useEffect, useMemo } from "react";

import {
  Alert,
  CircularProgress,
  Container,
  CssBaseline,
  Snackbar,
  Stack,
  ThemeProvider,
  createTheme
} from "@mui/material";

import { ReaderShell } from "./components/ReaderShell";
import { StarterShell } from "./components/StarterShell";
import { useAppStore } from "./store/appStore";
import { highlightBorder, mapFontFamily, mapFontWeight, toCssRgba } from "./theme/mapping";
import type { ThemeMode } from "./types";

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
    sourceOpenEvent,
    calibreLoadEvent,
    ttsStateEvent,
    pdfTranscriptionEvent,
    runtimeLogLevel,
    appSafeQuit,
    bootstrapState,
    session,
    reader,
    recents,
    calibreBooks,
    openSourcePath,
    openClipboardText,
    deleteRecent,
    refreshRecents,
    setRuntimeLogLevel,
    toggleTheme,
    loadCalibreBooks,
    openCalibreBook,
    closeReaderSession,
    readerNextPage,
    readerPrevPage,
    readerSetPage,
    readerSentenceClick,
    readerNextSentence,
    readerPrevSentence,
    readerTtsPlay,
    readerTtsPause,
    readerTtsTogglePlayPause,
    readerTtsPlayFromPageStart,
    readerTtsPlayFromHighlight,
    readerTtsSeekNext,
    readerTtsSeekPrev,
    readerTtsRepeatSentence,
    readerToggleTextOnly,
    readerSearchSetQuery,
    readerSearchNext,
    readerSearchPrev,
    readerApplySettings,
    toggleSettingsPanel,
    toggleStatsPanel,
    toggleTtsPanel
  } = useAppStore();

  const activeThemeMode: ThemeMode = reader?.settings.theme ?? bootstrapState?.config.theme ?? "day";
  const activeFontFamily = mapFontFamily(bootstrapState?.config.font_family);
  const activeFontWeight = mapFontWeight(bootstrapState?.config.font_weight);
  const dayHighlight =
    reader?.settings.day_highlight ?? bootstrapState?.config.day_highlight ?? { r: 0.2, g: 0.4, b: 0.7, a: 0.15 };
  const nightHighlight =
    reader?.settings.night_highlight ??
    bootstrapState?.config.night_highlight ??
    { r: 0.8, g: 0.8, b: 0.5, a: 0.2 };
  const activeHighlight = activeThemeMode === "night" ? nightHighlight : dayHighlight;

  const theme = useMemo(() => {
    const dark = activeThemeMode === "night";
    return createTheme({
      palette: {
        mode: dark ? "dark" : "light",
        primary: {
          main: dark ? "#79b93f" : "#5f8f2f"
        },
        secondary: {
          main: dark ? "#f4bc52" : "#f2b544"
        },
        background: {
          default: dark ? "#15110d" : "#f7f2e8",
          paper: dark ? "#201912" : "#fffaf0"
        }
      },
      shape: {
        borderRadius: 14
      },
      typography: {
        fontFamily: activeFontFamily,
        fontWeightRegular: activeFontWeight
      }
    });
  }, [activeFontFamily, activeFontWeight, activeThemeMode]);

  useEffect(() => {
    void bootstrap();
  }, [bootstrap]);

  useEffect(() => {
    const root = document.documentElement;
    root.style.setProperty("--app-bg", activeThemeMode === "night" ? "#15110d" : "#f7f2e8");
    root.style.setProperty("--app-fg", activeThemeMode === "night" ? "#f4ecdd" : "#2d2418");
    root.style.setProperty("--reader-highlight-bg", toCssRgba(activeHighlight));
    root.style.setProperty("--reader-highlight-border", highlightBorder(activeHighlight));
    root.style.setProperty("--reader-search-bg", activeThemeMode === "night" ? "#f4bc5230" : "#f2b5442d");
    root.style.setProperty("--reader-search-border", activeThemeMode === "night" ? "#f4bc52" : "#e39f21");
    root.style.setProperty("--app-color-scheme", activeThemeMode === "night" ? "dark" : "light");
  }, [activeHighlight, activeThemeMode]);

  useEffect(() => {
    if (!bootstrapState) {
      return;
    }

    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      const typingInInput =
        target?.tagName === "INPUT" || target?.tagName === "TEXTAREA" || target?.isContentEditable;
      if (typingInInput) {
        return;
      }

      if (matchesShortcut(event, bootstrapState.config.key_safe_quit)) {
        event.preventDefault();
        void appSafeQuit();
        return;
      }

      if (!session || session.mode !== "reader") {
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
      if (matchesShortcut(event, bootstrapState.config.key_toggle_play_pause)) {
        event.preventDefault();
        void readerTtsTogglePlayPause();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_next_sentence)) {
        event.preventDefault();
        void readerTtsSeekNext();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_prev_sentence)) {
        event.preventDefault();
        void readerTtsSeekPrev();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_repeat_sentence)) {
        event.preventDefault();
        void readerTtsRepeatSentence();
        return;
      }
      if (matchesShortcut(event, bootstrapState.config.key_toggle_search)) {
        event.preventDefault();
        const searchInput = document.querySelector<HTMLInputElement>(
          'input[data-reader-search-input="1"]'
        );
        searchInput?.focus();
        searchInput?.select();
      }
    };

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [
    bootstrapState,
    appSafeQuit,
    session,
    readerTtsTogglePlayPause,
    readerTtsSeekNext,
    readerTtsSeekPrev,
    readerTtsRepeatSentence,
    toggleSettingsPanel,
    toggleStatsPanel,
    toggleTtsPanel
  ]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <main className="app-root min-h-screen">
        <div
          data-testid="app-session-mode"
          data-mode={session?.mode ?? "unknown"}
          style={{ display: "none" }}
        />
        <div
          data-testid="app-last-source-open-event"
          data-phase={sourceOpenEvent?.phase ?? "none"}
          data-request-id={sourceOpenEvent?.request_id ?? 0}
          data-source-path={sourceOpenEvent?.source_path ?? ""}
          data-message={sourceOpenEvent?.message ?? ""}
          style={{ display: "none" }}
        />
        <div
          data-testid="app-last-pdf-event"
          data-phase={pdfTranscriptionEvent?.phase ?? "none"}
          data-request-id={pdfTranscriptionEvent?.request_id ?? 0}
          data-source-path={pdfTranscriptionEvent?.source_path ?? ""}
          data-message={pdfTranscriptionEvent?.message ?? ""}
          style={{ display: "none" }}
        />
        <div
          data-testid="app-last-calibre-event"
          data-phase={calibreLoadEvent?.phase ?? "none"}
          data-request-id={calibreLoadEvent?.request_id ?? 0}
          data-count={calibreLoadEvent?.count ?? -1}
          data-message={calibreLoadEvent?.message ?? ""}
          style={{ display: "none" }}
        />
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
                onTtsPlay={readerTtsPlay}
                onTtsPause={readerTtsPause}
                onTtsTogglePlayPause={readerTtsTogglePlayPause}
                onTtsPlayFromPageStart={readerTtsPlayFromPageStart}
                onTtsPlayFromHighlight={readerTtsPlayFromHighlight}
                onTtsSeekNext={readerTtsSeekNext}
                onTtsSeekPrev={readerTtsSeekPrev}
                onTtsRepeatSentence={readerTtsRepeatSentence}
                onApplySettings={readerApplySettings}
                ttsStateEvent={ttsStateEvent}
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
                sourceOpenEvent={sourceOpenEvent}
                calibreLoadEvent={calibreLoadEvent}
                pdfTranscriptionEvent={pdfTranscriptionEvent}
                runtimeLogLevel={runtimeLogLevel}
                onSetRuntimeLogLevel={setRuntimeLogLevel}
                onToggleTheme={toggleTheme}
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
    </ThemeProvider>
  );
}
