use std::thread;
use tauri::Manager;
use atlas_events::{AtlasEvent, EventBus};

pub fn spawn_event_forwarder(app: tauri::AppHandle, event_bus: &EventBus) {
    let receiver = event_bus.subscribe();
    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if let Some(window) = app.get_window("main") {
                let event_name = event_type_name(&event);
                let _ = window.emit(&event_name, event);
            }
        }
    });
}

fn event_type_name(event: &AtlasEvent) -> String {
    match event {
        AtlasEvent::AssetAdded { .. } => "asset-added".into(),
        AtlasEvent::AssetUpdated { .. } => "asset-updated".into(),
        AtlasEvent::AssetRemoved { .. } => "asset-removed".into(),
        AtlasEvent::AssetStateChanged { .. } => "asset-state-changed".into(),
        AtlasEvent::CollectionAdded { .. } => "collection-added".into(),
        AtlasEvent::CollectionRemoved { .. } => "collection-removed".into(),
        AtlasEvent::ScanStarted { .. } => "scan-started".into(),
        AtlasEvent::ScanProgress(_) => "scan-progress".into(),
        AtlasEvent::ScanFinished { .. } => "scan-finished".into(),
        AtlasEvent::ScanError { .. } => "scan-error".into(),
        AtlasEvent::WatcherEvent { .. } => "watcher-event".into(),
        AtlasEvent::IndexProgress { .. } => "index-progress".into(),
        AtlasEvent::IndexFinished => "index-finished".into(),
        AtlasEvent::HealthCheckFinished => "health-check-finished".into(),
        AtlasEvent::MetadataChanged { .. } => "metadata-changed".into(),
        AtlasEvent::BasketChanged => "basket-changed".into(),
        AtlasEvent::WorkspaceChanged { .. } => "workspace-changed".into(),
        AtlasEvent::CommandExecuted { .. } => "command-executed".into(),
        AtlasEvent::CommandRegistered { .. } => "command-registered".into(),
        AtlasEvent::JobQueued { .. } => "job-queued".into(),
        AtlasEvent::JobStarted { .. } => "job-started".into(),
        AtlasEvent::JobCompleted { .. } => "job-completed".into(),
        AtlasEvent::JobFailed { .. } => "job-failed".into(),
        AtlasEvent::PowerModeChanged { .. } => "power-mode-changed".into(),
        AtlasEvent::DiagnosticsUpdated => "diagnostics-updated".into(),
        AtlasEvent::SessionRestored { .. } => "session-restored".into(),
        AtlasEvent::SessionSaved { .. } => "session-saved".into(),
        AtlasEvent::ProviderRegistered { .. } => "provider-registered".into(),
        AtlasEvent::CapabilityRegistered { .. } => "capability-registered".into(),
        AtlasEvent::NotificationCreated { .. } => "notification-created".into(),
        AtlasEvent::UndoPerformed { .. } => "undo-performed".into(),
        AtlasEvent::RedoPerformed { .. } => "redo-performed".into(),
        AtlasEvent::MacroRecordingStarted { .. } => "macro-recording-started".into(),
        AtlasEvent::MacroRecordingStopped { .. } => "macro-recording-stopped".into(),
        AtlasEvent::MacroPlaybackStarted { .. } => "macro-playback-started".into(),
        AtlasEvent::MacroPlaybackCompleted { .. } => "macro-playback-completed".into(),
        AtlasEvent::BatchOperationStarted { .. } => "batch-operation-started".into(),
        AtlasEvent::BatchOperationProgress { .. } => "batch-operation-progress".into(),
        AtlasEvent::BatchOperationCompleted { .. } => "batch-operation-completed".into(),
        AtlasEvent::PipelineStarted { .. } => "pipeline-started".into(),
        AtlasEvent::PipelineStageStarted { .. } => "pipeline-stage-started".into(),
        AtlasEvent::PipelineStageCompleted { .. } => "pipeline-stage-completed".into(),
        AtlasEvent::PipelineCompleted { .. } => "pipeline-completed".into(),
        AtlasEvent::MaintenanceStarted { .. } => "maintenance-started".into(),
        AtlasEvent::MaintenanceCompleted { .. } => "maintenance-completed".into(),
        AtlasEvent::DatabaseMigrated { .. } => "database-migrated".into(),
        AtlasEvent::PluginLoaded { .. } => "plugin-loaded".into(),
        AtlasEvent::PluginUnloaded { .. } => "plugin-unloaded".into(),
        AtlasEvent::PluginError { .. } => "plugin-error".into(),
        AtlasEvent::BackupCreated { .. } => "backup-created".into(),
        AtlasEvent::BackupRestored { .. } => "backup-restored".into(),
        AtlasEvent::CrashReported { .. } => "crash-reported".into(),
        AtlasEvent::DiagnosticsExported { .. } => "diagnostics-exported".into(),
    }
}
