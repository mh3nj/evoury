import { create } from "zustand";

interface ViewerState {
  open: boolean;
  assetId: string | null;
  zoom: number;
  openViewer: (id: string) => void;
  closeViewer: () => void;
  setZoom: (value: number) => void;
}

export const useViewerStore = create<ViewerState>((set) => ({
  open: false,
  assetId: null,
  zoom: 1,
  openViewer(id) {
    set({ open: true, assetId: id, zoom: 1 });
  },
  closeViewer() {
    set({ open: false, assetId: null });
  },
  setZoom(value) {
    set({ zoom: value });
  },
}));
