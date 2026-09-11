use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewProfile {
    pub id: String,
    pub name: String,
    pub background: String,
    pub zoom_mode: String,
    pub show_grid: bool,
    pub loop_animation: bool,
    pub asset_types: Vec<String>,
}

impl Default for PreviewProfile {
    fn default() -> Self {
        Self {
            id: "default".into(),
            name: "Default".into(),
            background: "#1a1a2e".into(),
            zoom_mode: "fit".into(),
            show_grid: false,
            loop_animation: false,
            asset_types: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewProfile {
    pub id: String,
    pub name: String,
    pub thumbnail_size: u32,
    pub sort_field: String,
    pub sort_direction: String,
    pub group_by: Option<String>,
    pub show_details: bool,
    pub density: String,
}

impl Default for ViewProfile {
    fn default() -> Self {
        Self {
            id: "default".into(),
            name: "Default".into(),
            thumbnail_size: 240,
            sort_field: "name".into(),
            sort_direction: "asc".into(),
            group_by: None,
            show_details: false,
            density: "normal".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProfile {
    pub id: String,
    pub name: String,
    pub indexed_fields: Vec<String>,
    pub visible_filters: Vec<String>,
    pub suggested_queries: Vec<String>,
}

impl Default for SearchProfile {
    fn default() -> Self {
        Self {
            id: "default".into(),
            name: "Default".into(),
            indexed_fields: vec!["name".into(), "tags".into(), "notes".into()],
            visible_filters: vec!["type".into(), "rating".into(), "favorite".into(), "tag".into()],
            suggested_queries: vec![],
        }
    }
}
