import type { AppStore } from "../appStore";
import { ensureJobSubscriptions } from "./jobsSlice";
import { buildToast, finishTelemetry, toBridgeError, toMessage, withBusy } from "./shared";
import type { SliceContext } from "./types";

export function createSessionSliceActions({ set, get, backend }: SliceContext): Pick<
  AppStore,
  | "appSafeQuit"
  | "bootstrap"
  | "refreshRecents"
  | "openSourcePath"
  | "openClipboardText"
  | "deleteRecent"
  | "returnToStarter"
  | "closeReaderSession"
> {
  return {
    appSafeQuit: async () => {
      try {
        await backend.appSafeQuit();
        const session = get().session;
        if (session) {
          set({
            session: {
              ...session,
              mode: "starter",
              active_source_path: null,
              open_in_flight: false
            },
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
        await ensureJobSubscriptions({ set, get, backend });

        const [bootstrapState, session, recents] = await Promise.all([
          backend.sessionGetBootstrap(),
          backend.sessionGetState(),
          backend.recentList()
        ]);

        let reader = null;
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
    }
  };
}
