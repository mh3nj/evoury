use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

pub static CLOSE_TO_TRAY: AtomicBool = AtomicBool::new(false);
pub static TRAY_ON_START: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn set_close_to_tray(enabled: bool) {
    CLOSE_TO_TRAY.store(enabled, Ordering::SeqCst);
}

#[tauri::command]
pub fn set_tray_on_start(enabled: bool) {
    TRAY_ON_START.store(enabled, Ordering::SeqCst);
}

#[tauri::command]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    set_autostart_impl(enabled)
}

#[tauri::command]
pub fn hide_main_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_autostart_impl(enabled: bool) -> Result<(), String> {
    use std::process::Command;
    let exe = std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;
    let path = exe.to_string_lossy().to_string();
    if enabled {
        let output = Command::new("reg")
            .arg("add")
            .arg("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .arg("/v")
            .arg("Evoury")
            .arg("/t")
            .arg("REG_SZ")
            .arg("/d")
            .arg(&path)
            .arg("/f")
            .output()
            .map_err(|e| format!("Failed to set autostart: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("reg.exe add failed: {}", stderr));
        }
    } else {
        let output = Command::new("reg")
            .arg("delete")
            .arg("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .arg("/v")
            .arg("Evoury")
            .arg("/f")
            .output()
            .map_err(|e| format!("Failed to remove autostart: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("reg.exe delete failed: {}", stderr));
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn set_autostart_impl(enabled: bool) -> Result<(), String> {
    use std::path::PathBuf;
    let home = std::env::var("HOME").map_err(|_| "Cannot find HOME".to_string())?;
    let plist_path = PathBuf::from(&home).join("Library/LaunchAgents/com.evoury.app.plist");
    if enabled {
        let exe = std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;
        let plist = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.evoury.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
            exe.to_string_lossy()
        );
        std::fs::write(&plist_path, plist).map_err(|e| format!("Failed to write plist: {}", e))?;
    } else {
        let _ = std::fs::remove_file(&plist_path);
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_autostart_impl(enabled: bool) -> Result<(), String> {
    use std::path::PathBuf;
    let home = std::env::var("HOME").map_err(|_| "Cannot find HOME".to_string())?;
    let autostart_dir = PathBuf::from(&home).join(".config/autostart");
    let desktop_path = autostart_dir.join("evoury.desktop");
    if enabled {
        std::fs::create_dir_all(&autostart_dir).map_err(|e| format!("Failed to create autostart dir: {}", e))?;
        let exe = std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;
        let desktop = format!(
            "[Desktop Entry]\nType=Application\nName=Evoury\nExec={}\nTerminal=false\n",
            exe.to_string_lossy()
        );
        std::fs::write(&desktop_path, desktop).map_err(|e| format!("Failed to write .desktop: {}", e))?;
    } else {
        let _ = std::fs::remove_file(&desktop_path);
    }
    Ok(())
}
