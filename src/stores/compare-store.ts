import { create } from "zustand";

export type CompareMode = "side-by-side" | "overlay" | "slider" | "diff" | "swipe" | "split" | "toggle" | "difference" | "blend";

interface CompareState {
  open: boolean;
  mode: CompareMode;
  leftAssetId: string | null;
  rightAssetId: string | null;
  sliderPosition: number;
  zoom: number;
  linkedPan: boolean;
  synchronizedZoom: boolean;
  openCompare: (left: string, right: string) => void;
  closeCompare: () => void;
  setMode: (mode: CompareMode) => void;
  setSliderPosition: (pos: number) => void;
  setZoom: (zoom: number) => void;
  setLinkedPan: (linked: boolean) => void;
  setSynchronizedZoom: (sync: boolean) => void;
  swapAssets: () => void;
}

export const useCompareStore = create<CompareState>((set, get) => ({
  open: false,
  mode: "side-by-side",
  leftAssetId: null,
  rightAssetId: null,
  sliderPosition: 0.5,
  zoom: 1,
  linkedPan: true,
  synchronizedZoom: true,

  openCompare: (left, right) => set({ open: true, leftAssetId: left, rightAssetId: right, sliderPosition: 0.5, zoom: 1 }),
  closeCompare: () => set({ open: false, leftAssetId: null, rightAssetId: null }),
  setMode: (mode) => set({ mode }),
  setSliderPosition: (pos) => set({ sliderPosition: Math.max(0, Math.min(1, pos)) }),
  setZoom: (zoom) => set({ zoom: Math.max(0.1, Math.min(10, zoom)) }),
  setLinkedPan: (linkedPan) => set({ linkedPan }),
  setSynchronizedZoom: (synchronizedZoom) => set({ synchronizedZoom }),
  swapAssets: () => {
    const { leftAssetId, rightAssetId } = get();
    set({ leftAssetId: rightAssetId, rightAssetId: leftAssetId });
  },
}));
