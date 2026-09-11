use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{ArchiveType, Preview};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetState {
    Discovered,
    Paired,
    Validated,
    Indexed,
    Ready,
    Broken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub archive_type: ArchiveType,
    pub archive_path: PathBuf,
    pub preview: Option<Preview>,
    pub file_size: u64,
    pub modified_at: DateTime<Utc>,
    pub collection_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub state: AssetState,
}

impl Asset {
    pub fn has_preview(&self) -> bool {
        self.preview.is_some()
    }
}
