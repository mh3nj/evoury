use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSearch {
    pub id: Uuid,
    pub name: String,
    pub query_text: String,
    pub icon: String,
    pub color: Option<String>,
    pub folder_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFolder {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub icon: String,
    pub color: Option<String>,
}

impl SavedSearch {
    pub fn new(name: &str, query_text: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            query_text: query_text.to_string(),
            icon: "fa-search".into(),
            color: None,
            folder_id: None,
            created_at: chrono::Utc::now(),
        }
    }
}

impl SearchFolder {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent_id: None,
            icon: "fa-folder".into(),
            color: None,
        }
    }
}
