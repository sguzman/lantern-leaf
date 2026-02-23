import { create } from "zustand";

import { backendApi } from "../api/tauri";
import type { BootstrapState, BridgeError, RecentBook, SessionState, SourceOpenEvent } from "../types";

type ToastSeverity = "info" | "success" | "error";

export interface ToastMessage {
  id: number;
  severity: ToastSeverity;
  message: string;
}

interface AppStore {
  bootstrapState: BootstrapState | null;
  session: SessionState | null;
  recents: RecentBook[];
  loadingBootstrap: boolean;
  loadingRecents: boolean;
  busy: boolean;
  error: string | null;
  toast: ToastMessage | null;
  sourceOpenSubscribed: boolean;
  bootstrap: () => Promise<void>;
  refreshRecents: () => Promise<void>;
  openSourcePath: (path: string) => Promise<void>;
  openClipboardText: (text: string) => Promise<void>;
  deleteRecent: (path: string) => Promise<void>;
  returnToStarter: () => Promise<void>;
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

export const useAppStore = create<AppStore>((set, get) => ({
  bootstrapState: null,
  session: null,
  recents: [],
  loadingBootstrap: false,
  loadingRecents: false,
  busy: false,
  error: null,
  toast: null,
  sourceOpenSubscribed: false,

  bootstrap: async () => {
    if (get().loadingBootstrap) {
      return;
    }

    set({ loadingBootstrap: true, error: null });
    try {
      if (!get().sourceOpenSubscribed) {
        await backendApi.onSourceOpen((event: SourceOpenEvent) => {
          if (event.phase === "failed") {
            set({
              toast: buildToast("error", event.message ?? "Source open failed"),
              busy: false
            });
          }
        });
        set({ sourceOpenSubscribed: true });
      }

      const [bootstrapState, session, recents] = await Promise.all([
        backendApi.sessionGetBootstrap(),
        backendApi.sessionGetState(),
        backendApi.recentList()
      ]);

      set({
        bootstrapState,
        session,
        recents
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

  openSourcePath: async (path: string) => {
    set({ busy: true, error: null });
    try {
      const session = await backendApi.sourceOpenPath(path);
      const recents = await backendApi.recentList();
      set({
        session,
        recents,
        toast: buildToast("success", "Source opened"),
        busy: false
      });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({
        error: bridgeError.message,
        toast: buildToast("error", bridgeError.message),
        busy: false
      });
    }
  },

  openClipboardText: async (text: string) => {
    set({ busy: true, error: null });
    try {
      const session = await backendApi.sourceOpenClipboardText(text);
      const recents = await backendApi.recentList();
      set({
        session,
        recents,
        toast: buildToast("success", "Clipboard text opened"),
        busy: false
      });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({
        error: bridgeError.message,
        toast: buildToast("error", bridgeError.message),
        busy: false
      });
    }
  },

  deleteRecent: async (path: string) => {
    set({ busy: true, error: null });
    try {
      await backendApi.recentDelete(path);
      const recents = await backendApi.recentList();
      set({
        recents,
        toast: buildToast("success", "Recent entry deleted"),
        busy: false
      });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({
        error: bridgeError.message,
        toast: buildToast("error", bridgeError.message),
        busy: false
      });
    }
  },

  returnToStarter: async () => {
    set({ busy: true, error: null });
    try {
      const session = await backendApi.sessionReturnToStarter();
      set({
        session,
        busy: false
      });
    } catch (error) {
      const bridgeError = toBridgeError(error);
      set({
        error: bridgeError.message,
        toast: buildToast("error", bridgeError.message),
        busy: false
      });
    }
  },

  clearError: () => set({ error: null }),
  dismissToast: () => set({ toast: null })
}));
