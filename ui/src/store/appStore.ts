import { create } from "zustand";

import { backendApi } from "../api/tauri";
import type {
  BootstrapState,
  BridgeError,
  CalibreBook,
  ReaderSettingsPatch,
  ReaderSnapshot,
  RecentBook,
  SessionState
} from "../types";

type ToastSeverity = "info" | "success" | "error";

export interface ToastMessage {
  id: number;
  severity: ToastSeverity;
  message: string;
}

interface AppStore {
  bootstrapState: BootstrapState | null;
  session: SessionState | null;
  reader: ReaderSnapshot | null;
  recents: RecentBook[];
  calibreBooks: CalibreBook[];
  loadingBootstrap: boolean;
  loadingRecents: boolean;
  loadingCalibre: boolean;
  busy: boolean;
  error: string | null;
  toast: ToastMessage | null;
  sourceOpenSubscribed: boolean;
  calibreSubscribed: boolean;
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
  toggleSettingsPanel: () => Promise<void>;
  toggleStatsPanel: () => Promise<void>;
  toggleTtsPanel: () => Promise<void>;
  loadCalibreBooks: (forceRefresh?: boolean) => Promise<void>;
  openCalibreBook: (bookId: number) => Promise<void>;
  clearError: () => void;
  dismissToast: () => void;
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

async function withBusy(
  set: (partial: Partial<AppStore>) => void,
  fn: () => Promise<void>
): Promise<void> {
  set({ busy: true, error: null });
  try {
    await fn();
  } finally {
    set({ busy: false });
  }
}

export const useAppStore = create<AppStore>((set, get) => ({
  bootstrapState: null,
  session: null,
  reader: null,
  recents: [],
  calibreBooks: [],
  loadingBootstrap: false,
  loadingRecents: false,
  loadingCalibre: false,
  busy: false,
  error: null,
  toast: null,
  sourceOpenSubscribed: false,
  calibreSubscribed: false,

  bootstrap: async () => {
    if (get().loadingBootstrap) {
      return;
    }
    set({ loadingBootstrap: true, error: null });
    try {
      if (!get().sourceOpenSubscribed) {
        await backendApi.onSourceOpen((event) => {
          if (event.phase === "failed") {
            set({
              toast: buildToast("error", event.message ?? "Source open failed")
            });
          }
        });
        set({ sourceOpenSubscribed: true });
      }
      if (!get().calibreSubscribed) {
        await backendApi.onCalibreLoad((event) => {
          if (event.phase === "failed") {
            set({
              toast: buildToast("error", event.message ?? "Calibre load failed")
            });
          }
        });
        set({ calibreSubscribed: true });
      }

      const [bootstrapState, session, recents] = await Promise.all([
        backendApi.sessionGetBootstrap(),
        backendApi.sessionGetState(),
        backendApi.recentList()
      ]);

      let reader: ReaderSnapshot | null = null;
      if (session.mode === "reader") {
        try {
          reader = await backendApi.readerGetSnapshot();
        } catch {
          reader = null;
        }
      }

      set({
        bootstrapState,
        session,
        recents,
        reader
      });
    } catch (error) {
      set({ error: toMessage(error) });
    } finally {
      set({ loadingBootstrap: false });
    }
  },

  refreshRecents: async () => {
    set({ loadingRecents: true, error: null });
    try {
      const recents = await backendApi.recentList();
      set({ recents });
    } catch (error) {
      set({ error: toMessage(error) });
    } finally {
      set({ loadingRecents: false });
    }
  },

  openSourcePath: async (path) => {
    await withBusy(set, async () => {
      try {
        const result = await backendApi.sourceOpenPath(path);
        const recents = await backendApi.recentList();
        set({
          session: result.session,
          reader: result.reader,
          recents,
          toast: buildToast("success", "Source opened")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
      }
    });
  },

  openClipboardText: async (text) => {
    await withBusy(set, async () => {
      try {
        const result = await backendApi.sourceOpenClipboardText(text);
        const recents = await backendApi.recentList();
        set({
          session: result.session,
          reader: result.reader,
          recents,
          toast: buildToast("success", "Clipboard text opened")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
      }
    });
  },

  deleteRecent: async (path) => {
    await withBusy(set, async () => {
      try {
        await backendApi.recentDelete(path);
        const recents = await backendApi.recentList();
        set({
          recents,
          toast: buildToast("success", "Recent entry deleted")
        });
      } catch (error) {
        const bridgeError = toBridgeError(error);
        set({
          error: bridgeError.message,
          toast: buildToast("error", bridgeError.message)
        });
      }
    });
  },

  returnToStarter: async () => {
    await withBusy(set, async () => {
      try {
        const session = await backendApi.sessionReturnToStarter();
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
      }
    });
  },

  closeReaderSession: async () => {
    await withBusy(set, async () => {
      try {
        const session = await backendApi.readerCloseSession();
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
      }
    });
  },

  refreshReaderSnapshot: async () => {
    const session = get().session;
    if (!session || session.mode !== "reader") {
      return;
    }
    try {
      const reader = await backendApi.readerGetSnapshot();
      set({ reader });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({ error: bridgeError.message });
    }
  },

  readerNextPage: async () => {
    try {
      const reader = await backendApi.readerNextPage();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerPrevPage: async () => {
    try {
      const reader = await backendApi.readerPrevPage();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSetPage: async (page) => {
    try {
      const reader = await backendApi.readerSetPage(page);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSentenceClick: async (sentenceIdx) => {
    try {
      const reader = await backendApi.readerSentenceClick(sentenceIdx);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerNextSentence: async () => {
    try {
      const reader = await backendApi.readerNextSentence();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerPrevSentence: async () => {
    try {
      const reader = await backendApi.readerPrevSentence();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerToggleTextOnly: async () => {
    try {
      const reader = await backendApi.readerToggleTextOnly();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerApplySettings: async (patch) => {
    try {
      const reader = await backendApi.readerApplySettings(patch);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSearchSetQuery: async (query) => {
    try {
      const reader = await backendApi.readerSearchSetQuery(query);
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSearchNext: async () => {
    try {
      const reader = await backendApi.readerSearchNext();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  readerSearchPrev: async () => {
    try {
      const reader = await backendApi.readerSearchPrev();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  toggleSettingsPanel: async () => {
    try {
      const session = await backendApi.panelToggleSettings();
      set({ session });
      await get().refreshReaderSnapshot();
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  toggleStatsPanel: async () => {
    try {
      const session = await backendApi.panelToggleStats();
      set({ session });
      await get().refreshReaderSnapshot();
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  toggleTtsPanel: async () => {
    try {
      const session = await backendApi.panelToggleTts();
      set({ session });
      await get().refreshReaderSnapshot();
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  },

  loadCalibreBooks: async (forceRefresh) => {
    set({ loadingCalibre: true, error: null });
    try {
      const calibreBooks = await backendApi.calibreLoadBooks(forceRefresh);
      set({ calibreBooks });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({
        error: bridgeError.message,
        toast: buildToast("error", bridgeError.message)
      });
    } finally {
      set({ loadingCalibre: false });
    }
  },

  openCalibreBook: async (bookId) => {
    await withBusy(set, async () => {
      try {
        const result = await backendApi.calibreOpenBook(bookId);
        const recents = await backendApi.recentList();
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
      }
    });
  },

  clearError: () => set({ error: null }),
  dismissToast: () => set({ toast: null })
}));
