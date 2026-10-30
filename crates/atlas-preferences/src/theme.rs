use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioTheme {
    pub id: String,
    pub name: String,
    pub mode: String,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub bg: String,
    pub surface: String,
    pub elevated: String,
    pub border: String,
    pub border_light: String,
    pub text: String,
    pub text_dim: String,
    pub accent: String,
    pub accent_glow: String,
    pub accent2: String,
    pub accent2_glow: String,
    pub success: String,
    pub warning: String,
    pub error: String,
}
