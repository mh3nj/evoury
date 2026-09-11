use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub icon_path: Option<String>,
    pub accent_color: Option<String>,
    pub current_collection: Option<Uuid>,
    pub current_category: Option<Uuid>,
    pub selected_asset: Option<Uuid>,
    pub view_mode: ViewMode,
    pub sidebar_open: bool,
    pub inspector_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewMode {
    Grid,
    List,
    Filmstrip,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Default".into(),
            icon_path: None,
            accent_color: None,
            current_collection: None,
            current_category: None,
            selected_asset: None,
            view_mode: ViewMode::Grid,
            sidebar_open: true,
            inspector_open: true,
        }
    }
}
