import { create, type StateCreator } from "zustand";

import { backendApi, type BackendApi } from "../api/tauri";
import type {
  BootstrapState,
  BridgeError,
  CalibreBook,
  CalibreLoadEvent,
  LogLevelEvent,
  PdfTranscriptionEvent,
  ReaderSettingsPatch,
  ReaderSnapshot,
  RecentBook,
  SessionState,
  SourceOpenEvent,
  TtsStateEvent
} from "../types";

type ToastSeverity = "info" | "success" | "error";
const TELEMETRY_LIMIT = 200;

export interface ToastMessage {
  id: number;
  severity: ToastSeverity;
  message: string;
}

export interface ActionTelemetry {
  id: number;
  action: string;
  started_at_unix_ms: number;
  duration_ms: number;
  ok: boolean;
  error: string | null;
}

export interface AppStore {
  bootstrapState: BootstrapState | null;
  session: SessionState | null;
  reader: ReaderSnapshot | null;
  recents: RecentBook[];
  calibreBooks: CalibreBook[];
  telemetry: ActionTelemetry[];
  loadingBootstrap: boolean;
  loadingRecents: boolean;
  loadingCalibre: boolean;
  busy: boolean;
  error: string | null;
  toast: ToastMessage | null;
  sourceOpenEvent: SourceOpenEvent | null;
  calibreLoadEvent: CalibreLoadEvent | null;
  ttsStateEvent: TtsStateEvent | null;
  pdfTranscriptionEvent: PdfTranscriptionEvent | null;
  logLevelEvent: LogLevelEvent | null;
  runtimeLogLevel: string;
  sourceOpenSubscribed: boolean;
  calibreSubscribed: boolean;
  ttsStateSubscribed: boolean;
  pdfTranscriptionSubscribed: boolean;
  logLevelSubscribed: boolean;
  sessionStateSubscribed: boolean;
  readerStateSubscribed: boolean;
  lastSessionEventRequestId: number;
  lastReaderEventRequestId: number;
  appSafeQuit: () => Promise<void>;
  bootstrap: () => Promise<void>;
  refreshRecents: () => Promise<void>;
  openSourcePath: (path: string) => Promise<void>;
  openClipboardText: (text: string) => Promise<void>;
  deleteRecent: (path: string) => Promise<void>;
  returnToStarter: () => Promise<void>;
  closeReaderSession: () => Promise<void>;
  refreshReaderSnapshot: () => Promise<void>;
  readerNextPage: () => Promise<void>;
  readerPrevPage: () => Promise<void>;
  readerSetPage: (page: number) => Promise<void>;
  readerSentenceClick: (sentenceIdx: number) => Promise<void>;
  readerNextSentence: () => Promise<void>;
  readerPrevSentence: () => Promise<void>;
  readerToggleTextOnly: () => Promise<void>;
  readerApplySettings: (patch: ReaderSettingsPatch) => Promise<void>;
  readerSearchSetQuery: (query: string) => Promise<void>;
  readerSearchNext: () => Promise<void>;
  readerSearchPrev: () => Promise<void>;
  readerTtsPlay: () => Promise<void>;
  readerTtsPause: () => Promise<void>;
  readerTtsTogglePlayPause: () => Promise<void>;
  readerTtsPlayFromPageStart: () => Promise<void>;
  readerTtsPlayFromHighlight: () => Promise<void>;
  readerTtsSeekNext: () => Promise<void>;
  readerTtsSeekPrev: () => Promise<void>;
  readerTtsRepeatSentence: () => Promise<void>;
  toggleSettingsPanel: () => Promise<void>;
  toggleStatsPanel: () => Promise<void>;
  toggleTtsPanel: () => Promise<void>;
  loadCalibreBooks: (forceRefresh?: boolean) => Promise<void>;
  openCalibreBook: (bookId: number) => Promise<void>;
  setRuntimeLogLevel: (level: string) => Promise<void>;
  clearError: () => void;
  dismissToast: () => void;
  clearTelemetry: () => void;
}

