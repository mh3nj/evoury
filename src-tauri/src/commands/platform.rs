use tauri::State;
use crate::state::AppState;
use atlas_diagnostics::report::{CrashReport, SystemInfo};
use atlas_backup::BackupContents;
use atlas_plugin::PluginManifest;

// ── Database ──

#[tauri::command]
pub fn get_db_migrations(state: State<'_, AppState>) -> Result<Vec<(u32, String)>, String> {
    let conn = state.db_connection.lock().map_err(|e| e.to_string())?;
    atlas_db::MigrationRunner::applied_versions(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_db_pending_migrations(state: State<'_, AppState>) -> Result<Vec<u32>, String> {
    let conn = state.db_connection.lock().map_err(|e| e.to_string())?;
    atlas_db::MigrationRunner::pending(&conn).map_err(|e| e.to_string())
}

// ── Plugins ──

#[tauri::command]
pub fn get_plugins(state: State<'_, AppState>) -> Result<Vec<PluginManifest>, String> {
    let reg = state.plugin_registry.lock().map_err(|e| e.to_string())?;
    Ok(reg.all_manifests().into_iter().cloned().collect())
}

#[tauri::command]
pub fn unregister_plugin(state: State<'_, AppState>, plugin_id: String) -> Result<(), String> {
    let mut reg = state.plugin_registry.lock().map_err(|e| e.to_string())?;
    reg.unregister(&plugin_id).map_err(|e| format!("{}: {}", e.code, e.message))
}

// ── Diagnostics ──

#[tauri::command]
pub fn get_crash_reports(state: State<'_, AppState>) -> Result<Vec<CrashReport>, String> {
    let collector = state.diagnostic_collector.lock().map_err(|e| e.to_string())?;
    Ok(collector.crash_reports.clone())
}

#[tauri::command]
pub fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let collector = state.diagnostic_collector.lock().map_err(|e| e.to_string())?;
    Ok(collector.collect_system_info())
}

#[tauri::command]
pub fn export_diagnostic_bundle(state: State<'_, AppState>) -> Result<String, String> {
    let collector = state.diagnostic_collector.lock().map_err(|e| e.to_string())?;
    collector.export_to_json("{}")
}

#[tauri::command]
pub fn report_crash(
    state: State<'_, AppState>,
    error_message: String,
    error_kind: String,
) -> Result<String, String> {
    let report = CrashReport::new("0.1.0", &error_message, &error_kind);
    let id = report.id.clone();
    let mut collector = state.diagnostic_collector.lock().map_err(|e| e.to_string())?;
    collector.add_crash_report(report);
    Ok(id)
}

// ── Backups ──

#[tauri::command]
pub fn list_backups(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mgr = state.backup_manager.lock().map_err(|e| e.to_string())?;
    let backups = mgr.list_backups()?;
    Ok(backups.iter().filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string())).collect())
}

#[tauri::command]
pub fn create_backup(
    state: State<'_, AppState>,
    name: String,
    workspace: bool,
    collections: bool,
    metadata: bool,
    baskets: bool,
    preferences: bool,
) -> Result<String, String> {
    let app_dir = state.app_handle.path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let contents = BackupContents { workspace, collections, metadata, baskets, preferences };
    let mgr = state.backup_manager.lock().map_err(|e| e.to_string())?;
    let path = mgr.create_backup(&name, &contents, &app_dir)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn restore_backup(state: State<'_, AppState>, backup_name: String) -> Result<(), String> {
    let app_dir = state.app_handle.path_resolver()
        .app_data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let mgr = state.backup_manager.lock().map_err(|e| e.to_string())?;
    for p in mgr.list_backups()? {
        if let Some(name) = p.file_name() {
            if name == backup_name.as_str() {
                return mgr.restore_backup(&p, &app_dir);
            }
        }
    }
    Err("Backup not found".to_string())
}
