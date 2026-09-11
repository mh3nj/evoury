use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workstation {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub library_paths: Vec<String>,
    pub thumbnail_size: u32,
    pub default_view: String,
    pub sort_field: String,
    pub sort_direction: String,
    pub tags: Vec<String>,
    pub accent_color: Option<String>,
}

impl Default for Workstation {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Default".into(),
            description: String::new(),
            icon: "fa-briefcase".into(),
            library_paths: vec![],
            thumbnail_size: 240,
            default_view: "grid".into(),
            sort_field: "name".into(),
            sort_direction: "asc".into(),
            tags: vec![],
            accent_color: None,
        }
    }
}
