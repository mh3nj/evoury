use tauri::State;
use atlas_core::HierarchicalTag;
use crate::state::AppState;

#[tauri::command]
pub fn add_tag(state: State<'_, AppState>, tag: HierarchicalTag) -> Result<(), String> {
    state.tag_manager.lock().map_err(|e| e.to_string())?.add_tag(tag)
}

#[tauri::command]
pub fn remove_tag(state: State<'_, AppState>, tag_id: String) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&tag_id).map_err(|e| e.to_string())?;
    state.tag_manager.lock().map_err(|e| e.to_string())?.remove_tag(id)
}

#[tauri::command]
pub fn get_tag(state: State<'_, AppState>, tag_id: String) -> Option<atlas_core::HierarchicalTag> {
    let id = uuid::Uuid::parse_str(&tag_id).ok()?;
    state.tag_manager.lock().ok()?.get_tag(&id).cloned()
}

#[tauri::command]
pub fn get_all_tags(state: State<'_, AppState>) -> Vec<atlas_core::HierarchicalTag> {
    if let Ok(m) = state.tag_manager.lock() {
        m.all_in_namespace("default").into_iter().cloned().collect()
    } else {
        Vec::new()
    }
}

#[tauri::command]
pub fn update_tag(
    state: State<'_, AppState>,
    tag_id: String, name: Option<String>, color: Option<String>, description: Option<String>
) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&tag_id).map_err(|e| e.to_string())?;
    state.tag_manager.lock().map_err(|e| e.to_string())?
        .update_tag(&id, name.as_deref(), color.as_deref(), description.as_deref())
}

#[tauri::command]
pub fn move_tag(state: State<'_, AppState>, tag_id: String, new_parent_id: Option<String>) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&tag_id).map_err(|e| e.to_string())?;
    let parent = new_parent_id.map(|p| uuid::Uuid::parse_str(&p)).transpose().map_err(|e| e.to_string())?;
    state.tag_manager.lock().map_err(|e| e.to_string())?.move_tag(&id, parent)
}
