use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPreference {
    pub keyboard_navigation: bool,
    pub double_click_to_open: bool,
    pub show_tooltips: bool,
}

impl Default for InteractionPreference {
    fn default() -> Self {
        Self {
            keyboard_navigation: true,
            double_click_to_open: true,
            show_tooltips: true,
        }
    }
}
