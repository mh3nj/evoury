use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePreference {
    pub max_preview_memory: u64,
    pub enable_background_indexing: bool,
    pub reduce_motion: bool,
}

impl Default for PerformancePreference {
    fn default() -> Self {
        Self {
            max_preview_memory: 512,
            enable_background_indexing: true,
            reduce_motion: false,
        }
    }
}
