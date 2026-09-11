use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::content::BackupContents;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub version: String,
    pub created: DateTime<Utc>,
    pub includes: BackupContents,
}
