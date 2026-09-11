use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRecord {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub label: String,
    pub description: String,
    pub previous_version_id: Option<Uuid>,
    pub file_path: String,
    pub file_size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl VersionRecord {
    pub fn new(asset_id: Uuid, label: &str, file_path: &str, file_size: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            asset_id,
            label: label.to_string(),
            description: String::new(),
            previous_version_id: None,
            file_path: file_path.to_string(),
            file_size,
            created_at: chrono::Utc::now(),
        }
    }
}
