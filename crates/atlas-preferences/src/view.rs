use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewPreference {
    pub grid_size: u32,
    pub remember_last_collection: bool,
    pub show_details: bool,
    pub view_mode: String,
}

impl Default for ViewPreference {
    fn default() -> Self {
        Self {
            grid_size: 240,
            remember_last_collection: true,
            show_details: false,
            view_mode: "grid".to_string(),
        }
    }
}
