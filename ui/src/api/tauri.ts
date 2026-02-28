import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import type {
  BootstrapState,
  BridgeError,
  CalibreBook,
  CalibreLoadEvent,
  LogLevelEvent,
  OpenSourceResult,
  PdfTranscriptionEvent,
  ReaderStateEvent,
  ReaderSettingsPatch,
  ReaderSnapshot,
  RecentBook,
  SessionStateEvent,
  SessionState,
  SourceOpenEvent,
  TtsStateEvent
} from "../types";

const MAX_RECENT_LIMIT = 512;
const DEFAULT_RECENT_LIMIT = 64;

const isTauriRuntime = (): boolean => {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
};

function normalizeRecentLimit(limit?: number): number {
  const candidate = Number.isFinite(limit) ? Number(limit) : DEFAULT_RECENT_LIMIT;
  return Math.min(MAX_RECENT_LIMIT, Math.max(1, Math.floor(candidate)));
}

function bridgeErrorFromUnknown(error: unknown): BridgeError {
  if (
    typeof error === "object" &&
    error !== null &&
    "code" in error &&
    "message" in error &&
    typeof (error as { code: unknown }).code === "string" &&
    typeof (error as { message: unknown }).message === "string"
  ) {
    const structured = error as BridgeError;
    return {
      code: structured.code,
      message: structured.message
    };
  }

  if (error instanceof Error) {
    return {
      code: "unknown_error",
      message: error.message
    };
  }

  return {
    code: "unknown_error",
    message: String(error)
  };
}

async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw bridgeErrorFromUnknown(error);
  }
}

type MockBackendState = {
  bootstrap: BootstrapState;
  session: SessionState;
  recents: RecentBook[];
  calibreBooks: CalibreBook[];
  reader: ReaderSnapshot | null;
  logLevel: string;
};

const MOCK_PAGES: Array<{ text: string; sentences: string[] }> = [
  {
    text: "This is the mock reader content on page one.",
    sentences: ["This is the mock reader content on page one."]
  },
  {
    text: "This is the mock reader content on page two.",
    sentences: ["This is the mock reader content on page two."]
  },
  {
    text: "This is the mock reader content on page three.",
    sentences: ["This is the mock reader content on page three."]
  }
];

const mockReaderSnapshot = (): ReaderSnapshot => ({
  source_path: ".cache/clipboard/mock.txt",
  source_name: "mock.txt",
  current_page: 0,
  total_pages: MOCK_PAGES.length,
  text_only_mode: false,
  images: [],
  page_text: MOCK_PAGES[0].text,
  sentences: MOCK_PAGES[0].sentences,
  highlighted_sentence_idx: 0,
  search_query: "",
  search_matches: [],
  selected_search_match: null,
  settings: {
    theme: "day",
    font_family: "lexend",
    font_weight: "bold",
    day_highlight: { r: 0.2, g: 0.4, b: 0.7, a: 0.15 },
    night_highlight: { r: 0.8, g: 0.8, b: 0.5, a: 0.2 },
    font_size: 22,
    line_spacing: 1.2,
    word_spacing: 0,
    letter_spacing: 0,
    margin_horizontal: 100,
    margin_vertical: 12,
    lines_per_page: 700,
    pause_after_sentence: 0.06,
    auto_scroll_tts: true,
    center_spoken_sentence: true,
    time_remaining_display: "adaptive",
    tts_speed: 2.5,
    tts_volume: 1.0
  },
  stats: {
    page_index: 1,
    total_pages: MOCK_PAGES.length,
    tts_progress_pct: 100,
    global_progress_pct: 33.333,
    page_time_remaining_secs: 0,
    book_time_remaining_secs: 0,
    page_word_count: MOCK_PAGES[0].text.split(/\s+/).length,
    page_sentence_count: 1,
    page_start_percent: 0,
    page_end_percent: 33.333,
    words_read_up_to_page_start: 0,
    sentences_read_up_to_page_start: 0,
    words_read_up_to_page_end: MOCK_PAGES[0].text.split(/\s+/).length,
    sentences_read_up_to_page_end: 1,
    words_read_up_to_current_position: MOCK_PAGES[0].text.split(/\s+/).length,
    sentences_read_up_to_current_position: 1
  },
  tts: {
    state: "idle",
    current_sentence_idx: 0,
    sentence_count: 1,
    can_seek_prev: false,
    can_seek_next: false,
    progress_pct: 100
  },
  panels: {
    show_settings: true,
    show_stats: false,
    show_tts: true
  }
});

