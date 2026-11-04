use serde::{Deserialize, Serialize};

fn default_logo_color() -> String {
    "#a94f3d".to_string()
}

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
    /// Themes saved before the new logo existed simply fall back to brand rust.
    #[serde(default = "default_logo_color")]
    pub logo: String,
    pub success: String,
    pub warning: String,
    pub error: String,
}
