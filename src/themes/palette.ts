import type { StudioTheme, ThemeColors, ThemeMode } from "../types/theme";

interface VsCodeThemeJson {
  type?: string;
  name?: string;
  colors?: Record<string, string>;
}

// VS Code color key -> palette role. First defined key wins.
const SOURCES: Record<keyof ThemeColors, string[]> = {
  bg: ["editor.background", "sideBar.background", "activityBar.background"],
  surface: [
    "sideBar.background",
    "activityBar.background",
    "editor.background",
  ],
  elevated: [
    "editorWidget.background",
    "editor.background",
    "sideBar.background",
  ],
  border: [
    "editorGroupHeader.tabsBorder",
    "sideBar.border",
    "activityBar.border",
  ],
  border_light: [
    "sideBar.border",
    "editorGroupHeader.tabsBorder",
    "activityBar.border",
  ],
  text: ["editor.foreground", "sideBar.foreground", "foreground"],
  text_dim: ["descriptionForeground", "editor.foreground"],
  accent: ["focusBorder", "button.background", "activityBarBadge.background"],
  accent_glow: [],
  accent2: ["charts.blue", "focusBorder"],
  accent2_glow: [],
  success: ["charts.green"],
  warning: ["charts.yellow"],
  error: ["charts.red"],
};

const CSS_VARS: Record<keyof ThemeColors, string> = {
  bg: "--evoury-bg",
  surface: "--evoury-surface",
  elevated: "--evoury-elevated",
  border: "--evoury-border",
  border_light: "--evoury-border-light",
  text: "--evoury-text",
  text_dim: "--evoury-text-dim",
  accent: "--evoury-accent",
  accent_glow: "--evoury-accent-glow",
  accent2: "--evoury-accent2",
  accent2_glow: "--evoury-accent2-glow",
  success: "--evoury-success",
  warning: "--evoury-warning",
  error: "--evoury-error",
};

