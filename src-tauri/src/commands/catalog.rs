//! Library folders, the persistent index and duplicate analysis.
//!
//! These commands are deliberately thin: the work is done (and tested) in `atlas-catalog`.
//! The long ones are `async` so they run off the UI thread.

use crate::state::AppState;
use atlas_catalog::{AnalyzeProgress, DuplicateReport, RootInfo, ScanSummary};
use atlas_core::scan::ScanMode;
use atlas_duplicate::SharedOptions;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::{Manager, State};
use uuid::Uuid;

fn parse_mode(mode: Option<&str>) -> ScanMode {
    match mode {
        Some("incremental") => ScanMode::IncrementalScan,
        Some("startup") => ScanMode::StartupScan,
        Some("background") => ScanMode::BackgroundScan,
        Some("verify") => ScanMode::VerificationScan,
        _ => ScanMode::FullScan,
    }
}

fn parse_id(id: &str) -> Result<Uuid, String> {
    Uuid::parse_str(id).map_err(|e| format!("Invalid library id: {e}"))
}

/// Let the webview load preview images from a library folder.
fn allow_previews(app: &tauri::AppHandle, path: &Path) {
    if let Err(e) = app.asset_protocol_scope().allow_directory(path, true) {
        eprintln!("could not allow previews from {}: {e}", path.display());
    }
}

#[tauri::command]
pub async fn list_library_roots(state: State<'_, AppState>) -> Result<Vec<RootInfo>, String> {
    state.catalog.roots()
}

/// Add a library folder and (by default) scan it.
#[tauri::command]
pub async fn add_library_root(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
    name: Option<String>,
    scan: Option<bool>,
) -> Result<RootInfo, String> {
    let root = state.catalog.add_root(Path::new(&path), name.as_deref())?;
    allow_previews(&app, &root.path);
    if scan.unwrap_or(true) {
        state.catalog.scan_root(root.id, ScanMode::FullScan)?;
    }
    state
        .catalog
        .roots()?
        .into_iter()
        .find(|r| r.id == root.id)
        .ok_or_else(|| "The library folder was not saved.".to_string())
}

/// Remove a library folder from Evoury. Files on disk are never touched.
#[tauri::command]
pub async fn remove_library_root(state: State<'_, AppState>, id: String) -> Result<usize, String> {
    state.catalog.remove_root(parse_id(&id)?)
}

#[tauri::command]
pub async fn rename_library_root(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> Result<(), String> {
    state.catalog.rename_root(parse_id(&id)?, &name)
}

/// Scan every library folder (or just `root_id`). `mode` is `full`, `incremental`,
/// `startup`, `background` or `verify`; the incremental modes only re-list folders that
/// changed since the last scan.
#[tauri::command]
pub async fn scan_roots(
    state: State<'_, AppState>,
    mode: Option<String>,
    root_id: Option<String>,
) -> Result<ScanSummary, String> {
    let mode = parse_mode(mode.as_deref());
    match root_id {
        Some(id) => state.catalog.scan_root(parse_id(&id)?, mode),
        None => state.catalog.scan_all(mode),
    }
}

/// Watch every library folder for changes.
#[tauri::command]
pub async fn watch_libraries(state: State<'_, AppState>) -> Result<usize, String> {
    let paths: Vec<PathBuf> = state
        .catalog
        .roots()?
        .into_iter()
        .map(|r| PathBuf::from(r.path))
        .collect();
    let count = paths.len();
    let sender = state.event_bus.sender();
    let mut watcher = state.watcher.lock().map_err(|e| e.to_string())?;
    watcher.stop();
    if !paths.is_empty() {
        watcher.start_many(paths, sender, Duration::from_secs(5));
    }
    Ok(count)
}

/// Read what is inside up to `limit` more archives (needed before shared files and similar
/// packages can be found). Call until `remaining` reaches 0.
#[tauri::command]
pub async fn analyze_package_contents(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<AnalyzeProgress, String> {
    state.catalog_cancel.store(false, Ordering::SeqCst);
    let cancel = state.catalog_cancel.clone();
    state
        .catalog
        .analyze_contents(limit.unwrap_or(50), &|| cancel.load(Ordering::SeqCst))
}

/// Byte-identical archives, files shared between packages and near-identical packages.
#[tauri::command]
pub async fn find_duplicates(state: State<'_, AppState>) -> Result<DuplicateReport, String> {
    state.catalog_cancel.store(false, Ordering::SeqCst);
    let cancel = state.catalog_cancel.clone();
    state
        .catalog
        .duplicate_report(&SharedOptions::default(), &|| cancel.load(Ordering::SeqCst))
}

#[tauri::command]
pub fn cancel_catalog_task(state: State<'_, AppState>) {
    state.catalog_cancel.store(true, Ordering::SeqCst);
}
