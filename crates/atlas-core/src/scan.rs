use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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

/// What an incremental scan remembers about one folder.
///
/// A folder whose modification time has not changed has the same set of entries
/// (creating, deleting or renaming a file updates it), so its packages can be reused
/// without listing it again. Replacing an archive in place with the same name and a
/// different size is not seen this way; a full or verification scan catches that.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FolderState {
    pub path: PathBuf,
    /// Modification time as nanoseconds since the Unix epoch.
    pub modified_ns: i64,
    /// Names of the sub-folders that were found inside.
    pub subdirs: Vec<String>,
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
