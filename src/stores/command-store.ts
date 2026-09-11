import { create } from "zustand";

interface CommandState {
  open: boolean;
  settingsOpen: boolean;
  openPalette: () => void;
  closePalette: () => void;
  openSettings: () => void;
  closeSettings: () => void;
}

export const useCommandStore = create<CommandState>((set) => ({
  open: false,
  settingsOpen: false,
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
}));
