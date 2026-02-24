import type { AppStore } from "../appStore";
import { buildToast, toBridgeError, togglePanels, withBusy } from "./shared";
import type { SliceContext } from "./types";

export function createSettingsSliceActions({ set, get, backend }: SliceContext): Pick<
  AppStore,
  | "readerApplySettings"
  | "toggleSettingsPanel"
  | "toggleStatsPanel"
  | "toggleTtsPanel"
  | "setRuntimeLogLevel"
> {
  return {
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
    }
  };
}
