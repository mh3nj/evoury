import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { VersionRecord } from "../types";

interface VersionsState {
  versions: Record<string, VersionRecord[]>;
  loading: boolean;
  commit: (assetId: string, label: string, filePath: string, fileSize: number) => Promise<VersionRecord>;
  getHistory: (assetId: string) => Promise<VersionRecord[]>;
  restore: (assetId: string, versionId: string) => Promise<VersionRecord | null>;
  deleteVersion: (versionId: string) => Promise<void>;
  getLatest: (assetId: string) => Promise<VersionRecord | null>;
}

export const useVersionsStore = create<VersionsState>((set) => ({
  versions: {},
  loading: false,
  commit: async (assetId, label, filePath, fileSize) => {
    const record = await invoke<VersionRecord>("commit_version", { assetId, label, filePath, fileSize });
    return record;
  },
  getHistory: async (assetId) => {
    const history = await invoke<VersionRecord[]>("get_version_history", { assetId });
    set((s) => ({ versions: { ...s.versions, [assetId]: history } }));
    return history;
  },
  restore: async (assetId, versionId) => {
    return invoke<VersionRecord | null>("restore_version", { assetId, versionId });
  },
  deleteVersion: async (versionId) => {
    await invoke("delete_version", { versionId });
  },
  getLatest: async (assetId) => {
    return invoke<VersionRecord | null>("get_latest_version", { assetId });
  },
}));
