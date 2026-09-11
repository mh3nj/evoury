use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Density {
    Compact,
    Normal,
    Comfortable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IconPack {
    FontAwesome,
    Phosphor,
    Lucide,
    Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyTokens {
    pub font_family: String,
    pub font_family_mono: String,
    pub font_size_xs: String,
    pub font_size_sm: String,
    pub font_size_base: String,
    pub font_size_lg: String,
    pub font_size_xl: String,
    pub font_size_2xl: String,
    pub font_size_3xl: String,
    pub font_weight_normal: u16,
    pub font_weight_medium: u16,
    pub font_weight_semibold: u16,
    pub font_weight_bold: u16,
    pub line_height_tight: f32,
    pub line_height_normal: f32,
    pub line_height_relaxed: f32,
    pub letter_spacing_tight: String,
    pub letter_spacing_normal: String,
    pub letter_spacing_wide: String,
}

impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            font_family: "\"Inter\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif".into(),
            font_family_mono: "\"JetBrains Mono\", \"Fira Code\", \"Cascadia Code\", monospace".into(),
            font_size_xs: "10px".into(),
            font_size_sm: "12px".into(),
            font_size_base: "14px".into(),
            font_size_lg: "16px".into(),
            font_size_xl: "18px".into(),
            font_size_2xl: "24px".into(),
            font_size_3xl: "32px".into(),
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_semibold: 600,
            font_weight_bold: 700,
            line_height_tight: 1.25,
            line_height_normal: 1.5,
            line_height_relaxed: 1.75,
            letter_spacing_tight: "-0.025em".into(),
            letter_spacing_normal: "0".into(),
            letter_spacing_wide: "0.05em".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingTokens {
    pub spacing_xs: String,
    pub spacing_sm: String,
    pub spacing_md: String,
    pub spacing_lg: String,
    pub spacing_xl: String,
    pub spacing_2xl: String,
    pub spacing_3xl: String,
    pub radius_sm: String,
    pub radius_md: String,
    pub radius_lg: String,
    pub radius_xl: String,
    pub radius_full: String,
}

impl Default for SpacingTokens {
    fn default() -> Self {
        Self {
            spacing_xs: "4px".into(),
            spacing_sm: "8px".into(),
            spacing_md: "12px".into(),
            spacing_lg: "16px".into(),
            spacing_xl: "24px".into(),
            spacing_2xl: "32px".into(),
            spacing_3xl: "48px".into(),
            radius_sm: "6px".into(),
            radius_md: "8px".into(),
            radius_lg: "12px".into(),
            radius_xl: "16px".into(),
            radius_full: "9999px".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,
    pub accent_color: String,
    pub border_radius: String,
    pub animation_speed: String,
    pub density: Density,
    pub icon_pack: IconPack,
    pub typography: TypographyTokens,
    pub spacing: SpacingTokens,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Dark,
            accent_color: "#6366f1".into(),
            border_radius: "12px".into(),
            animation_speed: "200ms".into(),
            density: Density::Normal,
            icon_pack: IconPack::FontAwesome,
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
        }
    }
}
