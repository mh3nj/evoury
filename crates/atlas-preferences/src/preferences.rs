use crate::interaction::InteractionPreference;
use crate::performance::PerformancePreference;
use crate::view::ViewPreference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub theme: ThemeMode,
    pub default_workspace: Option<String>,
    pub default_view: ViewPreference,
    pub performance: PerformancePreference,
    pub interaction: InteractionPreference,
    pub density: String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            default_workspace: None,
            default_view: ViewPreference::default(),
            performance: PerformancePreference::default(),
            interaction: InteractionPreference::default(),
            density: "Normal".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}