function toMessage(error: unknown): string {
  if (typeof error === "object" && error !== null && "message" in error) {
    const withMessage = error as { message: unknown };
    if (typeof withMessage.message === "string") {
      return withMessage.message;
    }
  }
  if (error instanceof Error) {
    return error.message;
  }
  return String(error);
}

function toBridgeError(error: unknown): BridgeError {
  if (typeof error === "object" && error !== null && "code" in error && "message" in error) {
    const structured = error as { code: unknown; message: unknown };
    if (typeof structured.code === "string" && typeof structured.message === "string") {
      return {
        code: structured.code,
        message: structured.message
      };
    }
  }
  return {
    code: "unknown_error",
    message: toMessage(error)
  };
}

function buildToast(severity: ToastSeverity, message: string): ToastMessage {
  return {
    id: Date.now(),
    severity,
    message
  };
}

function appendTelemetry(
  set: (partial: Partial<AppStore>) => void,
  get: () => AppStore,
  telemetry: ActionTelemetry
): void {
  const next = [telemetry, ...get().telemetry];
  if (next.length > TELEMETRY_LIMIT) {
    next.length = TELEMETRY_LIMIT;
  }
  set({ telemetry: next });
}

function finishTelemetry(
  set: (partial: Partial<AppStore>) => void,
  get: () => AppStore,
  action: string,
  startedAt: number,
  ok: boolean,
  error: string | null
): void {
  appendTelemetry(set, get, {
    id: Date.now(),
    action,
    started_at_unix_ms: startedAt,
    duration_ms: Date.now() - startedAt,
    ok,
    error
  });
}

function togglePanels(
  panels: SessionState["panels"],
  panel: "show_settings" | "show_stats" | "show_tts"
): SessionState["panels"] {
  const next = {
    ...panels,
    [panel]: !panels[panel]
  };
  if (panel === "show_settings" && next.show_settings) {
    next.show_stats = false;
  }
  if (panel === "show_stats" && next.show_stats) {
    next.show_settings = false;
  }
  return next;
}

async function withBusy(
  set: (partial: Partial<AppStore>) => void,
  get: () => AppStore,
  action: string,
  fn: () => Promise<void>
): Promise<void> {
  const startedAt = Date.now();
  set({ busy: true, error: null });
  try {
    await fn();
    finishTelemetry(set, get, action, startedAt, true, null);
  } catch (error) {
    finishTelemetry(set, get, action, startedAt, false, toMessage(error));
  } finally {
    set({ busy: false });
  }
}