const mockState: MockBackendState = {
  bootstrap: {
    app_name: "LanternLeaf",
    mode: "mock",
    config: {
      theme: "day",
      font_family: "lexend",
      font_weight: "bold",
      day_highlight: { r: 0.2, g: 0.4, b: 0.7, a: 0.15 },
      night_highlight: { r: 0.8, g: 0.8, b: 0.5, a: 0.2 },
      log_level: "debug",
      default_font_size: 22,
      default_lines_per_page: 700,
      default_tts_speed: 2.5,
      default_pause_after_sentence: 0.06,
      key_toggle_play_pause: "space",
      key_next_sentence: "f",
      key_prev_sentence: "s",
      key_repeat_sentence: "r",
      key_toggle_search: "ctrl+f",
      key_safe_quit: "q",
      key_toggle_settings: "ctrl+t",
      key_toggle_stats: "ctrl+g",
      key_toggle_tts: "ctrl+y"
    }
  },
  session: {
    mode: "starter",
    active_source_path: null,
    open_in_flight: false,
    panels: {
      show_settings: true,
      show_stats: false,
      show_tts: true
    }
  },
  recents: [],
  calibreBooks: [],
  reader: null,
  logLevel: "debug"
};

function ensureMockReader(): ReaderSnapshot {
  if (!mockState.reader) {
    mockState.reader = mockReaderSnapshot();
  }
  return mockState.reader;
}

function applyMockPage(reader: ReaderSnapshot, page: number): void {
  const clampedPage = Math.max(0, Math.min(MOCK_PAGES.length - 1, Math.floor(page)));
  const pageData = MOCK_PAGES[clampedPage];
  const totalWords = MOCK_PAGES.reduce((sum, item) => sum + item.text.split(/\s+/).length, 0);
  const wordsBeforePage = MOCK_PAGES.slice(0, clampedPage).reduce(
    (sum, item) => sum + item.text.split(/\s+/).length,
    0
  );
  const wordsOnPage = pageData.text.split(/\s+/).length;

  reader.current_page = clampedPage;
  reader.total_pages = MOCK_PAGES.length;
  reader.page_text = pageData.text;
  reader.sentences = pageData.sentences;
  reader.highlighted_sentence_idx = 0;
  reader.tts.current_sentence_idx = 0;
  reader.tts.sentence_count = pageData.sentences.length;
  reader.stats.page_index = clampedPage + 1;
  reader.stats.total_pages = MOCK_PAGES.length;
  reader.stats.page_word_count = wordsOnPage;
  reader.stats.page_sentence_count = pageData.sentences.length;
  reader.stats.tts_progress_pct = 100;
  reader.stats.page_start_percent = Number(((wordsBeforePage / totalWords) * 100).toFixed(3));
  reader.stats.page_end_percent = Number(
    (((wordsBeforePage + wordsOnPage) / totalWords) * 100).toFixed(3)
  );
  reader.stats.words_read_up_to_page_start = wordsBeforePage;
  reader.stats.sentences_read_up_to_page_start = clampedPage;
  reader.stats.words_read_up_to_page_end = wordsBeforePage + wordsOnPage;
  reader.stats.sentences_read_up_to_page_end = clampedPage + pageData.sentences.length;
  reader.stats.words_read_up_to_current_position = wordsBeforePage + wordsOnPage;
  reader.stats.sentences_read_up_to_current_position = clampedPage + pageData.sentences.length;
  reader.stats.global_progress_pct = Number(
    ((reader.stats.words_read_up_to_current_position / totalWords) * 100).toFixed(3)
  );
}

async function mockOpenWithPath(path: string): Promise<OpenSourceResult> {
  const trimmed = path.trim();
  if (!trimmed) {
    throw {
      code: "invalid_input",
      message: "Path cannot be empty"
    } satisfies BridgeError;
  }

  const reader = ensureMockReader();
  reader.source_path = trimmed;
  reader.source_name = trimmed.split(/[\\/]/).pop() ?? trimmed;
  mockState.session.mode = "reader";
  mockState.session.active_source_path = trimmed;
  mockState.reader = reader;
  return {
    session: structuredClone(mockState.session),
    reader: structuredClone(reader)
  };
}

