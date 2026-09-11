import { create } from "zustand";
import { loadState, saveState } from "./persistence";

type ViewMode = "grid" | "comfortable" | "compact";

interface GalleryState {
  viewMode: ViewMode;
  gridSize: number;
  setViewMode: (mode: ViewMode) => void;
  setGridSize: (size: number) => void;
}

const saved = loadState();

export const useGalleryStore = create<GalleryState>((set) => ({
  viewMode: saved?.viewMode ?? "grid",
  gridSize: saved?.gridSize ?? 240,
  setViewMode(mode) {
    set({ viewMode: mode });
    saveState();
  },
  setGridSize(size) {
    set({ gridSize: size });
    saveState();
  },
}));
