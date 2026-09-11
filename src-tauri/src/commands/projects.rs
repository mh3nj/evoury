use tauri::State;
use uuid::Uuid;
use crate::state::AppState;

#[tauri::command]
pub fn create_project(state: State<'_, AppState>, name: String) -> Result<atlas_core::Project, String> {
    let mut mgr = state.project_manager.lock().map_err(|e| e.to_string())?;
    Ok(mgr.create_project(&name))
}

#[tauri::command]
pub fn get_project(state: State<'_, AppState>, project_id: String) -> Option<atlas_core::Project> {
    let id = Uuid::parse_str(&project_id).ok()?;
    state.project_manager.lock().ok()?.get_project(&id).cloned()
}

#[tauri::command]
pub fn get_all_projects(state: State<'_, AppState>) -> Vec<atlas_core::Project> {
    state.project_manager.lock()
        .map(|m| m.all_projects().into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn remove_project(state: State<'_, AppState>, project_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    state.project_manager.lock().map_err(|e| e.to_string())?.remove_project(&id)
}

#[tauri::command]
pub fn add_asset_to_project(state: State<'_, AppState>, project_id: String, asset_id: String) -> Result<(), String> {
    let pid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let aid = Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    state.project_manager.lock().map_err(|e| e.to_string())?.add_asset_to_project(&pid, aid)
}

#[tauri::command]
pub fn remove_asset_from_project(state: State<'_, AppState>, project_id: String, asset_id: String) -> Result<(), String> {
    let pid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let aid = Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    state.project_manager.lock().map_err(|e| e.to_string())?.remove_asset_from_project(&pid, &aid)
}

#[tauri::command]
pub fn get_project_assets(state: State<'_, AppState>, project_id: String) -> Vec<String> {
    let id = Uuid::parse_str(&project_id).unwrap_or_default();
    state.project_manager.lock()
        .map(|m| m.assets_in_project(&id).into_iter().map(|u| u.to_string()).collect())
        .unwrap_or_default()
}
