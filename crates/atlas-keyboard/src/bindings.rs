use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BindingScope {
    Global,
    Gallery,
    Viewer,
    Presentation,
    Inspector,
    Search,
    CommandPalette,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub id: String,
    pub combo: String,
    pub scope: BindingScope,
    pub action: String,
    pub category: String,
}
