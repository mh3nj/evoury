use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub current_collection: Option<Uuid>,
    pub current_category: Option<Uuid>,
    pub selected_asset: Option<Uuid>,
    pub view_mode: ViewMode,
    pub sidebar_open: bool,
    pub inspector_open: bool,
    pub search_query: Option<String>,
    pub active_filters: Vec<String>,
    pub basket_id: Option<Uuid>,
    pub zoom_level: f32,
    pub sort_field: String,
    pub sort_direction: String,
    pub last_scan_path: Option<String>,
    pub scroll_position: u32,
    pub focus_mode: bool,
    pub active_tab: Option<String>,
    pub navigation_position: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewMode {
    Grid,
    CompactGrid,
    List,
    Filmstrip,
    Detail,
}

impl Workspace {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            icon: "fa-window-maximize".into(),
            current_collection: None,
            current_category: None,
            selected_asset: None,
            view_mode: ViewMode::Grid,
            sidebar_open: true,
            inspector_open: true,
            search_query: None,
            active_filters: vec![],
            basket_id: None,
            zoom_level: 1.0,
            sort_field: "name".into(),
            sort_direction: "asc".into(),
            last_scan_path: None,
            scroll_position: 0,
            focus_mode: false,
            active_tab: None,
            navigation_position: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
