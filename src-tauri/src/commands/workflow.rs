use tauri::State;
use uuid::Uuid;
use crate::state::AppState;

// ── Undo/Redo ──

#[tauri::command]
pub fn push_undo(state: State<'_, AppState>, label: String, undo_data: String, redo_data: String) -> Result<(), String> {
    state.undo_stack.lock().map_err(|e| e.to_string())?.push(&label, &undo_data, &redo_data);
    Ok(())
}

#[tauri::command]
pub fn undo(state: State<'_, AppState>) -> Result<String, String> {
    let entry = state.undo_stack.lock().map_err(|e| e.to_string())?.undo()?;
    Ok(entry.undo_data)
}

#[tauri::command]
pub fn redo(state: State<'_, AppState>) -> Result<String, String> {
    let entry = state.undo_stack.lock().map_err(|e| e.to_string())?.redo()?;
    Ok(entry.redo_data)
}

#[tauri::command]
pub fn can_undo(state: State<'_, AppState>) -> bool {
    state.undo_stack.lock().map(|s| s.can_undo()).unwrap_or(false)
}

#[tauri::command]
pub fn can_redo(state: State<'_, AppState>) -> bool {
    state.undo_stack.lock().map(|s| s.can_redo()).unwrap_or(false)
}

// ── Batch Rename ──