export function createAppStoreState(backend: BackendApi): StateCreator<AppStore> {
  return (set, get) => ({
  bootstrapState: null,
  session: null,
  reader: null,
  recents: [],
  calibreBooks: [],
  telemetry: [],
  loadingBootstrap: false,
  loadingRecents: false,
  loadingCalibre: false,
  busy: false,
  error: null,
  toast: null,
  sourceOpenEvent: null,
  calibreLoadEvent: null,
  ttsStateEvent: null,
  pdfTranscriptionEvent: null,
  logLevelEvent: null,
  runtimeLogLevel: "info",
  sourceOpenSubscribed: false,
  calibreSubscribed: false,
  ttsStateSubscribed: false,
  pdfTranscriptionSubscribed: false,
  logLevelSubscribed: false,
  sessionStateSubscribed: false,
  readerStateSubscribed: false,
  lastSessionEventRequestId: 0,
  lastReaderEventRequestId: 0,

  appSafeQuit: async () => {
    try {
      await backend.appSafeQuit();
      const session = get().session;
      if (session) {
        set({
          session: { ...session, mode: "starter", active_source_path: null, open_in_flight: false },
          reader: null
        });
      } else {
        set({ reader: null });
      }
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  bootstrap: async () => {
    if (get().loadingBootstrap) {
      return;
    }
    const startedAt = Date.now();
    set({ loadingBootstrap: true, error: null });
    try {
      if (!get().sourceOpenSubscribed) {
        await backend.onSourceOpen((event) => {
          set({ sourceOpenEvent: event });
          if (event.phase === "cancelled") {
            const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
            set({
              toast: buildToast(
                "info",
                `${event.message ?? "Source open cancelled"}${suffix}`
              )
            });
            return;
          }
          if (event.phase === "failed") {
            const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
            set({
              toast: buildToast("error", `${event.message ?? "Source open failed"}${suffix}`)
            });
          }
        });
        set({ sourceOpenSubscribed: true });
      }
      if (!get().calibreSubscribed) {
        await backend.onCalibreLoad((event) => {
          set({ calibreLoadEvent: event });
          if (event.phase === "failed") {
            const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
            set({
              toast: buildToast("error", `${event.message ?? "Calibre load failed"}${suffix}`)
            });
          }
        });
        set({ calibreSubscribed: true });
      }
      if (!get().ttsStateSubscribed) {
        await backend.onTtsState((event) => {
          set({ ttsStateEvent: event });
        });
        set({ ttsStateSubscribed: true });
      }
      if (!get().pdfTranscriptionSubscribed) {
        await backend.onPdfTranscription((event) => {
          set({ pdfTranscriptionEvent: event });
          if (event.phase === "failed") {
            const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
            set({
              toast: buildToast(
                "error",
                `${event.message ?? "PDF transcription failed"}${suffix}`
              )
            });
          }
        });
        set({ pdfTranscriptionSubscribed: true });
      }
      if (!get().logLevelSubscribed) {
        await backend.onLogLevel((event) => {
          set({
            logLevelEvent: event,
            runtimeLogLevel: event.level
          });
        });
        set({ logLevelSubscribed: true });
      }
      if (!get().sessionStateSubscribed) {
        await backend.onSessionState((event) => {
          set((current) => {
            if (event.request_id < current.lastSessionEventRequestId) {
              return {};
            }
            const next: Partial<AppStore> = {
              session: event.session,
              lastSessionEventRequestId: event.request_id
            };
            if (event.session.mode !== "reader") {
              next.reader = null;
              next.lastReaderEventRequestId = Math.max(
                current.lastReaderEventRequestId,
                event.request_id
              );
            }
            return next;
          });
        });
        set({ sessionStateSubscribed: true });
      }
      if (!get().readerStateSubscribed) {
        await backend.onReaderState((event) => {
          set((current) => {
            if (event.request_id < current.lastReaderEventRequestId) {
              return {};
            }
            const nextSession: SessionState = current.session
              ? {
                  ...current.session,
                  mode: "reader",
                  active_source_path: event.reader.source_path,
                  open_in_flight: false,
                  panels: event.reader.panels
                }
              : {
                  mode: "reader",
                  active_source_path: event.reader.source_path,
                  open_in_flight: false,
                  panels: event.reader.panels
                };
            return {
              session: nextSession,
              reader: event.reader,
              lastReaderEventRequestId: event.request_id,
              lastSessionEventRequestId: Math.max(
                current.lastSessionEventRequestId,
                event.request_id
              )
            };
          });
        });
        set({ readerStateSubscribed: true });
      }

      const [bootstrapState, session, recents] = await Promise.all([
        backend.sessionGetBootstrap(),
        backend.sessionGetState(),
        backend.recentList()
      ]);

      let reader: ReaderSnapshot | null = null;
      if (session.mode === "reader") {
        try {
          reader = await backend.readerGetSnapshot();
        } catch {
          reader = null;
        }
      }

      set({
        bootstrapState,
        session,
        recents,
        reader,
        runtimeLogLevel: bootstrapState.config.log_level
      });
      finishTelemetry(set, get, "bootstrap", startedAt, true, null);
    } catch (error) {
      const message = toMessage(error);
      set({ error: message });
      finishTelemetry(set, get, "bootstrap", startedAt, false, message);
    } finally {
      set({ loadingBootstrap: false });
    }
  },

  refreshRecents: async () => {
    const startedAt = Date.now();
    set({ loadingRecents: true, error: null });
    try {
      const recents = await backend.recentList();
      set({ recents });
      finishTelemetry(set, get, "refreshRecents", startedAt, true, null);
    } catch (error) {
      const message = toMessage(error);
      set({ error: message });
      finishTelemetry(set, get, "refreshRecents", startedAt, false, message);
    } finally {
      set({ loadingRecents: false });
    }
  },

  openSourcePath: async (path) => {
    await withBusy(set, get, "openSourcePath", async () => {
      try {
        const result = await backend.sourceOpenPath(path);
        const recents = await backend.recentList();
        set({
          session: result.session,
          reader: result.reader,
          recents,
          toast: buildToast("success", "Source opened")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        if (bridgeError.code === "open_cancelled") {
          set({
            toast: buildToast("info", bridgeError.message)
          });
          return;
        }
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  openClipboardText: async (text) => {
    await withBusy(set, get, "openClipboardText", async () => {
      try {
        const result = await backend.sourceOpenClipboardText(text);
        const recents = await backend.recentList();
        set({
          session: result.session,
          reader: result.reader,
          recents,
          toast: buildToast("success", "Clipboard text opened")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        if (bridgeError.code === "open_cancelled") {
          set({
            toast: buildToast("info", bridgeError.message)
          });
          return;
        }
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  deleteRecent: async (path) => {
    await withBusy(set, get, "deleteRecent", async () => {
      try {
        await backend.recentDelete(path);
        const recents = await backend.recentList();
        set({
          recents,
          toast: buildToast("success", "Recent entry deleted")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        if (bridgeError.code === "open_cancelled") {
          set({
            toast: buildToast("info", bridgeError.message)
          });
          return;
        }
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  returnToStarter: async () => {
    await withBusy(set, get, "returnToStarter", async () => {
      try {
        const session = await backend.sessionReturnToStarter();
        set({
          session,
          reader: null
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  closeReaderSession: async () => {
    await withBusy(set, get, "closeReaderSession", async () => {
      try {
        const session = await backend.readerCloseSession();
        set({
          session,
          reader: null
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  refreshReaderSnapshot: async () => {
    const session = get().session;
    if (!session || session.mode !== "reader") {
      return;
    }
    try {
      const reader = await backend.readerGetSnapshot();
      set({ reader });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({ error: bridgeError.message });
    }
  },

  readerNextPage: async () => {
    try {
      const reader = await backend.readerNextPage();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerPrevPage: async () => {
    try {
      const reader = await backend.readerPrevPage();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSetPage: async (page) => {
    try {
      const reader = await backend.readerSetPage(page);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSentenceClick: async (sentenceIdx) => {
    try {
      const reader = await backend.readerSentenceClick(sentenceIdx);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerNextSentence: async () => {
    try {
      const reader = await backend.readerNextSentence();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerPrevSentence: async () => {
    try {
      const reader = await backend.readerPrevSentence();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerToggleTextOnly: async () => {
    try {
      const reader = await backend.readerToggleTextOnly();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerApplySettings: async (patch) => {
    const previous = get().reader;
    if (previous) {
      set({
        reader: {
          ...previous,
          settings: {
            ...previous.settings,
            ...patch
          }
        }
      });
    }
    try {
      const reader = await backend.readerApplySettings(patch);
      set({ reader });
    } catch (error) {
      if (previous) {
        set({ reader: previous });
      }
      set({ error: toBridgeError(error).message });
    }
  },

  readerSearchSetQuery: async (query) => {
    try {
      const reader = await backend.readerSearchSetQuery(query);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSearchNext: async () => {
    try {
      const reader = await backend.readerSearchNext();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSearchPrev: async () => {
    try {
      const reader = await backend.readerSearchPrev();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsPlay: async () => {
    try {
      const reader = await backend.readerTtsPlay();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsPause: async () => {
    try {
      const reader = await backend.readerTtsPause();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsTogglePlayPause: async () => {
    try {
      const reader = await backend.readerTtsTogglePlayPause();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsPlayFromPageStart: async () => {
    try {
      const reader = await backend.readerTtsPlayFromPageStart();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsPlayFromHighlight: async () => {
    try {
      const reader = await backend.readerTtsPlayFromHighlight();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsSeekNext: async () => {
    try {
      const reader = await backend.readerTtsSeekNext();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsSeekPrev: async () => {
    try {
      const reader = await backend.readerTtsSeekPrev();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerTtsRepeatSentence: async () => {
    try {
      const reader = await backend.readerTtsRepeatSentence();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  toggleSettingsPanel: async () => {
    const previousSession = get().session;
    const previousReader = get().reader;
    if (previousSession) {
      const panels = togglePanels(previousSession.panels, "show_settings");
      set({
        session: { ...previousSession, panels },
        reader: previousReader ? { ...previousReader, panels } : previousReader
      });
    }
    try {
      const session = await backend.panelToggleSettings();
      set({ session });
    } catch (error) {
      set({ session: previousSession, reader: previousReader });
      set({ error: toBridgeError(error).message });
    }
  },

  toggleStatsPanel: async () => {
    const previousSession = get().session;
    const previousReader = get().reader;
    if (previousSession) {
      const panels = togglePanels(previousSession.panels, "show_stats");
      set({
        session: { ...previousSession, panels },
        reader: previousReader ? { ...previousReader, panels } : previousReader
      });
    }
    try {
      const session = await backend.panelToggleStats();
      set({ session });
    } catch (error) {
      set({ session: previousSession, reader: previousReader });
      set({ error: toBridgeError(error).message });
    }
  },

  toggleTtsPanel: async () => {
    const previousSession = get().session;
    const previousReader = get().reader;
    if (previousSession) {
      const panels = togglePanels(previousSession.panels, "show_tts");
      set({
        session: { ...previousSession, panels },
        reader: previousReader ? { ...previousReader, panels } : previousReader
      });
    }
    try {
      const session = await backend.panelToggleTts();
      set({ session });
    } catch (error) {
      set({ session: previousSession, reader: previousReader });
      set({ error: toBridgeError(error).message });
    }
  },

  loadCalibreBooks: async (forceRefresh) => {
    const startedAt = Date.now();
    set({ loadingCalibre: true, error: null });
    try {
      const calibreBooks = await backend.calibreLoadBooks(forceRefresh);
      set({ calibreBooks });
      finishTelemetry(set, get, "loadCalibreBooks", startedAt, true, null);
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({
        error: bridgeError.message,
        toast: buildToast("error", bridgeError.message)
      });
      finishTelemetry(set, get, "loadCalibreBooks", startedAt, false, bridgeError.message);
    } finally {
      set({ loadingCalibre: false });
    }
  },

  openCalibreBook: async (bookId) => {
    await withBusy(set, get, "openCalibreBook", async () => {
      try {
        const result = await backend.calibreOpenBook(bookId);
        const recents = await backend.recentList();
        set({
          session: result.session,
          reader: result.reader,
          recents,
          toast: buildToast("success", "Book opened from calibre")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  setRuntimeLogLevel: async (level) => {
    await withBusy(set, get, "setRuntimeLogLevel", async () => {
      try {
        const normalized = await backend.loggingSetLevel(level);
        set({
          runtimeLogLevel: normalized,
          toast: buildToast("success", `Log level set to ${normalized}`)
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
        throw bridgeError;
      }
    });
  },

  clearError: () => set({ error: null }),
  dismissToast: () => set({ toast: null }),
  clearTelemetry: () => set({ telemetry: [] })
  });
}

export const useAppStore = create<AppStore>(createAppStoreState(backendApi));
