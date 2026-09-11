use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewMode {
    Grid,
    CompactGrid,
    List,
    Filmstrip,
    Detail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewProfileEntry {
    pub id: String,
    pub name: String,
    pub mode: ViewMode,
    pub thumbnail_size: u32,
    pub sort_field: String,
    pub sort_direction: String,
    pub group_by: Option<String>,
    pub show_details: bool,
    pub density: String,
}

impl ViewProfileEntry {
    pub fn grid() -> Self {
        Self {
            id: "grid".into(),
            name: "Grid".into(),
            mode: ViewMode::Grid,
            thumbnail_size: 256,
            sort_field: "name".into(),
            sort_direction: "asc".into(),
            group_by: None,
            show_details: false,
            density: "normal".into(),
        }
    }

    pub fn compact_grid() -> Self {
        Self {
            id: "compact-grid".into(),
            name: "Compact Grid".into(),
            mode: ViewMode::CompactGrid,
            thumbnail_size: 200,
            sort_field: "name".into(),
            sort_direction: "asc".into(),
            group_by: None,
            show_details: false,
            density: "compact".into(),
        }
    }

    pub fn list() -> Self {
        Self {
            id: "list".into(),
            name: "List".into(),
            mode: ViewMode::List,
            thumbnail_size: 64,
            sort_field: "name".into(),
            sort_direction: "asc".into(),
            group_by: None,
            show_details: true,
            density: "normal".into(),
        }
    }

    pub fn filmstrip() -> Self {
        Self {
            id: "filmstrip".into(),
            name: "Filmstrip".into(),
            mode: ViewMode::Filmstrip,
            thumbnail_size: 400,
            sort_field: "modified".into(),
            sort_direction: "desc".into(),
            group_by: None,
            show_details: true,
            density: "comfortable".into(),
        }
    }
}
