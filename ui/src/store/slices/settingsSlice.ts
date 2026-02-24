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
  | "toggleTheme"
> {
  return {
    readerApplySettings: async (patch) => {
      try {
        const reader = await backend.readerApplySettings(patch);
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

    toggleTheme: async () => {
      await withBusy(set, get, "toggleTheme", async () => {
        const previousBootstrap = get().bootstrapState;
        const previousReader = get().reader;

        if (previousBootstrap) {
          const optimisticTheme =
            previousBootstrap.config.theme === "night" ? "day" : "night";
          set({
            bootstrapState: {
              ...previousBootstrap,
              config: {
                ...previousBootstrap.config,
                theme: optimisticTheme
              }
            },
            reader: previousReader
              ? {
                  ...previousReader,
                  settings: {
                    ...previousReader.settings,
                    theme: optimisticTheme
                  }
                }
              : previousReader
          });
        }

        try {
          const bootstrapState = await backend.sessionToggleTheme();
          const reader = get().reader;
          set({
            bootstrapState,
            reader: reader
              ? {
                  ...reader,
                  settings: {
                    ...reader.settings,
                    theme: bootstrapState.config.theme
                  }
                }
              : reader
          });
        } catch (error) {
          const bridgeError = toBridgeError(error);
          set({
            bootstrapState: previousBootstrap,
            reader: previousReader,
            error: bridgeError.message
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
    }
  };
}
