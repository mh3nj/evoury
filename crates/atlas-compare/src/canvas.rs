use serde::{Deserialize, Serialize};
use crate::mode::CompareMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareCanvas {
    pub mode: CompareMode,
    pub split_position: f32,
    pub left_fit: bool,
    pub right_fit: bool,
    pub background: String,
    pub show_overlay_controls: bool,
}

impl Default for CompareCanvas {
    fn default() -> Self {
        Self {
            mode: CompareMode::SideBySide,
            split_position: 0.5,
            left_fit: true,
            right_fit: true,
            background: "#1a1a1a".into(),
            show_overlay_controls: true,
        }
    }
}
