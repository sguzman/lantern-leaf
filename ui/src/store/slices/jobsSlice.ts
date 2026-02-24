import type { AppStore } from "../appStore";
import { buildToast } from "./shared";
import type { SliceContext } from "./types";

export async function ensureJobSubscriptions({ set, get, backend }: SliceContext): Promise<void> {
  if (!get().sourceOpenSubscribed) {
    await backend.onSourceOpen((event) => {
      set({ sourceOpenEvent: event });
      if (event.phase === "cancelled") {
        const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
        set({
          toast: buildToast("info", `${event.message ?? "Source open cancelled"}${suffix}`)
        });
        return;
      }
      if (event.phase === "failed") {
        const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
        set({
          toast: buildToast("error", `${event.message ?? "Source open failed"}${suffix}`)
        });
      }
    });
    set({ sourceOpenSubscribed: true });
  }

  if (!get().calibreSubscribed) {
    await backend.onCalibreLoad((event) => {
      set({ calibreLoadEvent: event });
      if (event.phase === "failed") {
        const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
        set({
          toast: buildToast("error", `${event.message ?? "Calibre load failed"}${suffix}`)
        });
      }
    });
    set({ calibreSubscribed: true });
  }

  if (!get().ttsStateSubscribed) {
    await backend.onTtsState((event) => {
      set({ ttsStateEvent: event });
    });
    set({ ttsStateSubscribed: true });
  }

  if (!get().pdfTranscriptionSubscribed) {
    await backend.onPdfTranscription((event) => {
      set({ pdfTranscriptionEvent: event });
      if (event.phase === "failed") {
        const suffix = event.request_id > 0 ? ` (request ${event.request_id})` : "";
        set({
          toast: buildToast("error", `${event.message ?? "PDF transcription failed"}${suffix}`)
        });
      }
    });
    set({ pdfTranscriptionSubscribed: true });
  }

  if (!get().logLevelSubscribed) {
    await backend.onLogLevel((event) => {
      set({
        logLevelEvent: event,
        runtimeLogLevel: event.level
      });
    });
    set({ logLevelSubscribed: true });
  }

  if (!get().sessionStateSubscribed) {
    await backend.onSessionState((event) => {
      set((current) => {
        if (event.request_id < current.lastSessionEventRequestId) {
          return {};
        }
        const next: Partial<AppStore> = {
          session: event.session,
          lastSessionEventRequestId: event.request_id
        };
        if (event.session.mode !== "reader") {
          next.reader = null;
          next.lastReaderEventRequestId = Math.max(
            current.lastReaderEventRequestId,
            event.request_id
          );
        }
        return next;
      });
    });
    set({ sessionStateSubscribed: true });
  }

  if (!get().readerStateSubscribed) {
    await backend.onReaderState((event) => {
      set((current) => {
        if (event.request_id < current.lastReaderEventRequestId) {
          return {};
        }
        const nextSession = current.session
          ? {
              ...current.session,
              mode: "reader" as const,
              active_source_path: event.reader.source_path,
              open_in_flight: false,
              panels: event.reader.panels
            }
          : {
              mode: "reader" as const,
              active_source_path: event.reader.source_path,
              open_in_flight: false,
              panels: event.reader.panels
            };

        return {
          session: nextSession,
          reader: event.reader,
          lastReaderEventRequestId: event.request_id,
          lastSessionEventRequestId: Math.max(
            current.lastSessionEventRequestId,
            event.request_id
          )
        };
      });
    });
    set({ readerStateSubscribed: true });
  }
}
