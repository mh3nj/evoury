use crate::state::AppState;
use atlas_core::scan::ScanMode;
use std::path::Path;
use std::time::Duration;
use tauri::{Manager, State};
use uuid::Uuid;

/// Scan `path`, adding it as a library folder first if it is not one yet.
///
/// Kept for older callers; new code uses `add_library_root` / `scan_roots`
/// (see `commands::catalog`).
#[tauri::command]
pub async fn scan_library(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
    mode: Option<String>,
) -> Result<usize, String> {
    let scan_mode = match mode.as_deref() {
        Some("incremental") => ScanMode::IncrementalScan,
        Some("startup") => ScanMode::StartupScan,
        Some("background") => ScanMode::BackgroundScan,
        Some("verify") => ScanMode::VerificationScan,
        _ => ScanMode::FullScan,
    };
    let root = Path::new(&path);
    if let Err(e) = app.asset_protocol_scope().allow_directory(root, true) {
        eprintln!("could not allow previews from {}: {e}", root.display());
    }
    state.catalog.add_and_scan(root, scan_mode)?;
    Ok(state.library.len())
}

/// What is inside an asset's archive (read from Kaelovun's sidecar, else listed with
/// 7-Zip). Nothing is extracted. Takes an asset id, never a path, so the webview can't
/// point the lister at arbitrary files.
#[tauri::command]
pub async fn get_archive_contents(
    state: State<'_, AppState>,
    asset_id: String,
) -> Result<atlas_scanner::ArchiveContents, String> {
    let id = Uuid::parse_str(&asset_id).map_err(|e| format!("Invalid asset id: {e}"))?;
    let asset = state
        .library
        .get(&id)
        .ok_or_else(|| "That asset is no longer in the library.".to_string())?;
    atlas_scanner::read_contents(
        &asset.archive_path,
        atlas_scanner::contents::DEFAULT_MAX_ENTRIES,
    )
}

#[tauri::command]
pub fn start_watching(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let sender = state.event_bus.sender();
    let mut watcher = state.watcher.lock().map_err(|e| e.to_string())?;
    watcher.start(path, sender, Duration::from_secs(2));
    Ok(())
}

#[tauri::command]
pub fn stop_watching(state: State<'_, AppState>) -> Result<(), String> {
    let mut watcher = state.watcher.lock().map_err(|e| e.to_string())?;
    watcher.stop();
    Ok(())
}

#[tauri::command]
pub fn is_watching(state: State<'_, AppState>) -> bool {
    match state.watcher.lock() {
        Ok(w) => w.is_running(),
        Err(_) => false,
    }
}

#[tauri::command]
pub fn asset_count(state: State<'_, AppState>) -> usize {
    state.library.len()
}

#[tauri::command]
pub fn get_assets(state: State<'_, AppState>) -> Vec<atlas_core::Asset> {
    state.library.all()
}
