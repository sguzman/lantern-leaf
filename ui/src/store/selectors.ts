import type {
  CalibreLoadEvent,
  PdfTranscriptionEvent,
  ReaderSettingsView,
  ReaderSnapshot,
  ReaderStats,
  SessionState,
  SourceOpenEvent,
  TtsStateEvent
} from "../types";
import type { AppStore, ToastMessage } from "./appStore";

export interface SessionSlice {
  bootstrapState: AppStore["bootstrapState"];
  session: SessionState | null;
  recents: AppStore["recents"];
  loadingBootstrap: boolean;
  loadingRecents: boolean;
}

export interface ReaderSlice {
  reader: ReaderSnapshot | null;
  busy: boolean;
}

export interface TtsSlice {
  tts: ReaderSnapshot["tts"] | null;
  ttsStateEvent: TtsStateEvent | null;
}

export interface CalibreSlice {
  calibreBooks: AppStore["calibreBooks"];
  calibreLoadEvent: CalibreLoadEvent | null;
  loadingCalibre: boolean;
}

export interface SettingsSlice {
  settings: ReaderSettingsView | null;
  runtimeLogLevel: string;
}

export interface StatsSlice {
  stats: ReaderStats | null;
}

export interface JobsSlice {
  sourceOpenEvent: SourceOpenEvent | null;
  calibreLoadEvent: CalibreLoadEvent | null;
  ttsStateEvent: TtsStateEvent | null;
  pdfTranscriptionEvent: PdfTranscriptionEvent | null;
}

export interface NotificationsSlice {
  error: string | null;
  toast: ToastMessage | null;
}

export const selectSessionSlice = (state: AppStore): SessionSlice => ({
  bootstrapState: state.bootstrapState,
  session: state.session,
  recents: state.recents,
  loadingBootstrap: state.loadingBootstrap,
  loadingRecents: state.loadingRecents
});

export const selectReaderSlice = (state: AppStore): ReaderSlice => ({
  reader: state.reader,
  busy: state.busy
});

export const selectTtsSlice = (state: AppStore): TtsSlice => ({
  tts: state.reader?.tts ?? null,
  ttsStateEvent: state.ttsStateEvent
});

export const selectCalibreSlice = (state: AppStore): CalibreSlice => ({
  calibreBooks: state.calibreBooks,
  calibreLoadEvent: state.calibreLoadEvent,
  loadingCalibre: state.loadingCalibre
});

export const selectSettingsSlice = (state: AppStore): SettingsSlice => ({
  settings: state.reader?.settings ?? null,
  runtimeLogLevel: state.runtimeLogLevel
});

export const selectStatsSlice = (state: AppStore): StatsSlice => ({
  stats: state.reader?.stats ?? null
});

export const selectJobsSlice = (state: AppStore): JobsSlice => ({
  sourceOpenEvent: state.sourceOpenEvent,
  calibreLoadEvent: state.calibreLoadEvent,
  ttsStateEvent: state.ttsStateEvent,
  pdfTranscriptionEvent: state.pdfTranscriptionEvent
});

export const selectNotificationsSlice = (state: AppStore): NotificationsSlice => ({
  error: state.error,
  toast: state.toast
});
