import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { StudioTheme } from "../types/theme";
import {
  BUILTIN_THEMES,
  applyColorThemeToDOM,
  parseThemeContent,
  resolveTheme,
  slugify,
} from "../themes/palette";
import { useThemeStore } from "./theme-store";

interface ThemeStudioState {
  loaded: boolean;
  userThemes: StudioTheme[];
  error: string | null;
  load: () => Promise<void>;
  apply: (theme: StudioTheme) => Promise<void>;
  save: (theme: StudioTheme) => Promise<StudioTheme>;
  saveAndApply: (theme: StudioTheme) => Promise<void>;
  remove: (id: string) => Promise<void>;
  importTheme: () => Promise<StudioTheme | null>;
  exportTheme: (theme: StudioTheme) => Promise<void>;
  preview: (theme: StudioTheme) => void;
  stopPreview: () => void;
}

function uniqueId(id: string, taken: Set<string>): string {
  if (!taken.has(id)) return id;
  let n = 2;
  while (taken.has(`${id}-${n}`)) n += 1;
  return `${id}-${n}`;
}

export const useThemeStudioStore = create<ThemeStudioState>((set, get) => ({
  loaded: false,
  userThemes: [],
  error: null,

  load: async () => {
    try {
      const userThemes = await invoke<StudioTheme[]>("get_themes");
      set({ userThemes, loaded: true, error: null });
    } catch (e) {
      set({ loaded: true, error: String(e) });
    }
  },

  apply: async (theme) => {
    await useThemeStore.getState().setColorTheme(theme);
  },

  save: async (theme) => {
    const normalized: StudioTheme = {
      ...theme,
      id: slugify(theme.id || theme.name),
    };
    await invoke("save_theme", { theme: normalized });
    await get().load();
    return normalized;
  },

  saveAndApply: async (theme) => {
    const saved = await get().save(theme);
    await get().apply(saved);
  },

  remove: async (id) => {
    await invoke("delete_theme", { id });
    await get().load();
    const active = useThemeStore.getState().config.color_theme;
    if (active === id) {
      await useThemeStore.getState().setColorTheme(null);
    }
  },

  importTheme: async () => {
    const content = await invoke<string | null>("import_theme");
    if (!content) return null;
    const parsed = parseThemeContent(content);
    if (!parsed) {
      set({
        error: "That file is not a theme JSON (VS Code or Evoury format).",
      });
      return null;
    }
    const taken = new Set([
      ...BUILTIN_THEMES.map((t) => t.id),
      ...get().userThemes.map((t) => t.id),
    ]);
    const theme: StudioTheme = { ...parsed, id: uniqueId(parsed.id, taken) };
    await invoke("save_theme", { theme });
    await get().load();
    set({ error: null });
    return theme;
  },

  exportTheme: async (theme) => {
    await invoke("export_theme", { theme });
  },

  preview: (theme) => {
    applyColorThemeToDOM(theme);
  },

  stopPreview: () => {
    const config = useThemeStore.getState().config;
    const active = resolveTheme(config.color_theme, get().userThemes);
    applyColorThemeToDOM(active ?? null);
  },
}));