async function mockSessionGetBootstrap(): Promise<BootstrapState> {
  return structuredClone(mockState.bootstrap);
}

async function mockSessionToggleTheme(): Promise<BootstrapState> {
  const nextTheme = mockState.bootstrap.config.theme === "night" ? "day" : "night";
  mockState.bootstrap.config.theme = nextTheme;
  if (mockState.reader) {
    mockState.reader.settings.theme = nextTheme;
  }
  return structuredClone(mockState.bootstrap);
}

async function mockSessionGetState(): Promise<SessionState> {
  return structuredClone(mockState.session);
}

async function mockSessionReturnToStarter(): Promise<SessionState> {
  mockState.session.mode = "starter";
  mockState.session.active_source_path = null;
  mockState.reader = null;
  return structuredClone(mockState.session);
}

async function mockAppSafeQuit(): Promise<void> {
  await mockSessionReturnToStarter();
}

async function mockRecentList(limit?: number): Promise<RecentBook[]> {
  return structuredClone(mockState.recents.slice(0, normalizeRecentLimit(limit)));
}

async function mockRecentDelete(path: string): Promise<void> {
  mockState.recents = mockState.recents.filter((book) => book.source_path !== path);
}

async function mockSourceOpenPath(path: string): Promise<OpenSourceResult> {
  return mockOpenWithPath(path);
}

async function mockSourceOpenClipboardText(text: string): Promise<OpenSourceResult> {
  const trimmed = text.trim();
  if (!trimmed) {
    throw {
      code: "invalid_input",
      message: "Clipboard text is empty"
    } satisfies BridgeError;
  }
  return mockOpenWithPath(".cache/clipboard/mock.txt");
}

async function mockReaderGetSnapshot(): Promise<ReaderSnapshot> {
  return structuredClone(ensureMockReader());
}

async function mockReaderNextPage(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  applyMockPage(reader, reader.current_page + 1);
  return structuredClone(reader);
}

async function mockReaderPrevPage(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  applyMockPage(reader, reader.current_page - 1);
  return structuredClone(reader);
}

async function mockReaderSetPage(page: number): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  applyMockPage(reader, page);
  return structuredClone(reader);
}

async function mockReaderApplySettings(patch: ReaderSettingsPatch): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  reader.settings = {
    ...reader.settings,
    ...patch
  };
  return structuredClone(reader);
}

async function mockReaderNextSentence(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  const count = reader.sentences.length;
  if (count === 0) {
    reader.highlighted_sentence_idx = null;
    reader.tts.current_sentence_idx = null;
    return structuredClone(reader);
  }
  const current = reader.highlighted_sentence_idx ?? 0;
  reader.highlighted_sentence_idx = Math.min(count - 1, current + 1);
  reader.tts.current_sentence_idx = reader.highlighted_sentence_idx;
  return structuredClone(reader);
}

async function mockReaderPrevSentence(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  const count = reader.sentences.length;
  if (count === 0) {
    reader.highlighted_sentence_idx = null;
    reader.tts.current_sentence_idx = null;
    return structuredClone(reader);
  }
  const current = reader.highlighted_sentence_idx ?? 0;
  reader.highlighted_sentence_idx = Math.max(0, current - 1);
  reader.tts.current_sentence_idx = reader.highlighted_sentence_idx;
  return structuredClone(reader);
}

async function mockReaderTtsPlay(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  if (reader.highlighted_sentence_idx === null) {
    reader.highlighted_sentence_idx = 0;
  }
  reader.tts.current_sentence_idx = reader.highlighted_sentence_idx;
  reader.tts.state = "playing";
  return structuredClone(reader);
}

async function mockReaderTtsPause(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  if (reader.tts.state === "playing") {
    reader.tts.state = "paused";
  }
  return structuredClone(reader);
}

async function mockReaderTtsTogglePlayPause(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  if (reader.tts.state === "playing") {
    reader.tts.state = "paused";
  } else {
    reader.tts.state = "playing";
  }
  return structuredClone(reader);
}

