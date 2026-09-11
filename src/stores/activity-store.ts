import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { ActivityEvent } from "../types";

interface ActivityState {
  events: ActivityEvent[];
  loading: boolean;
  record: (assetId: string, eventType: string, details: string) => Promise<ActivityEvent>;
  getForAsset: (assetId: string, limit?: number) => Promise<ActivityEvent[]>;
  getRecent: (hours?: number) => Promise<ActivityEvent[]>;
  getMostViewed: (limit?: number) => Promise<[string, number][]>;
}

export const useActivityStore = create<ActivityState>(() => ({
  events: [],
  loading: false,
  record: async (assetId, eventType, details) => {
    return invoke<ActivityEvent>("record_activity", { assetId, eventType, details });
  },
  getForAsset: async (assetId, limit = 50) => {
    return invoke<ActivityEvent[]>("get_asset_activity", { assetId, limit });
  },
  getRecent: async (hours = 24) => {
    return invoke<ActivityEvent[]>("get_recent_activity", { hours });
  },
  getMostViewed: async (limit = 10) => {
    return invoke<[string, number][]>("get_most_viewed", { limit });
  },
}));
