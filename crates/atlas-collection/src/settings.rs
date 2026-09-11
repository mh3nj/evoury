use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSettings {
    pub background: Option<String>,
    pub invert_preview: bool,
    pub blend_mode: PreviewBlendMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreviewBlendMode {
    Normal,
    Multiply,
    Screen,
    Difference,
    ColorDodge,
}

impl Default for CollectionSettings {
    fn default() -> Self {
        Self {
            background: None,
            invert_preview: false,
            blend_mode: PreviewBlendMode::Normal,
        }
    }
}
