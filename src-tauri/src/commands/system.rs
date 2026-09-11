use std::time::Instant;
use tauri::State;
use std::sync::Mutex;
use atlas_health::{DiagnosticsCollector, DiagnosticsSnapshot};
use crate::state::AppState;

static START_TIME: Mutex<Option<Instant>> = Mutex::new(None);

fn uptime_seconds() -> u64 {
    let mut start = START_TIME.lock().unwrap();
    let now = Instant::now();
    match *start {
        Some(s) => now.duration_since(s).as_secs(),
        None => {
            *start = Some(now);
            0
        }
    }
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Evoury.", name)
}

#[tauri::command]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn get_diagnostics(state: State<'_, AppState>) -> Result<DiagnosticsSnapshot, String> {
    let collector = DiagnosticsCollector::new();

    let (worker_count, active_count) = {
        (1, state.scheduler.active_workers())
    };
    let (pending_jobs, running_jobs) = {
        (state.scheduler.pending_count(), state.scheduler.running_count())
    };
    let registered_commands = state.command_registry.lock().map_err(|e| e.to_string())?.count();
    let asset_count = state.library.len();
    let cache_entries = 0;
    let power_mode = format!("{:?}", state.sleep_manager.lock().map_err(|e| e.to_string())?.mode());

    Ok(collector.collect(
        worker_count,
        active_count,
        pending_jobs,
        running_jobs,
        registered_commands,
        asset_count,
        cache_entries,
        &power_mode,
        uptime_seconds(),
    ))
}

#[tauri::command]
pub fn get_power_mode(state: State<'_, AppState>) -> Result<String, String> {
    let mode = state.sleep_manager.lock().map_err(|e| e.to_string())?.mode();
    Ok(format!("{:?}", mode))
}

#[tauri::command]
pub fn touch_activity(state: State<'_, AppState>) -> Result<(), String> {
    state.sleep_manager.lock().map_err(|e| e.to_string())?.touch();
    Ok(())
}

#[tauri::command]
pub fn open_external_file(path: String) -> Result<(), String> {
    use std::process::Command;
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {}", path));
    }
    // Use `shell.open` equivalent — open with default system app
    #[cfg(target_os = "windows")]
    {
        // `start "" "path with spaces"` — empty title, then the path quoted
        Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg("")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn open_file_location(path: String) -> Result<(), String> {
    use std::process::Command;
    let p = std::path::Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {}", path));
    }
    #[cfg(target_os = "windows")]
    {
        // /select highlights the specific file in explorer
        Command::new("explorer")
            .arg("/select,")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open location: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        // `open -R` reveals the file in Finder
        Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open location: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        let parent = p.parent().unwrap_or(p);
        Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| format!("Failed to open location: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn save_session(state: State<'_, AppState>) -> Result<(), String> {
    let ws = state.workspace_manager.lock().map_err(|e| e.to_string())?;
    let workspace = ws.load_active();
    let recovery = state.session_recovery.lock().map_err(|e| e.to_string())?;
    recovery.save_snapshot(&workspace, "manual")?;
    Ok(())
}

#[tauri::command]
pub fn recover_session(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let recovery = state.session_recovery.lock().map_err(|e| e.to_string())?;
    match recovery.recover() {
        Some(ws) => {
            let data = serde_json::to_string(&ws).map_err(|e| e.to_string())?;
            Ok(Some(data))
        }
        None => Ok(None),
    }
}
