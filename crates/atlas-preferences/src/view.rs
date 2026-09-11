use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewPreference {
    pub grid_size: u32,
    pub remember_last_collection: bool,
    pub show_details: bool,
}

impl Default for ViewPreference {
    fn default() -> Self {
        Self {
            grid_size: 240,
            remember_last_collection: true,
            show_details: false,
        }
    }
}
