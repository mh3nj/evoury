use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanMode {
    FullScan,
    IncrementalScan,
    FolderScan,
    FileScan,
    StartupScan,
    VerificationScan,
    RepairScan,
    BackgroundScan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanFingerprint {
    pub folder: PathBuf,
    pub file_count: u64,
    pub total_size: u64,
    pub last_modified: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub scan_id: Uuid,
    pub mode: ScanMode,
    pub total_files: u64,
    pub scanned_files: u64,
    pub archives_found: u64,
    pub previews_found: u64,
    pub assets_created: u64,
    pub errors: Vec<String>,
    pub current_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStats {
    pub total_archives: u64,
    pub total_previews: u64,
    pub paired: u64,
    pub missing_previews: u64,
    pub errors: u64,
    pub duration_ms: u64,
}
