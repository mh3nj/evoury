import { create } from "zustand";

export interface KeyCombo {
  key: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
}

export interface ShortcutEntry {
  id: string;
  name: string;
  description: string;
  category: string;
  combo: KeyCombo;
  command: string;
  scope: string;
  enabled: boolean;
}

interface KeyboardState {
  shortcuts: ShortcutEntry[];
  enabled: boolean;
  registerShortcut: (entry: ShortcutEntry) => void;
  rebindShortcut: (id: string, combo: KeyCombo) => void;
  toggleShortcut: (id: string, enabled: boolean) => void;
  setEnabled: (enabled: boolean) => void;
  findMatch: (key: string, ctrl: boolean, shift: boolean, alt: boolean, meta: boolean, scope: string) => ShortcutEntry | undefined;
}

export const useKeyboardStore = create<KeyboardState>((set, get) => ({
  enabled: true,
  shortcuts: [
    { id: "nav.command-palette", name: "Command Palette", description: "Open the command palette", category: "Navigation", combo: { key: "k", ctrl: true, shift: false, alt: false, meta: false }, command: "command-palette.open", scope: "global", enabled: true },
    { id: "nav.back", name: "Go Back", description: "Navigate back", category: "Navigation", combo: { key: "[", ctrl: true, shift: false, alt: false, meta: false }, command: "navigation.back", scope: "global", enabled: true },
    { id: "nav.forward", name: "Go Forward", description: "Navigate forward", category: "Navigation", combo: { key: "]", ctrl: true, shift: false, alt: false, meta: false }, command: "navigation.forward", scope: "global", enabled: true },
    { id: "sel.select-all", name: "Select All", description: "Select all visible assets", category: "Selection", combo: { key: "a", ctrl: true, shift: false, alt: false, meta: false }, command: "selection.select-all", scope: "gallery", enabled: true },
    { id: "sel.invert", name: "Invert Selection", description: "Invert current selection", category: "Selection", combo: { key: "i", ctrl: true, shift: true, alt: false, meta: false }, command: "selection.invert", scope: "gallery", enabled: true },
    { id: "sel.clear", name: "Clear Selection", description: "Deselect all", category: "Selection", combo: { key: "Escape", ctrl: false, shift: false, alt: false, meta: false }, command: "selection.clear", scope: "gallery", enabled: true },
    { id: "view.toggle-sidebar", name: "Toggle Sidebar", description: "Show or hide sidebar", category: "View", combo: { key: "b", ctrl: true, shift: false, alt: false, meta: false }, command: "layout.toggle-sidebar", scope: "global", enabled: true },
    { id: "view.toggle-inspector", name: "Toggle Inspector", description: "Show or hide inspector", category: "View", combo: { key: "i", ctrl: true, shift: false, alt: false, meta: false }, command: "layout.toggle-inspector", scope: "global", enabled: true },
    { id: "view.focus-mode", name: "Focus Mode", description: "Toggle focus mode", category: "View", combo: { key: "f", ctrl: true, shift: true, alt: false, meta: false }, command: "layout.focus-mode", scope: "global", enabled: true },
    { id: "meta.favorite", name: "Toggle Favorite", description: "Favorite selected asset", category: "Metadata", combo: { key: "d", ctrl: true, shift: false, alt: false, meta: false }, command: "metadata.toggle-favorite", scope: "gallery", enabled: true },
    { id: "sys.settings", name: "Open Settings", description: "Open settings", category: "System", combo: { key: ",", ctrl: true, shift: false, alt: false, meta: false }, command: "settings.open", scope: "global", enabled: true },
    { id: "sys.search", name: "Focus Search", description: "Focus search bar", category: "System", combo: { key: "f", ctrl: true, shift: false, alt: false, meta: false }, command: "search.focus", scope: "global", enabled: true },
  ],

  registerShortcut: (entry) =>
    set((s) => ({ shortcuts: [...s.shortcuts.filter((sh) => sh.id !== entry.id), entry] })),

  rebindShortcut: (id, combo) =>
    set((s) => ({
      shortcuts: s.shortcuts.map((sh) => (sh.id === id ? { ...sh, combo } : sh)),
    })),

  toggleShortcut: (id, enabled) =>
    set((s) => ({
      shortcuts: s.shortcuts.map((sh) => (sh.id === id ? { ...sh, enabled } : sh)),
    })),

  setEnabled: (enabled) => set({ enabled }),

  findMatch: (key, ctrl, shift, alt, meta, scope) => {
    return get().shortcuts.find(
      (s) =>
        s.enabled &&
        (s.scope === scope || s.scope === "global") &&
        s.combo.key.toLowerCase() === key.toLowerCase() &&
        s.combo.ctrl === ctrl &&
        s.combo.shift === shift &&
        s.combo.alt === alt &&
        s.combo.meta === meta
    );
  },
}));