#[tauri::command]
pub fn batch_rename_preview(state: State<'_, AppState>, asset_ids: Vec<String>, names: Vec<String>, pattern: atlas_batch::RenamePattern) -> Result<Vec<atlas_batch::RenamePreview>, String> {
    let ids: Vec<Uuid> = asset_ids.iter().map(|s| Uuid::parse_str(s)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let name_pairs: Vec<(Uuid, String)> = ids.iter().cloned().zip(names.into_iter()).collect();
    let custom_fields = std::collections::HashMap::new();
    let mgr = state.batch_renamer.lock().map_err(|e| e.to_string())?;
    Ok(mgr.preview(&ids, &name_pairs, &pattern, &custom_fields))
}

#[tauri::command]
pub fn batch_rename_apply(state: State<'_, AppState>, previews: Vec<atlas_batch::RenamePreview>) -> Result<Vec<(String, String)>, String> {
    let results = state.batch_renamer.lock().map_err(|e| e.to_string())?.apply(&previews)?;
    Ok(results.into_iter().map(|(id, name)| (id.to_string(), name)).collect())
}

// ── Batch Metadata ──

#[tauri::command]
pub fn batch_set_tags(state: State<'_, AppState>, asset_ids: Vec<String>, add_tags: Vec<String>, remove_tags: Vec<String>) -> Result<Vec<(String, Vec<String>)>, String> {
    let ids: Vec<Uuid> = asset_ids.iter().map(|s| Uuid::parse_str(s)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let existing = std::collections::HashMap::new();
    let results = state.batch_metadata_editor.lock().map_err(|e| e.to_string())?.set_tags_batch(&ids, &add_tags, &remove_tags, &existing);
    Ok(results.into_iter().map(|(id, tags)| (id.to_string(), tags)).collect())
}

#[tauri::command]
pub fn batch_set_rating(state: State<'_, AppState>, asset_ids: Vec<String>, rating: u8) -> Result<Vec<(String, u8)>, String> {
    let ids: Vec<Uuid> = asset_ids.iter().map(|s| Uuid::parse_str(s)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let results = state.batch_metadata_editor.lock().map_err(|e| e.to_string())?.set_rating_batch(&ids, rating);
    Ok(results.into_iter().map(|(id, r)| (id.to_string(), r)).collect())
}

#[tauri::command]
pub fn batch_set_favorite(state: State<'_, AppState>, asset_ids: Vec<String>, favorite: bool) -> Result<Vec<(String, bool)>, String> {
    let ids: Vec<Uuid> = asset_ids.iter().map(|s| Uuid::parse_str(s)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let results = state.batch_metadata_editor.lock().map_err(|e| e.to_string())?.set_favorite_batch(&ids, favorite);
    Ok(results.into_iter().map(|(id, f)| (id.to_string(), f)).collect())
}

// ── Pipelines ──

#[tauri::command]
pub fn create_pipeline(state: State<'_, AppState>, name: String, description: String) -> Result<atlas_pipeline::Pipeline, String> {
    Ok(state.pipeline_manager.lock().map_err(|e| e.to_string())?.create_pipeline(&name, &description))
}

#[tauri::command]
pub fn get_pipelines(state: State<'_, AppState>) -> Vec<atlas_pipeline::Pipeline> {
    state.pipeline_manager.lock().map(|m| m.all_pipelines().into_iter().cloned().collect()).unwrap_or_default()
}

#[tauri::command]
pub fn remove_pipeline(state: State<'_, AppState>, pipeline_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&pipeline_id).map_err(|e| e.to_string())?;
    state.pipeline_manager.lock().map_err(|e| e.to_string())?.remove_pipeline(&id);
    Ok(())
}

#[tauri::command]
pub fn add_pipeline_stage(state: State<'_, AppState>, pipeline_id: String, stage: atlas_pipeline::PipelineStage) -> Result<(), String> {
    let id = Uuid::parse_str(&pipeline_id).map_err(|e| e.to_string())?;
    let mut mgr = state.pipeline_manager.lock().map_err(|e| e.to_string())?;
    if let Some(p) = mgr.get_pipeline_mut(&id) { p.add_stage(stage); }
    Ok(())
}

// ── Macros ──

#[tauri::command]
pub fn start_macro_recording(state: State<'_, AppState>, name: String) -> Result<(), String> {
    state.macro_recorder.lock().map_err(|e| e.to_string())?.start_recording(&name)
}

#[tauri::command]
pub fn stop_macro_recording(state: State<'_, AppState>) -> Result<atlas_macro::MacroRecording, String> {
    state.macro_recorder.lock().map_err(|e| e.to_string())?.stop_recording()
}

#[tauri::command]
pub fn record_macro_step(state: State<'_, AppState>, command_id: String, args: String) -> Result<(), String> {
    state.macro_recorder.lock().map_err(|e| e.to_string())?.record_step(&command_id, &args)
}

#[tauri::command]
pub fn is_recording(state: State<'_, AppState>) -> bool {
    state.macro_recorder.lock().map(|r| r.is_recording()).unwrap_or(false)
}

#[tauri::command]
pub fn get_macros(state: State<'_, AppState>) -> Vec<atlas_macro::MacroRecording> {
    state.macro_recorder.lock().map(|r| r.all_recordings().clone()).unwrap_or_default()
}

#[tauri::command]
pub fn remove_macro(state: State<'_, AppState>, macro_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&macro_id).map_err(|e| e.to_string())?;
    state.macro_recorder.lock().map_err(|e| e.to_string())?.remove_recording(&id);
    Ok(())
}

// ── Notifications ──

#[tauri::command]
pub fn get_notifications(state: State<'_, AppState>) -> Vec<atlas_notification::Notification> {
    state.notification_manager.lock().map(|m| m.all().into_iter().cloned().collect()).unwrap_or_default()
}

#[tauri::command]
pub fn get_unread_notifications(state: State<'_, AppState>) -> Vec<atlas_notification::Notification> {
    state.notification_manager.lock().map(|m| m.unread().into_iter().cloned().collect()).unwrap_or_default()
}

#[tauri::command]
pub fn get_unread_count(state: State<'_, AppState>) -> usize {
    state.notification_manager.lock().map(|m| m.unread_count()).unwrap_or(0)
}

#[tauri::command]
pub fn mark_notification_read(state: State<'_, AppState>, notification_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&notification_id).map_err(|e| e.to_string())?;
    state.notification_manager.lock().map_err(|e| e.to_string())?.mark_read(&id);
    Ok(())
}

#[tauri::command]
pub fn mark_all_notifications_read(state: State<'_, AppState>) -> Result<(), String> {
    state.notification_manager.lock().map_err(|e| e.to_string())?.mark_all_read();
    Ok(())
}

#[tauri::command]
pub fn dismiss_notification(state: State<'_, AppState>, notification_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&notification_id).map_err(|e| e.to_string())?;
    state.notification_manager.lock().map_err(|e| e.to_string())?.dismiss(&id);
    Ok(())
}

#[tauri::command]
pub fn clear_notifications(state: State<'_, AppState>) -> Result<(), String> {
    state.notification_manager.lock().map_err(|e| e.to_string())?.clear_all();
    Ok(())
}

// ── Maintenance ──

#[tauri::command]
pub fn run_maintenance(state: State<'_, AppState>, task_name: String) -> Result<atlas_health::MaintenanceReport, String> {
    let task = match task_name.to_lowercase().as_str() {
        "vacuum" => atlas_health::MaintenanceTask::VacuumOrphanedMetadata,
        "rebuild_previews" => atlas_health::MaintenanceTask::RebuildMissingPreviews,
        "reindex" => atlas_health::MaintenanceTask::ReindexAll,
        "cleanup_versions" => atlas_health::MaintenanceTask::CleanupOldVersions { keep: 10 },
        "prune_activity" => atlas_health::MaintenanceTask::PruneActivityLog { older_than_days: 90 },
        "compact_cache" => atlas_health::MaintenanceTask::CompactCache,
        "verify_files" => atlas_health::MaintenanceTask::VerifyFileIntegrity,
        "repair_refs" => atlas_health::MaintenanceTask::RepairBrokenReferences,
        _ => return Err(format!("Unknown maintenance task: {}", task_name)),
    };
    Ok(state.maintenance_engine.lock().map_err(|e| e.to_string())?.run_task(&task))
}

#[tauri::command]
pub fn run_all_maintenance(state: State<'_, AppState>) -> Vec<atlas_health::MaintenanceReport> {
    state.maintenance_engine.lock().map(|mut m| m.run_all()).unwrap_or_default()
}

#[tauri::command]
pub fn get_maintenance_reports(state: State<'_, AppState>) -> Vec<atlas_health::MaintenanceReport> {
    state.maintenance_engine.lock().map(|m| m.reports().clone()).unwrap_or_default()
}

#[tauri::command]
pub fn get_maintenance_schedule(state: State<'_, AppState>) -> atlas_health::MaintenanceSchedule {
    state.maintenance_engine.lock().map(|m| m.schedule().clone()).unwrap_or_default()
}

#[tauri::command]
pub fn set_maintenance_schedule(state: State<'_, AppState>, schedule: atlas_health::MaintenanceSchedule) -> Result<(), String> {
    *state.maintenance_engine.lock().map_err(|e| e.to_string())?.schedule_mut() = schedule;
    Ok(())
}
