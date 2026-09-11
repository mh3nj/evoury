use tauri::State;
use atlas_core::Asset;
use atlas_search::{SearchEngine, query::parse_query, result::SearchResult};
use crate::state::AppState;

#[tauri::command]
pub fn search_assets(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    let parsed = parse_query(&query);
    let assets = state.library.all();
    let metadata = state.metadata_store.lock().map_err(|e| e.to_string())?.all();
    let results = SearchEngine::search(&assets, &metadata, &parsed);
    Ok(results)
}

#[tauri::command]
pub fn suggest_assets(state: State<'_, AppState>, prefix: String) -> Vec<String> {
    let assets = state.library.all();
    SearchEngine::suggest(&assets, &prefix, 10)
}

#[tauri::command]
pub fn get_asset_batch(state: State<'_, AppState>, ids: Vec<String>) -> Vec<Asset> {
    let assets = state.library.all();
    assets
        .into_iter()
        .filter(|a| ids.iter().any(|id| *id == a.id.to_string()))
        .collect()
}
