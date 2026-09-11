use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSignature {
    pub hash: String,
    pub size: u64,
}
