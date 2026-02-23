import { describe, expect, it } from "vitest";

import type { AppStore } from "../src/store/appStore";
import {
  selectCalibreSlice,
  selectJobsSlice,
  selectNotificationsSlice,
  selectReaderSlice,
  selectSessionSlice,
  selectSettingsSlice,
  selectStatsSlice,
  selectTtsSlice
} from "../src/store/selectors";

function makeState(): AppStore {
  return {
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
    appSafeQuit: async () => {},
    bootstrap: async () => {},
    refreshRecents: async () => {},
    openSourcePath: async () => {},
    openClipboardText: async () => {},
    deleteRecent: async () => {},
    returnToStarter: async () => {},
    closeReaderSession: async () => {},
    refreshReaderSnapshot: async () => {},
    readerNextPage: async () => {},
    readerPrevPage: async () => {},
    readerSetPage: async () => {},
    readerSentenceClick: async () => {},
    readerNextSentence: async () => {},
    readerPrevSentence: async () => {},
    readerToggleTextOnly: async () => {},
    readerApplySettings: async () => {},
    readerSearchSetQuery: async () => {},
    readerSearchNext: async () => {},
    readerSearchPrev: async () => {},
    readerTtsPlay: async () => {},
    readerTtsPause: async () => {},
    readerTtsTogglePlayPause: async () => {},
    readerTtsPlayFromPageStart: async () => {},
    readerTtsPlayFromHighlight: async () => {},
    readerTtsSeekNext: async () => {},
    readerTtsSeekPrev: async () => {},
    readerTtsRepeatSentence: async () => {},
    toggleSettingsPanel: async () => {},
    toggleStatsPanel: async () => {},
    toggleTtsPanel: async () => {},
    loadCalibreBooks: async () => {},
    openCalibreBook: async () => {},
    setRuntimeLogLevel: async () => {},
    clearError: () => {},
    dismissToast: () => {},
    clearTelemetry: () => {}
  };
}

describe("store selectors", () => {
  it("projects stable slices from a base app store", () => {
    const state = makeState();

    expect(selectSessionSlice(state).loadingBootstrap).toBe(false);
    expect(selectReaderSlice(state).reader).toBeNull();
    expect(selectTtsSlice(state).ttsStateEvent).toBeNull();
    expect(selectCalibreSlice(state).calibreBooks).toEqual([]);
    expect(selectSettingsSlice(state).runtimeLogLevel).toBe("info");
    expect(selectStatsSlice(state).stats).toBeNull();
    expect(selectJobsSlice(state).pdfTranscriptionEvent).toBeNull();
    expect(selectNotificationsSlice(state).error).toBeNull();
  });
});