export function stripAlpha(value: string): string {
  const v = value.trim();
  if (/^#[0-9a-fA-F]{8}$/.test(v)) return v.slice(0, 7);
  return v;
}

export function hexToRgba(hex: string, alpha: number): string {
  const v = stripAlpha(hex);
  if (!/^#[0-9a-fA-F]{6}$/.test(v)) return v;
  const r = parseInt(v.slice(1, 3), 16);
  const g = parseInt(v.slice(3, 5), 16);
  const b = parseInt(v.slice(5, 7), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

export function slugify(name: string): string {
  const slug = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return slug || "theme";
}

export function mapVsCodeTheme(json: unknown, id?: string): StudioTheme | null {
  if (!json || typeof json !== "object") return null;
  const raw = json as VsCodeThemeJson;
  const colors = raw.colors;
  if (!colors || typeof colors !== "object") return null;

  const stem = id ?? slugify(raw.name ?? "theme");
  const mode: ThemeMode = raw.type === "light" ? "light" : "dark";
  const pick = (keys: string[]): string => {
    for (const k of keys) {
      const v = colors[k];
      if (typeof v === "string" && v.trim()) return stripAlpha(v);
    }
    return "";
  };

  const accent = pick(SOURCES.accent) || "#6366f1";
  const accent2 = pick(SOURCES.accent2) || accent;
  const out: ThemeColors = {
    bg: pick(SOURCES.bg),
    surface: pick(SOURCES.surface),
    elevated: pick(SOURCES.elevated),
    border: pick(SOURCES.border),
    border_light: pick(SOURCES.border_light),
    text: pick(SOURCES.text),
    text_dim: pick(SOURCES.text_dim),
    accent,
    accent_glow: hexToRgba(accent, 0.2),
    accent2,
    accent2_glow: hexToRgba(accent2, 0.15),
    success: pick(SOURCES.success),
    warning: pick(SOURCES.warning),
    error: pick(SOURCES.error),
  };
  if (!out.bg || !out.text) return null;

  return {
    id: stem,
    name:
      raw.name?.trim() ||
      stem.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase()),
    mode,
    colors: out,
  };
}

const modules = import.meta.glob("./*.json", { eager: true }) as Record<
  string,
  { default: unknown }
>;

export const BUILTIN_THEMES: StudioTheme[] = Object.entries(modules)
  .map(([path, mod]) => {
    const stem =
      path
        .split("/")
        .pop()
        ?.replace(/\.json$/, "") ?? "theme";
    return mapVsCodeTheme(mod.default, stem);
  })
  .filter((t): t is StudioTheme => t !== null)
  .sort((a, b) => a.name.localeCompare(b.name));

export function mergeThemes(
  userThemes: StudioTheme[],
): Map<string, StudioTheme> {
  const map = new Map<string, StudioTheme>();
  for (const t of BUILTIN_THEMES) map.set(t.id, t);
  for (const t of userThemes) map.set(t.id, t);
  return map;
}

const FALLBACK_COLORS: ThemeColors = {
  bg: "#1a1a2e",
  surface: "#222240",
  elevated: "#2a2a4a",
  border: "#3a3a5c",
  border_light: "#4a4a6e",
  text: "#e0e0f0",
  text_dim: "#8888b0",
  accent: "#6366f1",
  accent_glow: "rgba(99, 102, 241, 0.2)",
  accent2: "#06b6d4",
  accent2_glow: "rgba(6, 182, 212, 0.15)",
  success: "#22c55e",
  warning: "#f59e0b",
  error: "#ef4444",
};

export function blankTheme(name: string): StudioTheme {
  return {
    id: slugify(name),
    name,
    mode: "dark",
    colors: { ...FALLBACK_COLORS },
  };
}

function isPaletteTheme(json: any): json is StudioTheme {
  const colors = json?.colors;
  return (
    !!json &&
    typeof json.name === "string" &&
    typeof colors === "object" &&
    typeof colors.bg === "string" &&
    typeof colors.text === "string"
  );
}

export function parseThemeContent(content: string): StudioTheme | null {
  let json: unknown;
  try {
    json = JSON.parse(content);
  } catch {
    return null;
  }
  if (!json || typeof json !== "object") return null;
  const obj = json as Record<string, unknown>;

  // VS Code format: { type, name, colors: { "editor.background": ... } }
  if (obj.colors && typeof obj.colors === "object") {
    const firstKey = Object.keys(obj.colors as object)[0] ?? "";
    if (firstKey.includes(".") || typeof obj.type === "string") {
      const mapped = mapVsCodeTheme(obj);
      return mapped
        ? { ...mapped, id: mapped.id || slugify(mapped.name) }
        : null;
    }
  }
  if (!isPaletteTheme(obj)) return null;

  const colors: ThemeColors = { ...FALLBACK_COLORS };
  for (const key of Object.keys(FALLBACK_COLORS) as (keyof ThemeColors)[]) {
    const v = (obj.colors as unknown as Record<string, unknown>)[key];
    if (typeof v === "string" && v.trim()) colors[key] = stripAlpha(v);
  }
  const mode: ThemeMode =
    obj.mode === "light" || obj.mode === "dark" ? obj.mode : "dark";
  const name = String(obj.name).trim() || "Imported Theme";
  return {
    id:
      typeof obj.id === "string" && obj.id.trim()
        ? slugify(obj.id)
        : slugify(name),
    name,
    mode,
    colors,
  };
}

export function resolveTheme(
  id: string | null | undefined,
  userThemes: StudioTheme[],
): StudioTheme | undefined {
  if (!id) return undefined;
  return mergeThemes(userThemes).get(id);
}

export function applyColorThemeToDOM(theme: StudioTheme | null) {
  const root = document.documentElement;
  if (!theme) {
    for (const key of Object.keys(CSS_VARS) as (keyof ThemeColors)[]) {
      root.style.removeProperty(CSS_VARS[key]);
    }
    return;
  }
  for (const key of Object.keys(CSS_VARS) as (keyof ThemeColors)[]) {
    root.style.setProperty(CSS_VARS[key], theme.colors[key]);
  }
}
