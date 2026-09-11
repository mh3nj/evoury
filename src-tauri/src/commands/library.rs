use std::path::Path;
use std::time::Duration;
use tauri::State;
use atlas_core::scan::ScanMode;
use atlas_scanner::Scanner;
use crate::state::AppState;

#[tauri::command]
pub fn scan_library(
    state: State<'_, AppState>,
    path: String,
    mode: Option<String>,
) -> Result<usize, String> {
    let scan_mode = match mode.as_deref() {
        Some("incremental") => ScanMode::IncrementalScan,
        Some("startup") => ScanMode::StartupScan,
        Some("folder") => ScanMode::FolderScan,
        Some("background") => ScanMode::BackgroundScan,
        Some("verify") => ScanMode::VerificationScan,
        _ => ScanMode::FullScan,
    };

    let sender = state.event_bus.sender();
    let scanner = Scanner::new()
        .with_mode(scan_mode)
        .with_event_sender(sender);

    let (assets, _stats) = scanner.scan(Path::new(&path), &[]);

    for asset in assets {
        state.library.insert(asset);
    }

    Ok(state.library.len())
}

#[tauri::command]
pub fn start_watching(
    state: State<'_, AppState>,
    path: String,
) -> Result<(), String> {
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
