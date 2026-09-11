import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { SavedSearch, SearchFolder } from "../types";

interface SavedSearchesState {
  searches: SavedSearch[];
  folders: SearchFolder[];
  loading: boolean;
  loadAll: () => Promise<void>;
  saveSearch: (name: string, queryText: string) => Promise<SavedSearch>;
  removeSearch: (id: string) => Promise<void>;
  createFolder: (name: string) => Promise<SearchFolder>;
  moveSearch: (searchId: string, folderId: string | null) => Promise<void>;
}

export const useSavedSearchesStore = create<SavedSearchesState>((set) => ({
  searches: [],
  folders: [],
  loading: false,
  loadAll: async () => {
    set({ loading: true });
    try {
      const [searches, folders] = await Promise.all([
        invoke<SavedSearch[]>("get_saved_searches"),
        invoke<SearchFolder[]>("get_search_folders"),
      ]);
      set({ searches, folders, loading: false });
    } catch { set({ loading: false }); }
  },
  saveSearch: async (name, queryText) => {
    const search = await invoke<SavedSearch>("save_search", { name, queryText });
    set((s) => ({ searches: [...s.searches, search] }));
    return search;
  },
  removeSearch: async (id) => {
    await invoke("remove_saved_search", { searchId: id });
    set((s) => ({ searches: s.searches.filter((se) => se.id !== id) }));
  },
  createFolder: async (name) => {
    const folder = await invoke<SearchFolder>("create_search_folder", { name });
    set((s) => ({ folders: [...s.folders, folder] }));
    return folder;
  },
  moveSearch: async (searchId, folderId) => {
    await invoke("move_search_to_folder", { searchId, folderId });
    const searches = await invoke<SavedSearch[]>("get_saved_searches");
    set({ searches });
  },
}));
