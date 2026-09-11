import { create } from "zustand";

export interface ViewProfile {
  id: string;
  name: string;
  mode: "grid" | "compact-grid" | "list" | "filmstrip" | "detail";
  thumbnailSize: number;
  sortField: string;
  sortDirection: string;
  groupBy: string | null;
  showDetails: boolean;
  density: string;
}

export interface PreviewProfile {
  id: string;
  name: string;
  background: string;
  zoomMode: string;
  showGrid: boolean;
}

export interface SearchProfile {
  id: string;
  name: string;
  indexedFields: string[];
  visibleFilters: string[];
  suggestedQueries: string[];
}

interface ProfilesState {
  activeView: string;
  activePreview: string;
  activeSearch: string;
  viewProfiles: ViewProfile[];
  previewProfiles: PreviewProfile[];
  searchProfiles: SearchProfile[];
  setActiveView: (id: string) => void;
  setActivePreview: (id: string) => void;
  setActiveSearch: (id: string) => void;
  registerView: (profile: ViewProfile) => void;
  registerPreview: (profile: PreviewProfile) => void;
  registerSearch: (profile: SearchProfile) => void;
}

export const useProfilesStore = create<ProfilesState>((set) => ({
  activeView: "grid",
  activePreview: "transparent-bg",
  activeSearch: "logo-design",
  viewProfiles: [
    { id: "grid", name: "Grid", mode: "grid", thumbnailSize: 256, sortField: "name", sortDirection: "asc", groupBy: null, showDetails: false, density: "normal" },
    { id: "compact-grid", name: "Compact Grid", mode: "compact-grid", thumbnailSize: 200, sortField: "name", sortDirection: "asc", groupBy: null, showDetails: false, density: "compact" },
    { id: "list", name: "List", mode: "list", thumbnailSize: 64, sortField: "name", sortDirection: "asc", groupBy: null, showDetails: true, density: "normal" },
    { id: "filmstrip", name: "Filmstrip", mode: "filmstrip", thumbnailSize: 400, sortField: "modified", sortDirection: "desc", groupBy: null, showDetails: true, density: "comfortable" },
  ],
  previewProfiles: [
    { id: "transparent-bg", name: "Transparent BG", background: "checkerboard", zoomMode: "fit", showGrid: false },
    { id: "dark-bg", name: "Dark BG", background: "#1a1a1a", zoomMode: "fill", showGrid: false },
    { id: "light-bg", name: "Light BG", background: "#f5f5f5", zoomMode: "fit", showGrid: true },
  ],
  searchProfiles: [
    { id: "logo-design", name: "Logo Design", indexedFields: ["name", "tags", "notes"], visibleFilters: ["type", "tags", "rating"], suggestedQueries: ["logo", "brand", "vector"] },
    { id: "packaging", name: "Packaging", indexedFields: ["name", "tags", "notes", "category"], visibleFilters: ["type", "date", "tags"], suggestedQueries: ["dieline", "mockup", "label"] },
    { id: "ui-design", name: "UI Design", indexedFields: ["name", "tags", "notes"], visibleFilters: ["type", "tags", "rating", "favorite"], suggestedQueries: ["icon", "button", "screen"] },
  ],
  setActiveView: (id) => set({ activeView: id }),
  setActivePreview: (id) => set({ activePreview: id }),
  setActiveSearch: (id) => set({ activeSearch: id }),
  registerView: (profile) => set((s) => ({ viewProfiles: [...s.viewProfiles.filter((p) => p.id !== profile.id), profile] })),
  registerPreview: (profile) => set((s) => ({ previewProfiles: [...s.previewProfiles.filter((p) => p.id !== profile.id), profile] })),
  registerSearch: (profile) => set((s) => ({ searchProfiles: [...s.searchProfiles.filter((p) => p.id !== profile.id), profile] })),
}));
