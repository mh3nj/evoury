use tauri::State;
use uuid::Uuid;
use atlas_core::VersionRecord;
use crate::state::AppState;

#[tauri::command]
pub fn commit_version(state: State<'_, AppState>, asset_id: String, label: String, file_path: String, file_size: u64) -> Result<VersionRecord, String> {
    let id = Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    state.version_history.lock()
        .map_err(|e| e.to_string())
        .map(|mut v| v.commit_version(id, &label, &file_path, file_size))
}

#[tauri::command]
pub fn get_version_history(state: State<'_, AppState>, asset_id: String) -> Vec<VersionRecord> {
    let id = Uuid::parse_str(&asset_id).unwrap_or_default();
    state.version_history.lock()
        .map(|v| v.get_history(&id).into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn restore_version(state: State<'_, AppState>, asset_id: String, version_id: String) -> Option<VersionRecord> {
    let aid = Uuid::parse_str(&asset_id).ok()?;
    let vid = Uuid::parse_str(&version_id).ok()?;
    state.version_history.lock().ok()?.restore(&aid, &vid)
}

#[tauri::command]
pub fn delete_version(state: State<'_, AppState>, version_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&version_id).map_err(|e| e.to_string())?;
    state.version_history.lock().map_err(|e| e.to_string())?.delete_version(&id);
    Ok(())
}

#[tauri::command]
pub fn get_latest_version(state: State<'_, AppState>, asset_id: String) -> Option<VersionRecord> {
    let id = Uuid::parse_str(&asset_id).ok()?;
    state.version_history.lock().ok()?.latest_version(&id).cloned()
}
