import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { Collection, SmartCollection } from "../types";

interface CollectionsState {
  collections: Collection[];
  smartCollections: SmartCollection[];
  loading: boolean;
  loadCollections: () => Promise<void>;
  addCollection: (collection: Collection) => Promise<void>;
  removeCollection: (id: string) => Promise<void>;
  moveCollection: (id: string, newParentId: string | null) => Promise<void>;
  getChildren: (parentId: string) => Promise<Collection[]>;
}

export const useCollectionsStore = create<CollectionsState>((set) => ({
  collections: [],
  smartCollections: [],
  loading: false,
  loadCollections: async () => {
    set({ loading: true });
    try {
      const collections = await invoke<Collection[]>("get_collections");
      set({ collections, loading: false });
    } catch { set({ loading: false }); }
  },
  addCollection: async (collection) => {
    await invoke("add_collection", { collection });
    const collections = await invoke<Collection[]>("get_collections");
    set({ collections });
  },
  removeCollection: async (id) => {
    await invoke("remove_collection", { collectionId: id });
    set((s) => ({ collections: s.collections.filter((c) => c.id !== id) }));
  },
  moveCollection: async (id, newParentId) => {
    await invoke("move_collection", { collectionId: id, newParentId });
    const collections = await invoke<Collection[]>("get_collections");
    set({ collections });
  },
  getChildren: async (parentId) => {
    return invoke<Collection[]>("get_collection_children", { collectionId: parentId });
  },
}));
