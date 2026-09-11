use tauri::State;
use atlas_metadata::AssetMetadata;
use crate::state::AppState;

fn save_metadata(state: &AppState) {
    if let Ok(store) = state.metadata_store.lock() {
        let mut map = std::collections::HashMap::new();
        for m in store.all() {
            map.insert(m.asset_id, m);
        }
        if let Ok(persistence) = state.persistence.lock() {
            let _ = persistence.save_metadata(&map);
        }
    }
}

#[tauri::command]
pub fn get_metadata(state: State<'_, AppState>, asset_id: String) -> Result<AssetMetadata, String> {
    let id = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    Ok(store.get(&id).cloned().unwrap_or_else(|| AssetMetadata {
        asset_id: id,
        ..Default::default()
    }))
}

#[tauri::command]
pub fn update_metadata(state: State<'_, AppState>, metadata: AssetMetadata) -> Result<(), String> {
    let mut store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    store.insert(metadata);
    drop(store);
    save_metadata(&state);
    Ok(())
}

#[tauri::command]
pub fn set_tags(state: State<'_, AppState>, asset_id: String, tags: Vec<String>) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let mut store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    let mut meta = store.get(&id).cloned().unwrap_or_else(|| AssetMetadata {
        asset_id: id,
        ..Default::default()
    });
    meta.tags = tags;
    store.insert(meta);
    drop(store);
    save_metadata(&state);
    Ok(())
}

#[tauri::command]
pub fn set_rating(state: State<'_, AppState>, asset_id: String, rating: u8) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let mut store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    let mut meta = store.get(&id).cloned().unwrap_or_else(|| AssetMetadata {
        asset_id: id,
        ..Default::default()
    });
    meta.rating = rating.min(5);
    store.insert(meta);
    drop(store);
    save_metadata(&state);
    Ok(())
}

#[tauri::command]
pub fn set_favorite(state: State<'_, AppState>, asset_id: String, favorite: bool) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let mut store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    let mut meta = store.get(&id).cloned().unwrap_or_else(|| AssetMetadata {
        asset_id: id,
        ..Default::default()
    });
    meta.favorite = favorite;
    store.insert(meta);
    drop(store);
    save_metadata(&state);
    Ok(())
}

#[tauri::command]
pub fn set_notes(state: State<'_, AppState>, asset_id: String, notes: String) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let mut store = state.metadata_store.lock().map_err(|e| e.to_string())?;
    let mut meta = store.get(&id).cloned().unwrap_or_else(|| AssetMetadata {
        asset_id: id,
        ..Default::default()
    });
    meta.notes = notes;
    store.insert(meta);
    drop(store);
    save_metadata(&state);
    Ok(())
}
