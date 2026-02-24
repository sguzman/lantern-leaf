import type { AppStore } from "../appStore";
import type { SliceContext } from "./types";

export function createStatsSliceActions({ set }: SliceContext): Pick<AppStore, "clearTelemetry"> {
  return {
    clearTelemetry: () => set({ telemetry: [] })
  };
}