async function mockReaderTtsPlayFromPageStart(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  reader.highlighted_sentence_idx = 0;
  reader.tts.current_sentence_idx = 0;
  reader.tts.state = "playing";
  return structuredClone(reader);
}

async function mockReaderTtsPlayFromHighlight(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  if (reader.highlighted_sentence_idx === null) {
    reader.highlighted_sentence_idx = 0;
  }
  reader.tts.current_sentence_idx = reader.highlighted_sentence_idx;
  reader.tts.state = "playing";
  return structuredClone(reader);
}

async function mockReaderTtsSeekNext(): Promise<ReaderSnapshot> {
  return mockReaderNextSentence();
}

async function mockReaderTtsSeekPrev(): Promise<ReaderSnapshot> {
  return mockReaderPrevSentence();
}

async function mockReaderTtsRepeatSentence(): Promise<ReaderSnapshot> {
  const reader = ensureMockReader();
  if (reader.highlighted_sentence_idx === null) {
    reader.highlighted_sentence_idx = 0;
  }
  reader.tts.current_sentence_idx = reader.highlighted_sentence_idx;
  return structuredClone(reader);
}

async function mockReaderTtsPrecomputePage(): Promise<ReaderSnapshot> {
  return mockReaderGetSnapshot();
}
async function mockCalibreLoadBooks(): Promise<CalibreBook[]> {
  return structuredClone(mockState.calibreBooks);
}

async function mockCalibreOpenBook(): Promise<OpenSourceResult> {
  return mockOpenWithPath(".cache/calibre-downloads/mock.epub");
}

async function mockPanelToggleSettings(): Promise<SessionState> {
  mockState.session.panels.show_settings = !mockState.session.panels.show_settings;
  if (mockState.session.panels.show_settings) {
    mockState.session.panels.show_stats = false;
  }
  return structuredClone(mockState.session);
}

async function mockPanelToggleStats(): Promise<SessionState> {
  mockState.session.panels.show_stats = !mockState.session.panels.show_stats;
  if (mockState.session.panels.show_stats) {
    mockState.session.panels.show_settings = false;
  }
  return structuredClone(mockState.session);
}

async function mockPanelToggleTts(): Promise<SessionState> {
  mockState.session.panels.show_tts = !mockState.session.panels.show_tts;
  return structuredClone(mockState.session);
}

async function mockOnSourceOpen(handler: (event: SourceOpenEvent) => void): Promise<UnlistenFn> {
  queueMicrotask(() =>
    handler({
      request_id: 0,
      phase: "ready",
      source_path: null,
      message: "Using mock backend adapter"
    })
  );
  return () => Promise.resolve();
}

async function mockOnCalibreLoad(handler: (event: CalibreLoadEvent) => void): Promise<UnlistenFn> {
  queueMicrotask(() =>
    handler({
      request_id: 0,
      phase: "ready",
      count: 0,
      message: "Using mock backend adapter"
    })
  );
  return () => Promise.resolve();
}

async function mockOnSessionState(handler: (event: SessionStateEvent) => void): Promise<UnlistenFn> {
  queueMicrotask(() =>
    handler({
      request_id: 0,
      action: "ready",
      session: structuredClone(mockState.session)
    })
  );
  return () => Promise.resolve();
}

async function mockOnReaderState(handler: (event: ReaderStateEvent) => void): Promise<UnlistenFn> {
  queueMicrotask(() => {
    if (!mockState.reader) {
      return;
    }
    handler({
      request_id: 0,
      action: "ready",
      reader: structuredClone(mockState.reader)
    });
  });
  return () => Promise.resolve();
}

async function mockOnTtsState(handler: (event: TtsStateEvent) => void): Promise<UnlistenFn> {
  queueMicrotask(() => {
    const reader = ensureMockReader();
    handler({
      request_id: 0,
      action: "ready",
      tts: structuredClone(reader.tts)
    });
  });
  return () => Promise.resolve();
}

async function mockOnPdfTranscription(
  handler: (event: PdfTranscriptionEvent) => void
): Promise<UnlistenFn> {
  queueMicrotask(() =>
    handler({
      request_id: 0,
      phase: "ready",
      source_path: "",
      message: "Using mock backend adapter"
    })
  );
  return () => Promise.resolve();
}

