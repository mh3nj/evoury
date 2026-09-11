use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSelection {
    pub id: Uuid,
    pub name: String,
    pub asset_ids: Vec<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SavedSelection {
    pub fn new(name: &str, asset_ids: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            asset_ids,
            created_at: chrono::Utc::now(),
        }
    }
}
