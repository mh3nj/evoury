import { create } from "zustand";
import { loadState, saveState } from "./persistence";
import { usePreferencesStore } from "./preferences-store";
import type { ViewMode } from "../types/preferences";

interface GalleryState {
  viewMode: ViewMode;
  gridSize: number;
  setViewMode: (mode: ViewMode) => void;
  setGridSize: (size: number) => void;
  applyView: (gridSize: number, viewMode: ViewMode) => void;
}

const saved = loadState();

export const useGalleryStore = create<GalleryState>((set) => ({
  viewMode: saved?.viewMode ?? "grid",
  gridSize: saved?.gridSize ?? 240,
  setViewMode(mode) {
    set({ viewMode: mode });
    saveState();
    usePreferencesStore.getState().setView({ view_mode: mode });
  },
  setGridSize(size) {
    set({ gridSize: size });
    saveState();
    usePreferencesStore.getState().setView({ grid_size: size });
  },
  applyView(gridSize, viewMode) {
    set({ gridSize, viewMode });
    saveState();
  },
}));
