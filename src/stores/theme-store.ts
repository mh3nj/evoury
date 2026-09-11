import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

export type ThemeMode = "Dark" | "Light" | "System";
export type Density = "Compact" | "Normal" | "Comfortable";
export type IconPack = "FontAwesome" | "Phosphor" | "Lucide" | "Material";

export interface TypographyTokens {
  font_family: string;
  font_family_mono: string;
  font_size_xs: string;
  font_size_sm: string;
  font_size_base: string;
  font_size_lg: string;
  font_size_xl: string;
  font_size_2xl: string;
  font_size_3xl: string;
  font_weight_normal: number;
  font_weight_medium: number;
  font_weight_semibold: number;
  font_weight_bold: number;
  line_height_tight: number;
  line_height_normal: number;
  line_height_relaxed: number;
  letter_spacing_tight: string;
  letter_spacing_normal: string;
  letter_spacing_wide: string;
}

export interface SpacingTokens {
  spacing_xs: string;
  spacing_sm: string;
  spacing_md: string;
  spacing_lg: string;
  spacing_xl: string;
  spacing_2xl: string;
  spacing_3xl: string;
  radius_sm: string;
  radius_md: string;
  radius_lg: string;
  radius_xl: string;
  radius_full: string;
}

export interface ThemeConfig {
  mode: ThemeMode;
  accent_color: string;
  border_radius: string;
  animation_speed: string;
  density: Density;
  icon_pack: IconPack;
  typography: TypographyTokens;
  spacing: SpacingTokens;
}

function applyThemeToDOM(theme: ThemeConfig) {
  const root = document.documentElement;
  const t = theme.typography;
  const s = theme.spacing;

  root.classList.toggle("theme-light", theme.mode === "Light");

  root.style.setProperty("--evoury-accent", theme.accent_color);
  root.style.setProperty("--evoury-radius", theme.border_radius);
  root.style.setProperty("--evoury-animation-speed", theme.animation_speed);

  root.style.setProperty("--evoury-font-family", t.font_family);
  root.style.setProperty("--evoury-font-family-mono", t.font_family_mono);
  root.style.setProperty("--evoury-font-size-xs", t.font_size_xs);
  root.style.setProperty("--evoury-font-size-sm", t.font_size_sm);
  root.style.setProperty("--evoury-font-size-base", t.font_size_base);
  root.style.setProperty("--evoury-font-size-lg", t.font_size_lg);
  root.style.setProperty("--evoury-font-size-xl", t.font_size_xl);
  root.style.setProperty("--evoury-font-size-2xl", t.font_size_2xl);
  root.style.setProperty("--evoury-font-size-3xl", t.font_size_3xl);

  root.style.setProperty("--evoury-spacing-xs", s.spacing_xs);
  root.style.setProperty("--evoury-spacing-sm", s.spacing_sm);
  root.style.setProperty("--evoury-spacing-md", s.spacing_md);
  root.style.setProperty("--evoury-spacing-lg", s.spacing_lg);
  root.style.setProperty("--evoury-spacing-xl", s.spacing_xl);
  root.style.setProperty("--evoury-spacing-2xl", s.spacing_2xl);
  root.style.setProperty("--evoury-spacing-3xl", s.spacing_3xl);

  root.style.setProperty("--evoury-radius-sm", s.radius_sm);
  root.style.setProperty("--evoury-radius-md", s.radius_md);
  root.style.setProperty("--evoury-radius-lg", s.radius_lg);
  root.style.setProperty("--evoury-radius-xl", s.radius_xl);

  root.style.fontFamily = t.font_family;
  root.style.fontSize = t.font_size_base;

  const mul = theme.density === "Compact" ? "0.75" : theme.density === "Comfortable" ? "1.25" : "1";
  root.style.setProperty("--evoury-density-multiplier", mul);
}

interface ThemeState {
  config: ThemeConfig;
  loading: boolean;
  load: () => Promise<void>;
  update: (partial: Partial<ThemeConfig>) => Promise<void>;
  setAccent: (color: string) => Promise<void>;
  setMode: (mode: ThemeMode) => Promise<void>;
  setDensity: (density: Density) => Promise<void>;
}

const defaultTheme: ThemeConfig = {
  mode: "Dark",
  accent_color: "#6366f1",
  border_radius: "12px",
  animation_speed: "200ms",
  density: "Normal",
  icon_pack: "FontAwesome",
  typography: {
    font_family: `"Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif`,
    font_family_mono: `"JetBrains Mono", "Fira Code", "Cascadia Code", monospace`,
    font_size_xs: "10px", font_size_sm: "12px", font_size_base: "14px",
    font_size_lg: "16px", font_size_xl: "18px", font_size_2xl: "24px", font_size_3xl: "32px",
    font_weight_normal: 400, font_weight_medium: 500, font_weight_semibold: 600, font_weight_bold: 700,
    line_height_tight: 1.25, line_height_normal: 1.5, line_height_relaxed: 1.75,
    letter_spacing_tight: "-0.025em", letter_spacing_normal: "0", letter_spacing_wide: "0.05em",
  },
  spacing: {
    spacing_xs: "4px", spacing_sm: "8px", spacing_md: "12px",
    spacing_lg: "16px", spacing_xl: "24px", spacing_2xl: "32px", spacing_3xl: "48px",
    radius_sm: "6px", radius_md: "8px", radius_lg: "12px", radius_xl: "16px", radius_full: "9999px",
  },
};

applyThemeToDOM(defaultTheme);

export const useThemeStore = create<ThemeState>((set, get) => ({
  config: defaultTheme,
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const prefs = await invoke<any>("get_preferences");
      if (prefs?.theme) {
        const config = { ...defaultTheme, mode: prefs.theme };
        applyThemeToDOM(config);
        set({ config, loading: false });
      } else {
        set({ loading: false });
      }
    } catch { set({ loading: false }); }
  },
  update: async (partial) => {
    const next = { ...get().config, ...partial };
    applyThemeToDOM(next);
    set({ config: next });
    try { await invoke("set_preferences", { prefs: { theme: next.mode } }); } catch { /* ignore */ }
  },
  setAccent: async (color) => {
    await get().update({ accent_color: color });
  },
  setMode: async (mode) => {
    await get().update({ mode });
  },
  setDensity: async (density) => {
    await get().update({ density });
  },
}));
