export type ThemeMode = "dark" | "light";

export interface StudioTheme {
  id: string;
  name: string;
  mode: ThemeMode;
  colors: ThemeColors;
}

export interface ThemeColors {
  bg: string;
  surface: string;
  elevated: string;
  border: string;
  border_light: string;
  text: string;
  text_dim: string;
  accent: string;
  accent_glow: string;
  accent2: string;
  accent2_glow: string;
  success: string;
  warning: string;
  error: string;
}

export const COLOR_KEYS: (keyof ThemeColors)[] = [
  "bg",
  "surface",
  "elevated",
  "border",
  "border_light",
  "text",
  "text_dim",
  "accent",
  "accent_glow",
  "accent2",
  "accent2_glow",
  "success",
  "warning",
  "error",
];

export const COLOR_KEYS_SOLID: (keyof ThemeColors)[] = [
  "bg",
  "surface",
  "elevated",
  "border",
  "border_light",
  "text",
  "text_dim",
  "accent",
  "accent2",
  "success",
  "warning",
  "error",
];