async function mockOnLogLevel(handler: (event: LogLevelEvent) => void): Promise<UnlistenFn> {
  queueMicrotask(() =>
    handler({
      request_id: 0,
      level: mockState.logLevel
    })
  );
  return () => Promise.resolve();
}

async function mockLoggingSetLevel(level: string): Promise<string> {
  const normalized = level.trim().toLowerCase();
  if (!["trace", "debug", "info", "warn", "error"].includes(normalized)) {
    throw {
      code: "invalid_input",
      message: `Unsupported log level '${level}'`
    } satisfies BridgeError;
  }
  mockState.logLevel = normalized;
  return mockState.logLevel;
}

export interface BackendApi {
  appSafeQuit: () => Promise<void>;
  sessionGetBootstrap: () => Promise<BootstrapState>;
  sessionToggleTheme: () => Promise<BootstrapState>;
  sessionGetState: () => Promise<SessionState>;
  sessionReturnToStarter: () => Promise<SessionState>;
  panelToggleSettings: () => Promise<SessionState>;
  panelToggleStats: () => Promise<SessionState>;
  panelToggleTts: () => Promise<SessionState>;
  recentList: (limit?: number) => Promise<RecentBook[]>;
  recentDelete: (path: string) => Promise<void>;
  sourceOpenPath: (path: string) => Promise<OpenSourceResult>;
  sourceOpenClipboard: () => Promise<OpenSourceResult>;
  sourceOpenClipboardText: (text: string) => Promise<OpenSourceResult>;
  readerGetSnapshot: () => Promise<ReaderSnapshot>;
  readerNextPage: () => Promise<ReaderSnapshot>;
  readerPrevPage: () => Promise<ReaderSnapshot>;
  readerSetPage: (page: number) => Promise<ReaderSnapshot>;
  readerSentenceClick: (sentenceIdx: number) => Promise<ReaderSnapshot>;
  readerNextSentence: () => Promise<ReaderSnapshot>;
  readerPrevSentence: () => Promise<ReaderSnapshot>;
  readerToggleTextOnly: () => Promise<ReaderSnapshot>;
  readerApplySettings: (patch: ReaderSettingsPatch) => Promise<ReaderSnapshot>;
  readerSearchSetQuery: (query: string) => Promise<ReaderSnapshot>;
  readerSearchNext: () => Promise<ReaderSnapshot>;
  readerSearchPrev: () => Promise<ReaderSnapshot>;
  readerTtsPlay: () => Promise<ReaderSnapshot>;
  readerTtsPause: () => Promise<ReaderSnapshot>;
  readerTtsTogglePlayPause: () => Promise<ReaderSnapshot>;
  readerTtsPlayFromPageStart: () => Promise<ReaderSnapshot>;
  readerTtsPlayFromHighlight: () => Promise<ReaderSnapshot>;
  readerTtsSeekNext: () => Promise<ReaderSnapshot>;
  readerTtsSeekPrev: () => Promise<ReaderSnapshot>;
  readerTtsRepeatSentence: () => Promise<ReaderSnapshot>;
  readerTtsPrecomputePage: () => Promise<ReaderSnapshot>;
  readerCloseSession: () => Promise<SessionState>;
  loggingSetLevel: (level: string) => Promise<string>;
  calibreLoadCachedBooks: () => Promise<CalibreBook[]>;
  calibreLoadBooks: (forceRefresh?: boolean) => Promise<CalibreBook[]>;
  calibreOpenBook: (bookId: number) => Promise<OpenSourceResult>;
  onSourceOpen: (handler: (event: SourceOpenEvent) => void) => Promise<UnlistenFn>;
  onCalibreLoad: (handler: (event: CalibreLoadEvent) => void) => Promise<UnlistenFn>;
  onSessionState: (handler: (event: SessionStateEvent) => void) => Promise<UnlistenFn>;
  onReaderState: (handler: (event: ReaderStateEvent) => void) => Promise<UnlistenFn>;
  onTtsState: (handler: (event: TtsStateEvent) => void) => Promise<UnlistenFn>;
  onPdfTranscription: (handler: (event: PdfTranscriptionEvent) => void) => Promise<UnlistenFn>;
  onLogLevel: (handler: (event: LogLevelEvent) => void) => Promise<UnlistenFn>;
}

