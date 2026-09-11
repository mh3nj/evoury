use tauri::State;
use uuid::Uuid;
use atlas_core::{SavedSearch, SearchFolder};
use crate::state::AppState;

#[tauri::command]
pub fn save_search(state: State<'_, AppState>, name: String, query_text: String) -> Result<SavedSearch, String> {
    Ok(state.saved_search_manager.lock().map_err(|e| e.to_string())?.save_search(&name, &query_text))
}

#[tauri::command]
pub fn get_saved_searches(state: State<'_, AppState>) -> Vec<SavedSearch> {
    state.saved_search_manager.lock()
        .map(|m| m.all_searches().into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn remove_saved_search(state: State<'_, AppState>, search_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&search_id).map_err(|e| e.to_string())?;
    state.saved_search_manager.lock().map_err(|e| e.to_string())?.remove_search(&id);
    Ok(())
}

#[tauri::command]
pub fn create_search_folder(state: State<'_, AppState>, name: String) -> Result<SearchFolder, String> {
    Ok(state.saved_search_manager.lock().map_err(|e| e.to_string())?.create_folder(&name))
}

#[tauri::command]
pub fn get_search_folders(state: State<'_, AppState>) -> Vec<SearchFolder> {
    state.saved_search_manager.lock()
        .map(|m| m.all_folders().into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn move_search_to_folder(state: State<'_, AppState>, search_id: String, folder_id: Option<String>) -> Result<(), String> {
    let sid = Uuid::parse_str(&search_id).map_err(|e| e.to_string())?;
    let fid = folder_id.map(|f| Uuid::parse_str(&f)).transpose().map_err(|e| e.to_string())?;
    state.saved_search_manager.lock().map_err(|e| e.to_string())?.move_search_to_folder(&sid, fid)
}
