use serde::{Deserialize, Serialize};
use crate::types::FileType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub path: String,
    pub file_type: FileType,
}
