use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupContents {
    pub workspace: bool,
    pub collections: bool,
    pub metadata: bool,
    pub baskets: bool,
    pub preferences: bool,
}
