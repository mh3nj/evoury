use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::state::PreviewStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewDescriptor {
    pub path: PathBuf,
    pub format: PreviewFormat,
    pub status: PreviewStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreviewFormat {
    Avif,
    Png,
    Jpeg,
}
