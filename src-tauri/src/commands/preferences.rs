use tauri::State;
use atlas_preferences::Preferences;
use crate::state::AppState;

#[tauri::command]
pub fn get_preferences(state: State<'_, AppState>) -> Result<Preferences, String> {
    let store = state.preferences.lock().map_err(|e| e.to_string())?;
    Ok(store.get().clone())
}

#[tauri::command]
pub fn set_preferences(state: State<'_, AppState>, prefs: Preferences) -> Result<(), String> {
    {
        let mut store = state.preferences.lock().map_err(|e| e.to_string())?;
        store.update(prefs.clone());
    }
    let persistence = state.persistence.lock().map_err(|e| e.to_string())?;
    persistence.save_preferences(&prefs)
}
