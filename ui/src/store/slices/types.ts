import type { BackendApi } from "../../api/tauri";
import type { AppStore } from "../appStore";

export type StoreSet = (
  partial: Partial<AppStore> | ((current: AppStore) => Partial<AppStore>)
) => void;

export type StoreGet = () => AppStore;

export interface SliceContext {
  set: StoreSet;
  get: StoreGet;
  backend: BackendApi;
}
