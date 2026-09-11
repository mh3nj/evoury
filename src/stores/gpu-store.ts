import { create } from "zustand";

interface TextureInfo {
  id: string;
  assetId: string;
  width: number;
  height: number;
  format: string;
  bytes: number;
  state: "loading" | "resident" | "evicted" | "failed";
  priority: number;
}

interface GpuState {
  maxVramMb: number;
  maxTextures: number;
  totalVramBytes: number;
  residentCount: number;
  textures: TextureInfo[];
  trackTexture: (tex: TextureInfo) => void;
  evictTexture: (id: string) => void;
  setMaxVram: (mb: number) => void;
  setMaxTextures: (n: number) => void;
}

export const useGpuStore = create<GpuState>((set) => ({
  maxVramMb: 1024,
  maxTextures: 256,
  totalVramBytes: 0,
  residentCount: 0,
  textures: [],
  trackTexture: (tex) => set((s) => ({
    textures: [...s.textures.filter((t) => t.id !== tex.id), tex],
    totalVramBytes: [...s.textures.filter((t) => t.id !== tex.id), tex]
      .reduce((sum, t) => t.state === "resident" ? sum + t.bytes : sum, 0),
    residentCount: [...s.textures.filter((t) => t.id !== tex.id), tex]
      .filter((t) => t.state === "resident").length,
  })),
  evictTexture: (id) => set((s) => ({
    textures: s.textures.map((t) => t.id === id ? { ...t, state: "evicted" as const } : t),
    residentCount: s.textures.filter((t) => t.id !== id && t.state === "resident").length,
  })),
  setMaxVram: (maxVramMb) => set({ maxVramMb }),
  setMaxTextures: (maxTextures) => set({ maxTextures }),
}));
