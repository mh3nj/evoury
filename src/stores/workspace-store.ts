import { create } from "zustand";
import { loadState, saveState } from "./persistence";

interface WorkspaceState {
  currentWorkspace: string | null;
  currentCollection: string | null;
  currentCategory: string | null;
  setWorkspace: (id: string | null) => void;
  setCollection: (id: string | null) => void;
  setCategory: (id: string | null) => void;
}

const saved = loadState();

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  currentWorkspace: null,
  currentCollection: saved?.currentCollection ?? null,
  currentCategory: saved?.currentCategory ?? null,
  setWorkspace(id) {
    set({ currentWorkspace: id });
    saveState();
  },
  setCollection(id) {
    set({ currentCollection: id });
    saveState();
  },
  setCategory(id) {
    set({ currentCategory: id });
    saveState();
  },
}));
