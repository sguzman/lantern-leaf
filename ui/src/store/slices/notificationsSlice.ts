import type { AppStore } from "../appStore";
import type { SliceContext } from "./types";

export function createNotificationsSliceActions({ set }: SliceContext): Pick<
  AppStore,
  "clearError" | "dismissToast"
> {
  return {
    clearError: () => set({ error: null }),
    dismissToast: () => set({ toast: null })
  };
}
