use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundTask {
    // Thumbnail generation
    GenerateThumbnail { asset_id: Uuid },
    GenerateAllThumbnails,
    ClearThumbnailCache,

    // Duplicate detection
    DetectDuplicates { mode: String },
    RemoveDuplicates { group_id: Uuid, keep_id: Uuid },

    // Version detection
    DetectVersions,
    GroupVersions { group_id: Uuid },

    // Smart collections
    UpdateSmartCollections,
    RefreshCollection { collection_id: Uuid },

    // Library maintenance
    RebuildIndex,
    VacuumDatabase,
    ExportLibrary { path: String },
    ImportLibrary { path: String },

    // Metadata
    BatchMetadataUpdate { asset_ids: Vec<Uuid> },
}

impl BackgroundTask {
    pub fn id(&self) -> &str {
        match self {
            Self::GenerateThumbnail { .. } => "generate_thumbnail",
            Self::GenerateAllThumbnails => "generate_all_thumbnails",
            Self::ClearThumbnailCache => "clear_thumbnail_cache",
            Self::DetectDuplicates { .. } => "detect_duplicates",
            Self::RemoveDuplicates { .. } => "remove_duplicates",
            Self::DetectVersions => "detect_versions",
            Self::GroupVersions { .. } => "group_versions",
            Self::UpdateSmartCollections => "update_smart_collections",
            Self::RefreshCollection { .. } => "refresh_collection",
            Self::RebuildIndex => "rebuild_index",
            Self::VacuumDatabase => "vacuum_database",
            Self::ExportLibrary { .. } => "export_library",
            Self::ImportLibrary { .. } => "import_library",
            Self::BatchMetadataUpdate { .. } => "batch_metadata_update",
        }
    }

    pub fn priority(&self) -> TaskPriority {
        match self {
            Self::GenerateThumbnail { .. } => TaskPriority::Normal,
            Self::GenerateAllThumbnails => TaskPriority::Low,
            Self::ClearThumbnailCache => TaskPriority::Low,
            Self::DetectDuplicates { .. } => TaskPriority::Normal,
            Self::RemoveDuplicates { .. } => TaskPriority::High,
            Self::DetectVersions => TaskPriority::Low,
            Self::GroupVersions { .. } => TaskPriority::Normal,
            Self::UpdateSmartCollections => TaskPriority::Low,
            Self::RefreshCollection { .. } => TaskPriority::Normal,
            Self::RebuildIndex => TaskPriority::Low,
            Self::VacuumDatabase => TaskPriority::Low,
            Self::ExportLibrary { .. } => TaskPriority::Normal,
            Self::ImportLibrary { .. } => TaskPriority::High,
            Self::BatchMetadataUpdate { .. } => TaskPriority::Normal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEntry {
    pub id: Uuid,
    pub task: BackgroundTask,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress: f32,
}