function createTauriBackendApi(): BackendApi {
  return {
    appSafeQuit: () => invokeCommand<void>("app_safe_quit"),
    sessionGetBootstrap: () => invokeCommand<BootstrapState>("session_get_bootstrap"),
    sessionToggleTheme: () => invokeCommand<BootstrapState>("session_toggle_theme"),
    sessionGetState: () => invokeCommand<SessionState>("session_get_state"),
    sessionReturnToStarter: () => invokeCommand<SessionState>("session_return_to_starter"),
    panelToggleSettings: () => invokeCommand<SessionState>("panel_toggle_settings"),
    panelToggleStats: () => invokeCommand<SessionState>("panel_toggle_stats"),
    panelToggleTts: () => invokeCommand<SessionState>("panel_toggle_tts"),
    recentList: (limit) =>
      invokeCommand<RecentBook[]>("recent_list", { limit: normalizeRecentLimit(limit) }),
    recentDelete: (path) => invokeCommand<void>("recent_delete", { path }),
    sourceOpenPath: (path) => invokeCommand<OpenSourceResult>("source_open_path", { path }),
    sourceOpenClipboard: () => invokeCommand<OpenSourceResult>("source_open_clipboard"),
    sourceOpenClipboardText: (text) =>
      invokeCommand<OpenSourceResult>("source_open_clipboard_text", { text }),
    readerGetSnapshot: () => invokeCommand<ReaderSnapshot>("reader_get_snapshot"),
    readerNextPage: () => invokeCommand<ReaderSnapshot>("reader_next_page"),
    readerPrevPage: () => invokeCommand<ReaderSnapshot>("reader_prev_page"),
    readerSetPage: (page) => invokeCommand<ReaderSnapshot>("reader_set_page", { page }),
    readerSentenceClick: (sentenceIdx) =>
      invokeCommand<ReaderSnapshot>("reader_sentence_click", { sentenceIdx }),
    readerNextSentence: () => invokeCommand<ReaderSnapshot>("reader_next_sentence"),
    readerPrevSentence: () => invokeCommand<ReaderSnapshot>("reader_prev_sentence"),
    readerToggleTextOnly: () => invokeCommand<ReaderSnapshot>("reader_toggle_text_only"),
    readerApplySettings: (patch) => invokeCommand<ReaderSnapshot>("reader_apply_settings", { patch }),
    readerSearchSetQuery: (query) =>
      invokeCommand<ReaderSnapshot>("reader_search_set_query", { query }),
    readerSearchNext: () => invokeCommand<ReaderSnapshot>("reader_search_next"),
    readerSearchPrev: () => invokeCommand<ReaderSnapshot>("reader_search_prev"),
    readerTtsPlay: () => invokeCommand<ReaderSnapshot>("reader_tts_play"),
    readerTtsPause: () => invokeCommand<ReaderSnapshot>("reader_tts_pause"),
    readerTtsTogglePlayPause: () => invokeCommand<ReaderSnapshot>("reader_tts_toggle_play_pause"),
    readerTtsPlayFromPageStart: () => invokeCommand<ReaderSnapshot>("reader_tts_play_from_page_start"),
    readerTtsPlayFromHighlight: () => invokeCommand<ReaderSnapshot>("reader_tts_play_from_highlight"),
    readerTtsSeekNext: () => invokeCommand<ReaderSnapshot>("reader_tts_seek_next"),
    readerTtsSeekPrev: () => invokeCommand<ReaderSnapshot>("reader_tts_seek_prev"),
    readerTtsRepeatSentence: () => invokeCommand<ReaderSnapshot>("reader_tts_repeat_sentence"),
    readerTtsPrecomputePage: () => invokeCommand<ReaderSnapshot>("reader_tts_precompute_page"),
    readerCloseSession: () => invokeCommand<SessionState>("reader_close_session"),
    loggingSetLevel: (level) => invokeCommand<string>("logging_set_level", { level }),
    calibreLoadCachedBooks: () => invokeCommand<CalibreBook[]>("calibre_load_cached_books"),
    calibreLoadBooks: (forceRefresh) =>
      invokeCommand<CalibreBook[]>("calibre_load_books", { forceRefresh }),
    calibreOpenBook: (bookId) => invokeCommand<OpenSourceResult>("calibre_open_book", { bookId }),
    onSourceOpen: async (handler) => {
      return listen<SourceOpenEvent>("source-open", (event) => handler(event.payload));
    },
    onCalibreLoad: async (handler) => {
      return listen<CalibreLoadEvent>("calibre-load", (event) => handler(event.payload));
    },
    onSessionState: async (handler) => {
      return listen<SessionStateEvent>("session-state", (event) => handler(event.payload));
    },
    onReaderState: async (handler) => {
      return listen<ReaderStateEvent>("reader-state", (event) => handler(event.payload));
    },
    onTtsState: async (handler) => {
      return listen<TtsStateEvent>("tts-state", (event) => handler(event.payload));
    },
    onPdfTranscription: async (handler) => {
      return listen<PdfTranscriptionEvent>("pdf-transcription", (event) => handler(event.payload));
    },
    onLogLevel: async (handler) => {
      return listen<LogLevelEvent>("log-level", (event) => handler(event.payload));
    }
  };
}

