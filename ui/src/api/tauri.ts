import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import type {
  BootstrapState,
  BridgeError,
  RecentBook,
  SessionState,
  SourceOpenEvent
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
};

const mockState: MockBackendState = {
  bootstrap: {
    app_name: "ebup-viewer",
    mode: "mock",
    config: {
      default_font_size: 22,
      default_lines_per_page: 700,
      default_tts_speed: 2.5,
      default_pause_after_sentence: 0.06
    }
  },
  session: {
    mode: "starter",
    active_source_path: null,
    open_in_flight: false
  },
  recents: []
};

async function mockSessionGetBootstrap(): Promise<BootstrapState> {
  return structuredClone(mockState.bootstrap);
}

async function mockSessionGetState(): Promise<SessionState> {
  return structuredClone(mockState.session);
}

async function mockSessionReturnToStarter(): Promise<SessionState> {
  mockState.session.mode = "starter";
  mockState.session.active_source_path = null;
  return structuredClone(mockState.session);
}

async function mockRecentList(limit?: number): Promise<RecentBook[]> {
  return structuredClone(mockState.recents.slice(0, normalizeRecentLimit(limit)));
}

async function mockRecentDelete(path: string): Promise<void> {
  mockState.recents = mockState.recents.filter((book) => book.source_path !== path);
}

function upsertMockRecent(path: string): void {
  const now = Math.floor(Date.now() / 1000);
  const title = path.split(/[\\/]/).pop() ?? path;
  mockState.recents = [
    {
      source_path: path,
      display_title: title,
      thumbnail_path: null,
      last_opened_unix_secs: now
    },
    ...mockState.recents.filter((book) => book.source_path !== path)
  ];
}

async function mockSourceOpenPath(path: string): Promise<SessionState> {
  const trimmed = path.trim();
  if (!trimmed) {
    throw {
      code: "invalid_input",
      message: "Path cannot be empty"
    } satisfies BridgeError;
  }

  mockState.session.mode = "reader";
  mockState.session.active_source_path = trimmed;
  upsertMockRecent(trimmed);
  return structuredClone(mockState.session);
}

async function mockSourceOpenClipboardText(text: string): Promise<SessionState> {
  const trimmed = text.trim();
  if (!trimmed) {
    throw {
      code: "invalid_input",
      message: "Clipboard text is empty"
    } satisfies BridgeError;
  }

  const sourcePath = ".cache/clipboard/mock.txt";
  mockState.session.mode = "reader";
  mockState.session.active_source_path = sourcePath;
  upsertMockRecent(sourcePath);
  return structuredClone(mockState.session);
}

async function mockOnSourceOpen(handler: (event: SourceOpenEvent) => void): Promise<UnlistenFn> {
  const event: SourceOpenEvent = {
    phase: "ready",
    source_path: null,
    message: "Using mock backend adapter"
  };
  queueMicrotask(() => handler(event));
  return async () => {};
}

export interface BackendApi {
  sessionGetBootstrap: () => Promise<BootstrapState>;
  sessionGetState: () => Promise<SessionState>;
  sessionReturnToStarter: () => Promise<SessionState>;
  recentList: (limit?: number) => Promise<RecentBook[]>;
  recentDelete: (path: string) => Promise<void>;
  sourceOpenPath: (path: string) => Promise<SessionState>;
  sourceOpenClipboardText: (text: string) => Promise<SessionState>;
  onSourceOpen: (handler: (event: SourceOpenEvent) => void) => Promise<UnlistenFn>;
}

function createTauriBackendApi(): BackendApi {
  return {
    sessionGetBootstrap: () => invokeCommand<BootstrapState>("session_get_bootstrap"),
    sessionGetState: () => invokeCommand<SessionState>("session_get_state"),
    sessionReturnToStarter: () => invokeCommand<SessionState>("session_return_to_starter"),
    recentList: (limit) =>
      invokeCommand<RecentBook[]>("recent_list", { limit: normalizeRecentLimit(limit) }),
    recentDelete: (path) => invokeCommand<void>("recent_delete", { path }),
    sourceOpenPath: (path) => invokeCommand<SessionState>("source_open_path", { path }),
    sourceOpenClipboardText: (text) =>
      invokeCommand<SessionState>("source_open_clipboard_text", { text }),
    onSourceOpen: async (handler) => {
      return listen<SourceOpenEvent>("source-open", (event) => handler(event.payload));
    }
  };
}

function createMockBackendApi(): BackendApi {
  return {
    sessionGetBootstrap: mockSessionGetBootstrap,
    sessionGetState: mockSessionGetState,
    sessionReturnToStarter: mockSessionReturnToStarter,
    recentList: mockRecentList,
    recentDelete: mockRecentDelete,
    sourceOpenPath: mockSourceOpenPath,
    sourceOpenClipboardText: mockSourceOpenClipboardText,
    onSourceOpen: mockOnSourceOpen
  };
}

export const backendApi: BackendApi = isTauriRuntime()
  ? createTauriBackendApi()
  : createMockBackendApi();
