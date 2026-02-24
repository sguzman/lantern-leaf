import type { AppStore } from "../appStore";
import { toBridgeError } from "./shared";
import type { SliceContext } from "./types";

export function createTtsSliceActions({ set, backend }: SliceContext): Pick<
  AppStore,
  | "readerTtsPlay"
  | "readerTtsPause"
  | "readerTtsTogglePlayPause"
  | "readerTtsPlayFromPageStart"
  | "readerTtsPlayFromHighlight"
  | "readerTtsSeekNext"
  | "readerTtsSeekPrev"
  | "readerTtsRepeatSentence"
  | "readerTtsPrecomputePage"
> {
  const syncReader = async (
    fn: () => Promise<Awaited<ReturnType<typeof backend.readerGetSnapshot>>>
  ) => {
    try {
      const reader = await fn();
      set({ reader });
    } catch (error) {
      set({ error: toBridgeError(error).message });
    }
  };

  return {
    readerTtsPlay: async () => syncReader(() => backend.readerTtsPlay()),
    readerTtsPause: async () => syncReader(() => backend.readerTtsPause()),
    readerTtsTogglePlayPause: async () => syncReader(() => backend.readerTtsTogglePlayPause()),
    readerTtsPlayFromPageStart: async () =>
      syncReader(() => backend.readerTtsPlayFromPageStart()),
    readerTtsPlayFromHighlight: async () =>
      syncReader(() => backend.readerTtsPlayFromHighlight()),
    readerTtsSeekNext: async () => syncReader(() => backend.readerTtsSeekNext()),
    readerTtsSeekPrev: async () => syncReader(() => backend.readerTtsSeekPrev()),
    readerTtsRepeatSentence: async () => syncReader(() => backend.readerTtsRepeatSentence()),
    readerTtsPrecomputePage: async () => syncReader(() => backend.readerTtsPrecomputePage())
  };
}
