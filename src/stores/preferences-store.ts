import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { Preferences, ThemeMode, ViewPreference, PerformancePreference, InteractionPreference } from "../types/preferences";

const DEFAULT_VIEW: ViewPreference = { grid_size: 240, remember_last_collection: true, show_details: false };
const DEFAULT_PERF: PerformancePreference = { max_preview_memory: 512, enable_background_indexing: true, reduce_motion: false };
const DEFAULT_INT: InteractionPreference = { keyboard_navigation: true, double_click_to_open: true, show_tooltips: true };

interface PreferencesState {
  loaded: boolean;
  theme: ThemeMode;
  default_view: ViewPreference;
  performance: PerformancePreference;
  interaction: InteractionPreference;
  default_workspace: string | null;
  load: () => Promise<void>;
  save: () => Promise<void>;
  setTheme: (theme: ThemeMode) => void;
  setView: (v: Partial<ViewPreference>) => void;
  setPerformance: (p: Partial<PerformancePreference>) => void;
  setInteraction: (i: Partial<InteractionPreference>) => void;
  setDefaultWorkspace: (ws: string | null) => void;
}

export const usePreferencesStore = create<PreferencesState>((set, get) => ({
  loaded: false,
  theme: "Dark",
  default_view: DEFAULT_VIEW,
  performance: DEFAULT_PERF,
  interaction: DEFAULT_INT,
  default_workspace: null,

  load: async () => {
    try {
      const p = await invoke<Preferences>("get_preferences");
      set({
        loaded: true,
        theme: p.theme,
        default_view: p.default_view,
        performance: p.performance,
        interaction: p.interaction,
        default_workspace: p.default_workspace,
      });
    } catch {
      set({ loaded: true });
    }
  },

  save: async () => {
    const s = get();
    const prefs: Preferences = {
      theme: s.theme,
      default_workspace: s.default_workspace,
      default_view: s.default_view,
      performance: s.performance,
      interaction: s.interaction,
    };
    await invoke("set_preferences", { prefs });
  },

  setTheme: (theme) => {
    set({ theme });
    get().save();
  },

  setView: (partial) => {
    set((s) => ({ default_view: { ...s.default_view, ...partial } }));
    get().save();
  },

  setPerformance: (partial) => {
    set((s) => ({ performance: { ...s.performance, ...partial } }));
    get().save();
  },

  setInteraction: (partial) => {
    set((s) => ({ interaction: { ...s.interaction, ...partial } }));
    get().save();
  },

  setDefaultWorkspace: (default_workspace) => {
    set({ default_workspace });
    get().save();
  },
}));
