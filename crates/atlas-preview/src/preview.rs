use crate::state::PreviewStatus;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
