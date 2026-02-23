import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn()
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(async () => async () => {})
}));

type TauriModule = typeof import("../src/api/tauri");

function setTauriRuntimeWindow(): void {
  Object.defineProperty(globalThis, "window", {
    configurable: true,
    writable: true,
    value: { __TAURI_INTERNALS__: {} }
  });
}

async function loadTauriApiModule(): Promise<TauriModule> {
  vi.resetModules();
  setTauriRuntimeWindow();
  return import("../src/api/tauri");
}

beforeEach(() => {
  vi.clearAllMocks();
});

afterEach(() => {
  Reflect.deleteProperty(globalThis, "window");
});

describe("tauri command adapter", () => {
  it("normalizes recent-list limits before invoking the backend", async () => {
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockResolvedValue([]);
    const api = await loadTauriApiModule();

    await api.backendApi.recentList(9999);
    await api.backendApi.recentList(0);
    await api.backendApi.recentList();

    expect(invokeMock).toHaveBeenNthCalledWith(1, "recent_list", { limit: 512 });
    expect(invokeMock).toHaveBeenNthCalledWith(2, "recent_list", { limit: 1 });
    expect(invokeMock).toHaveBeenNthCalledWith(3, "recent_list", { limit: 64 });
  });

  it("propagates structured bridge errors unchanged", async () => {
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockRejectedValue({
      code: "invalid_input",
      message: "Path cannot be empty"
    });
    const api = await loadTauriApiModule();

    await expect(api.backendApi.sourceOpenPath("")).rejects.toEqual({
      code: "invalid_input",
      message: "Path cannot be empty"
    });
  });

  it("maps unknown errors to unknown_error bridge payload", async () => {
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockRejectedValue(new Error("boom"));
    const api = await loadTauriApiModule();

    await expect(api.backendApi.readerNextPage()).rejects.toEqual({
      code: "unknown_error",
      message: "boom"
    });
  });

  it("subscribes session-state listener on the expected event channel", async () => {
    const api = await loadTauriApiModule();
    const listenMock = vi.mocked(listen);
    const handler = vi.fn();

    await api.backendApi.onSessionState(handler);

    expect(listenMock).toHaveBeenCalledTimes(1);
    const [eventName, callback] = listenMock.mock.calls[0] as [string, (event: unknown) => void];
    expect(eventName).toBe("session-state");

    callback({
      payload: {
        request_id: 8,
        action: "session_return_to_starter",
        session: {
          mode: "starter",
          active_source_path: null,
          open_in_flight: false,
          panels: {
            show_settings: true,
            show_stats: false,
            show_tts: true
          }
        }
      }
    });

    expect(handler).toHaveBeenCalledWith({
      request_id: 8,
      action: "session_return_to_starter",
      session: {
        mode: "starter",
        active_source_path: null,
        open_in_flight: false,
        panels: {
          show_settings: true,
          show_stats: false,
          show_tts: true
        }
      }
    });
  });
});
