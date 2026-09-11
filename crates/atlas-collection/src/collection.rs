use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::settings::CollectionSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub path: Option<String>,
    pub icon_path: Option<String>,
    pub accent_color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub settings: CollectionSettings,
}
