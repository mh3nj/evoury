use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetLink {
    pub asset_id: Uuid,
    pub preview_path: String,
    pub source_path: String,
}
