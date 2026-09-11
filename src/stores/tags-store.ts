import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { HierarchicalTag } from "../types";

interface TagsState {
  tags: HierarchicalTag[];
  loading: boolean;
  loadTags: () => Promise<void>;
  addTag: (tag: HierarchicalTag) => Promise<void>;
  removeTag: (id: string) => Promise<void>;
  updateTag: (id: string, name?: string, color?: string, description?: string) => Promise<void>;
  moveTag: (id: string, newParentId: string | null) => Promise<void>;
}

export const useTagsStore = create<TagsState>((set) => ({
  tags: [],
  loading: false,
  loadTags: async () => {
    set({ loading: true });
    try {
      const tags = await invoke<HierarchicalTag[]>("get_all_tags");
      set({ tags, loading: false });
    } catch { set({ loading: false }); }
  },
  addTag: async (tag) => {
    await invoke("add_tag", { tag });
    const tags = await invoke<HierarchicalTag[]>("get_all_tags");
    set({ tags });
  },
  removeTag: async (id) => {
    await invoke("remove_tag", { tagId: id });
    set((s) => ({ tags: s.tags.filter((t) => t.id !== id) }));
  },
  updateTag: async (id, name, color, description) => {
    await invoke("update_tag", { tagId: id, name, color, description });
    const tags = await invoke<HierarchicalTag[]>("get_all_tags");
    set({ tags });
  },
  moveTag: async (id, newParentId) => {
    await invoke("move_tag", { tagId: id, newParentId });
    const tags = await invoke<HierarchicalTag[]>("get_all_tags");
    set({ tags });
  },
}));
