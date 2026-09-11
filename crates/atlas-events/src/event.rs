use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use atlas_core::ScanProgress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtlasEvent {
    // Asset lifecycle
    AssetAdded { asset_id: Uuid },
    AssetUpdated { asset_id: Uuid },
    AssetRemoved { asset_id: Uuid },
    AssetStateChanged { asset_id: Uuid },

    // Collections
    CollectionAdded { collection_id: Uuid },
    CollectionRemoved { collection_id: Uuid },

    // Scanning
    ScanStarted { scan_id: Uuid, mode: String, root: PathBuf },
    ScanProgress(ScanProgress),
    ScanFinished { scan_id: Uuid },
    ScanError { scan_id: Uuid, error: String },

    // Watching
    WatcherEvent { kind: String, path: PathBuf },

    // Indexing
    IndexProgress { total: usize, completed: usize },
    IndexFinished,

    // Metadata
    MetadataChanged { asset_id: Uuid },

    // Baskets
    BasketChanged,

    // Workspace
    WorkspaceChanged { workspace_id: Uuid },

    // Commands
    CommandExecuted { command_id: Uuid, success: bool },
    CommandRegistered { command_id: Uuid, name: String },

    // Scheduler / Jobs
    JobQueued { job_id: Uuid, name: String },
    JobStarted { job_id: Uuid },
    JobCompleted { job_id: Uuid },
    JobFailed { job_id: Uuid, error: String },

    // Power / Sleep
    PowerModeChanged { mode: String },

    // Health / Diagnostics
    HealthCheckFinished,
    DiagnosticsUpdated,

    // Session
    SessionRestored { snapshot_id: String },
    SessionSaved { snapshot_id: String },

    // Registry / Capabilities
    ProviderRegistered { provider_id: String, kind: String },
    CapabilityRegistered { capability: String },

    // Notifications
    NotificationCreated { notification_id: Uuid, level: String, title: String },

    // Undo / Redo
    UndoPerformed { entry_id: Uuid, label: String },
    RedoPerformed { entry_id: Uuid, label: String },

    // Macros
    MacroRecordingStarted { name: String },
    MacroRecordingStopped { name: String, step_count: usize },
    MacroPlaybackStarted { name: String },
    MacroPlaybackCompleted { name: String, success: bool },

    // Batch Operations
    BatchOperationStarted { operation: String, count: usize },
    BatchOperationProgress { completed: usize, total: usize },
    BatchOperationCompleted { operation: String, success_count: usize, fail_count: usize },

    // Pipelines
    PipelineStarted { pipeline_id: Uuid, name: String },
    PipelineStageStarted { pipeline_id: Uuid, stage: String },
    PipelineStageCompleted { pipeline_id: Uuid, stage: String, success: bool },
    PipelineCompleted { pipeline_id: Uuid, name: String, success: bool },

    // Maintenance
    MaintenanceStarted { task: String },
    MaintenanceCompleted { task: String, success: bool },

    // Platform
    DatabaseMigrated { version: u32, name: String },
    PluginLoaded { plugin_id: String, name: String },
    PluginUnloaded { plugin_id: String },
    PluginError { plugin_id: String, code: String, message: String },
    BackupCreated { name: String, path: String },
    BackupRestored { name: String },
    CrashReported { report_id: String, error_kind: String },
    DiagnosticsExported { bundle_id: String },
}
