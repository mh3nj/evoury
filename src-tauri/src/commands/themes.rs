use crate::state::AppState;
use atlas_preferences::StudioTheme;
use tauri::api::dialog::FileDialogBuilder;
use tauri::State;

#[tauri::command]
pub fn get_themes(state: State<'_, AppState>) -> Result<Vec<StudioTheme>, String> {
    let persistence = state.persistence.lock().map_err(|e| e.to_string())?;
    Ok(persistence.load_themes())
}

#[tauri::command]
pub fn save_theme(state: State<'_, AppState>, theme: StudioTheme) -> Result<(), String> {
    let persistence = state.persistence.lock().map_err(|e| e.to_string())?;
    let mut themes = persistence.load_themes();
    match themes.iter_mut().find(|t| t.id == theme.id) {
        Some(slot) => *slot = theme,
        None => themes.push(theme),
    }
    persistence.save_themes(&themes)
}

#[tauri::command]
pub fn delete_theme(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let persistence = state.persistence.lock().map_err(|e| e.to_string())?;
    let mut themes = persistence.load_themes();
    themes.retain(|t| t.id != id);
    persistence.save_themes(&themes)
}

#[tauri::command]
pub async fn import_theme() -> Result<Option<String>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let (tx, rx) = std::sync::mpsc::channel();
        FileDialogBuilder::new()
            .set_title("Import theme")
            .add_filter("Theme JSON", &["json"])
            .pick_file(move |path| {
                let _ = tx.send(path);
            });
        let path = rx.recv().map_err(|e| e.to_string())?;
        match path {
            Some(p) => std::fs::read_to_string(&p)
                .map(Some)
                .map_err(|e| e.to_string()),
            None => Ok(None),
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn export_theme(theme: StudioTheme) -> Result<Option<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let file_name = format!("{}.theme.json", theme.id);
        let (tx, rx) = std::sync::mpsc::channel();
        FileDialogBuilder::new()
            .set_title("Export theme")
            .set_file_name(&file_name)
            .add_filter("Theme JSON", &["json"])
            .save_file(move |path| {
                let _ = tx.send(path);
            });
        let path = rx.recv().map_err(|e| e.to_string())?;
        match path {
            Some(p) => {
                let content = serde_json::to_string_pretty(&theme).map_err(|e| e.to_string())?;
                std::fs::write(&p, content).map_err(|e| e.to_string())?;
                Ok(Some(p.to_string_lossy().to_string()))
            }
            None => Ok(None),
        }
    })
    .await
    .map_err(|e| e.to_string())?
}
