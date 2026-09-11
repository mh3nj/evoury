use tauri::State;
use uuid::Uuid;
use atlas_core::MetadataTemplate;
use crate::state::AppState;

#[tauri::command]
pub fn add_template(state: State<'_, AppState>, template: MetadataTemplate) -> Result<(), String> {
    state.template_manager.lock().map_err(|e| e.to_string())?.add_template(template);
    Ok(())
}

#[tauri::command]
pub fn get_templates(state: State<'_, AppState>) -> Vec<MetadataTemplate> {
    state.template_manager.lock()
        .map(|m| m.all_templates().into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn remove_template(state: State<'_, AppState>, template_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&template_id).map_err(|e| e.to_string())?;
    state.template_manager.lock().map_err(|e| e.to_string())?.remove_template(&id);
    Ok(())
}
