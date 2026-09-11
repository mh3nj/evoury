import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { RenamePattern, RenamePreview } from "../types";

interface BatchState {
  renames: RenamePreview[];
  loading: boolean;
  renamePreview: (assetIds: string[], names: string[], pattern: RenamePattern) => Promise<RenamePreview[]>;
  renameApply: (previews: RenamePreview[]) => Promise<[string, string][]>;
  setTags: (assetIds: string[], addTags: string[], removeTags: string[]) => Promise<[string, string[]][]>;
  setRating: (assetIds: string[], rating: number) => Promise<[string, number][]>;
  setFavorite: (assetIds: string[], favorite: boolean) => Promise<[string, boolean][]>;
}

export const useBatchStore = create<BatchState>(() => ({
  renames: [],
  loading: false,
  renamePreview: async (assetIds, names, pattern) => {
    return invoke<RenamePreview[]>("batch_rename_preview", { assetIds, names, pattern });
  },
  renameApply: async (previews) => {
    return invoke<[string, string][]>("batch_rename_apply", { previews });
  },
  setTags: async (assetIds, addTags, removeTags) => {
    return invoke<[string, string[]][]>("batch_set_tags", { assetIds, addTags, removeTags });
  },
  setRating: async (assetIds, rating) => {
    return invoke<[string, number][]>("batch_set_rating", { assetIds, rating });
  },
  setFavorite: async (assetIds, favorite) => {
    return invoke<[string, boolean][]>("batch_set_favorite", { assetIds, favorite });
  },
}));
