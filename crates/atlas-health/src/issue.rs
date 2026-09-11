use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIssue {
    pub asset_id: Option<Uuid>,
    pub issue_type: IssueType,
    pub severity: IssueSeverity,
    pub message: String,
    pub repair_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    MissingPreview,
    MissingFile,
    BrokenMetadata,
    InvalidPath,
    CorruptedCache,
    OrphanRecord,
    DuplicateId,
    UnpairedAsset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}
