import { create } from "zustand";

export interface WorkstationPreset {
  id: string;
  name: string;
  role: string;
  description: string;
  defaultThumbnailSize: number;
  defaultSortField: string;
  defaultSortDirection: string;
  enabledFilters: string[];
  welcomeMessage: string;
}

interface WorkstationState {
  activeWorkstation: string | null;
  presets: WorkstationPreset[];
  setWorkstation: (id: string) => void;
  registerPreset: (preset: WorkstationPreset) => void;
}

export const useWorkstationStore = create<WorkstationState>((set) => ({
  activeWorkstation: null,
  presets: [
    {
      id: "logo-design", name: "Logo Design", role: "LogoDesign",
      description: "Optimized for logo files and brand assets",
      defaultThumbnailSize: 256, defaultSortField: "name", defaultSortDirection: "asc",
      enabledFilters: ["type", "tags", "rating", "date"],
      welcomeMessage: "Welcome to Logo Design mode.",
    },
    {
      id: "packaging", name: "Packaging", role: "Packaging",
      description: "Layout for packaging mockups and print assets",
      defaultThumbnailSize: 320, defaultSortField: "modified", defaultSortDirection: "desc",
      enabledFilters: ["type", "tags", "date"],
      welcomeMessage: "Packaging mode active.",
    },
    {
      id: "ui-design", name: "UI Design", role: "UiDesign",
      description: "Perfect for UI kits, icons, and screenshots",
      defaultThumbnailSize: 200, defaultSortField: "name", defaultSortDirection: "asc",
      enabledFilters: ["type", "tags", "rating", "favorite"],
      welcomeMessage: "UI Design mode loaded.",
    },
  ],
  setWorkstation: (id) => set({ activeWorkstation: id }),
  registerPreset: (preset) =>
    set((s) => ({ presets: [...s.presets.filter((p) => p.id !== preset.id), preset] })),
}));
