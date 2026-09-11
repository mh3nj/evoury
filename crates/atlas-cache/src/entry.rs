use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub source_hash: String,
    pub preview_path: String,
    pub created: DateTime<Utc>,
    pub size: u64,
}
