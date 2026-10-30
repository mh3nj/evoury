import { create } from "zustand";

interface CommandState {
  open: boolean;
  settingsOpen: boolean;
  themeStudioOpen: boolean;
  openPalette: () => void;
  closePalette: () => void;
  openSettings: () => void;
  closeSettings: () => void;
  openThemeStudio: () => void;
  closeThemeStudio: () => void;
}

export const useCommandStore = create<CommandState>((set) => ({
  open: false,
  settingsOpen: false,
  themeStudioOpen: false,
  openPalette() {
    set({ open: true });
  },
  closePalette() {
    set({ open: false });
  },
  openSettings() {
    set({ settingsOpen: true });
  },
  closeSettings() {
    set({ settingsOpen: false });
  },
  openThemeStudio() {
    set({ themeStudioOpen: true });
  },
  closeThemeStudio() {
    set({ themeStudioOpen: false });
  },
}));
