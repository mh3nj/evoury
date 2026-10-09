export type ThemeMode = "Dark" | "Light" | "System";
export type ViewMode = "grid" | "comfortable" | "compact";
export type Density = "Compact" | "Normal" | "Comfortable";

export interface ViewPreference {
  grid_size: number;
  remember_last_collection: boolean;
  show_details: boolean;
  view_mode: ViewMode;
}

export interface PerformancePreference {
  max_preview_memory: number;
  enable_background_indexing: boolean;
  reduce_motion: boolean;
}

export interface InteractionPreference {
  keyboard_navigation: boolean;
  double_click_to_open: boolean;
  show_tooltips: boolean;
}

export interface Preferences {
  theme: ThemeMode;
  default_workspace: string | null;
  default_view: ViewPreference;
  performance: PerformancePreference;
  interaction: InteractionPreference;
  density: Density;
}
