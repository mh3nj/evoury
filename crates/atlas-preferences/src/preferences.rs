use serde::{Deserialize, Serialize};
use crate::interaction::InteractionPreference;
use crate::performance::PerformancePreference;
use crate::view::ViewPreference;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub theme: ThemeMode,
    pub default_workspace: Option<String>,
    pub default_view: ViewPreference,
    pub performance: PerformancePreference,
    pub interaction: InteractionPreference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            default_workspace: None,
            default_view: ViewPreference::default(),
            performance: PerformancePreference::default(),
            interaction: InteractionPreference::default(),
        }
    }
}