function createMockBackendApi(): BackendApi {
  return {
    appSafeQuit: mockAppSafeQuit,
    sessionGetBootstrap: mockSessionGetBootstrap,
    sessionToggleTheme: mockSessionToggleTheme,
    sessionGetState: mockSessionGetState,
    sessionReturnToStarter: mockSessionReturnToStarter,
    panelToggleSettings: mockPanelToggleSettings,
    panelToggleStats: mockPanelToggleStats,
    panelToggleTts: mockPanelToggleTts,
    recentList: mockRecentList,
    recentDelete: mockRecentDelete,
    sourceOpenPath: mockSourceOpenPath,
    sourceOpenClipboard: () => mockSourceOpenClipboardText(""),
    sourceOpenClipboardText: mockSourceOpenClipboardText,
    readerGetSnapshot: mockReaderGetSnapshot,
    readerNextPage: mockReaderNextPage,
    readerPrevPage: mockReaderPrevPage,
    readerSetPage: mockReaderSetPage,
    readerSentenceClick: mockReaderGetSnapshot,
    readerNextSentence: mockReaderNextSentence,
    readerPrevSentence: mockReaderPrevSentence,
    readerToggleTextOnly: mockReaderGetSnapshot,
    readerApplySettings: mockReaderApplySettings,
    readerSearchSetQuery: mockReaderGetSnapshot,
    readerSearchNext: mockReaderGetSnapshot,
    readerSearchPrev: mockReaderGetSnapshot,
    readerTtsPlay: mockReaderTtsPlay,
    readerTtsPause: mockReaderTtsPause,
    readerTtsTogglePlayPause: mockReaderTtsTogglePlayPause,
    readerTtsPlayFromPageStart: mockReaderTtsPlayFromPageStart,
    readerTtsPlayFromHighlight: mockReaderTtsPlayFromHighlight,
    readerTtsSeekNext: mockReaderTtsSeekNext,
    readerTtsSeekPrev: mockReaderTtsSeekPrev,
    readerTtsRepeatSentence: mockReaderTtsRepeatSentence,
    readerTtsPrecomputePage: mockReaderTtsPrecomputePage,
    readerCloseSession: mockSessionReturnToStarter,
    loggingSetLevel: mockLoggingSetLevel,
    calibreLoadCachedBooks: mockCalibreLoadBooks,
    calibreLoadBooks: mockCalibreLoadBooks,
    calibreOpenBook: mockCalibreOpenBook,
    onSourceOpen: mockOnSourceOpen,
    onCalibreLoad: mockOnCalibreLoad,
    onSessionState: mockOnSessionState,
    onReaderState: mockOnReaderState,
    onTtsState: mockOnTtsState,
    onPdfTranscription: mockOnPdfTranscription,
    onLogLevel: mockOnLogLevel
  };
}

export const backendApi: BackendApi = isTauriRuntime()
  ? createTauriBackendApi()
  : createMockBackendApi();
