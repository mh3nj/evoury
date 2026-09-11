use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub asset_id: Uuid,
    pub tags: Vec<String>,
    pub custom_fields: HashMap<String, String>,
    pub notes: String,
    pub favorite: bool,
    pub rating: u8,
    pub last_opened: Option<DateTime<Utc>>,
    pub times_opened: u64,
}

impl Default for AssetMetadata {
    fn default() -> Self {
        Self {
            asset_id: Uuid::nil(),
            tags: Vec::new(),
            custom_fields: HashMap::new(),
            notes: String::new(),
            favorite: false,
            rating: 0,
            last_opened: None,
            times_opened: 0,
        }
    }
}
