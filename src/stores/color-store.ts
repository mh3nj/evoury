import { create } from "zustand";

export type ColorSpace = "srgb" | "adobe-rgb" | "display-p3" | "prophoto-rgb" | "unmanaged";
export type DisplayColorSpace = "native" | "srgb" | "display-p3" | "hdr10";

interface ColorState {
  displaySpace: DisplayColorSpace;
  enableIcc: boolean;
  enableWideGamut: boolean;
  sourceSpace: ColorSpace;
  setDisplaySpace: (space: DisplayColorSpace) => void;
  setEnableIcc: (enable: boolean) => void;
  setEnableWideGamut: (enable: boolean) => void;
  setSourceSpace: (space: ColorSpace) => void;
}

export const useColorStore = create<ColorState>((set) => ({
  displaySpace: "srgb",
  enableIcc: true,
  enableWideGamut: false,
  sourceSpace: "srgb",
  setDisplaySpace: (displaySpace) => set({ displaySpace }),
  setEnableIcc: (enableIcc) => set({ enableIcc }),
  setEnableWideGamut: (enableWideGamut) => set({ enableWideGamut }),
  setSourceSpace: (sourceSpace) => set({ sourceSpace }),
}));
