use tauri::State;
use uuid::Uuid;
use atlas_core::AutoClassifyRule;
use crate::state::AppState;

#[tauri::command]
pub fn add_classify_rule(state: State<'_, AppState>, rule: AutoClassifyRule) -> Result<(), String> {
    state.auto_classifier.lock().map_err(|e| e.to_string())?.add_rule(rule);
    Ok(())
}

#[tauri::command]
pub fn get_classify_rules(state: State<'_, AppState>) -> Vec<AutoClassifyRule> {
    state.auto_classifier.lock()
        .map(|c| c.all_rules().into_iter().cloned().collect())
        .unwrap_or_default()
}

#[tauri::command]
pub fn remove_classify_rule(state: State<'_, AppState>, rule_id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&rule_id).map_err(|e| e.to_string())?;
    state.auto_classifier.lock().map_err(|e| e.to_string())?.remove_rule(&id);
    Ok(())
}

#[tauri::command]
pub fn classify_asset(state: State<'_, AppState>, name: String, mime_type: Option<String>, extension: Option<String>) -> Vec<AutoClassifyRule> {
    state.auto_classifier.lock()
        .map(|c| c.classify(&name, mime_type.as_deref(), extension.as_deref()).into_iter().cloned().collect())
        .unwrap_or_default()
}
