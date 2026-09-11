use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub severity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub total_pixels: u64,
    pub changed_pixels: u64,
    pub difference_pct: f64,
    pub regions: Vec<DiffRegion>,
}
