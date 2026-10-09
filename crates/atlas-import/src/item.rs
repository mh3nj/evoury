use crate::types::FileType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportItem {
    pub path: String,
    pub file_type: FileType,
}
