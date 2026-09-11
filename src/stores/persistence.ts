import { useGalleryStore, useWorkspaceStore } from ".";

const STORAGE_KEY = "evoury_state";

interface SavedState {
  libraryPath: string | null;
  viewMode: "grid" | "comfortable" | "compact";
  gridSize: number;
  currentCollection: string | null;
  currentCategory: string | null;
}

export function saveState() {
  const gallery = useGalleryStore.getState();
  const workspace = useWorkspaceStore.getState();
  const data: SavedState = {
    libraryPath: localStorage.getItem("evoury_last_library"),
    viewMode: gallery.viewMode,
    gridSize: gallery.gridSize,
    currentCollection: workspace.currentCollection,
    currentCategory: workspace.currentCategory,
  };
  localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
}

export function loadState(): SavedState | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? (JSON.parse(raw) as SavedState) : null;
  } catch {
    return null;
  }
}

export function saveLibraryPath(path: string) {
  localStorage.setItem("evoury_last_library", path);
  saveState();
}

export function getSavedLibraryPath(): string | null {
  return localStorage.getItem("evoury_last_library");
}

export function clearSavedLibraryPath() {
  localStorage.removeItem("evoury_last_library");
  saveState();
}
