import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

interface UndoState {
  canUndo: boolean;
  canRedo: boolean;
  undoLabel: string;
  check: () => Promise<void>;
  undo: () => Promise<string | null>;
  redo: () => Promise<string | null>;
  push: (label: string, undoData: string, redoData: string) => Promise<void>;
}

export const useUndoStore = create<UndoState>((set) => ({
  canUndo: false,
  canRedo: false,
  undoLabel: "",
  check: async () => {
    try {
      const [cu, cr] = await Promise.all([invoke<boolean>("can_undo"), invoke<boolean>("can_redo")]);
      set({ canUndo: cu, canRedo: cr });
    } catch { /* ignore */ }
  },
  undo: async () => {
    try { return await invoke<string>("undo"); } catch { return null; }
  },
  redo: async () => {
    try { return await invoke<string>("redo"); } catch { return null; }
  },
  push: async (label, undoData, redoData) => {
    await invoke("push_undo", { label, undoData, redoData });
    set({ canUndo: true });
  },
}));
