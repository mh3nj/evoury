use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionMode {
    ExactHash,
    Filename,
    FileSize,
    FilenameAndSize,
    All,
}

impl DetectionMode {
    pub fn name(&self) -> &str {
        match self {
            Self::ExactHash => "Exact Hash",
            Self::Filename => "Filename",
            Self::FileSize => "File Size",
            Self::FilenameAndSize => "Filename + Size",
            Self::All => "All Methods",
        }
    }
}
