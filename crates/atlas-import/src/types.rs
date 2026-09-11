use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Folder,
    Archive,
    Image,
    DesignFile,
    Unknown,
}
