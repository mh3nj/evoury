use tauri::State;
use uuid::Uuid;
use crate::state::AppState;

#[tauri::command]
pub fn get_collections(state: State<'_, AppState>) -> Vec<atlas_collection::Collection> {
    state.collection_manager.lock().map(|m| m.all()).unwrap_or_default()
}

#[tauri::command]
pub fn add_collection(state: State<'_, AppState>, collection: atlas_collection::Collection) -> Result<(), String> {
    state.collection_manager.lock().map_err(|e| e.to_string())?.add(collection);
    Ok(())
}

#[tauri::command]
pub fn remove_collection(state: State<'_, AppState>, collection_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&collection_id).map_err(|e| e.to_string())?;
    state.collection_manager.lock().map_err(|e| e.to_string())?.remove(&id);
    Ok(())
}

#[tauri::command]
pub fn get_collection_children(state: State<'_, AppState>, collection_id: String) -> Vec<atlas_collection::Collection> {
    let id = Uuid::parse_str(&collection_id).unwrap_or_default();
    state.collection_manager.lock()
        .map(|m| m.children_of(id))
        .unwrap_or_default()
}

#[tauri::command]
pub fn move_collection(state: State<'_, AppState>, collection_id: String, new_parent_id: Option<String>) -> Result<(), String> {
    let id = Uuid::parse_str(&collection_id).map_err(|e| e.to_string())?;
    let parent = new_parent_id.map(|p| Uuid::parse_str(&p)).transpose().map_err(|e| e.to_string())?;
    state.collection_manager.lock().map_err(|e| e.to_string())?.move_to(id, parent)
}

#[tauri::command]
pub fn evaluate_smart_collections(state: State<'_, AppState>) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    let _smart = state.collection_manager.lock().map_err(|e| e.to_string())?;
    Ok(std::collections::HashMap::new())
}
