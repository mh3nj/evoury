use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShortcutCategory {
    Navigation,
    Selection,
    View,
    Metadata,
    Gallery,
    System,
    Custom,
}

impl ShortcutCategory {
    pub fn label(&self) -> &str {
        match self {
            Self::Navigation => "Navigation",
            Self::Selection => "Selection",
            Self::View => "View",
            Self::Metadata => "Metadata",
            Self::Gallery => "Gallery",
            Self::System => "System",
            Self::Custom => "Custom",
        }
    }
}
