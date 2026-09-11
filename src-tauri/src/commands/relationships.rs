use tauri::State;
use uuid::Uuid;
use atlas_core::Relationship;
use crate::state::AppState;

#[tauri::command]
pub fn add_relationship(state: State<'_, AppState>, relationship: Relationship) -> Result<(), String> {
    state.relationship_manager.lock().map_err(|e| e.to_string())?.add_relationship(relationship)
}

#[tauri::command]
pub fn remove_relationship(state: State<'_, AppState>, relationship_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&relationship_id).map_err(|e| e.to_string())?;
    state.relationship_manager.lock().map_err(|e| e.to_string())?.remove_relationship(&id);
    Ok(())
}

#[tauri::command]
pub fn get_relationships_for_asset(state: State<'_, AppState>, asset_id: String) -> Vec<Relationship> {
    let id = Uuid::parse_str(&asset_id).unwrap_or_default();
    state.relationship_manager.lock()
        .map(|m| m.relationships_for_asset(&id).into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_related_assets(state: State<'_, AppState>, asset_id: String) -> Vec<String> {
    let id = Uuid::parse_str(&asset_id).unwrap_or_default();
    state.relationship_manager.lock()
        .map(|m| m.related_asset_ids(&id).into_iter().map(|u| u.to_string()).collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_dependency_chain(state: State<'_, AppState>, asset_id: String) -> Vec<String> {
    let id = Uuid::parse_str(&asset_id).unwrap_or_default();
    state.dependency_graph.lock()
        .map(|g| g.transitive_dependencies(&id).into_iter().map(|u| u.to_string()).collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn get_dependent_chain(state: State<'_, AppState>, asset_id: String) -> Vec<String> {
    let id = Uuid::parse_str(&asset_id).unwrap_or_default();
    state.dependency_graph.lock()
        .map(|g| g.transitive_dependents(&id).into_iter().map(|u| u.to_string()).collect())
        .unwrap_or_default()
}
