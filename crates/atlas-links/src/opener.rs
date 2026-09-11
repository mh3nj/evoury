use std::process::Command;

pub struct FileOpener;

impl FileOpener {
    pub fn open(path: &str) {
        #[cfg(target_os = "windows")]
        {
            Command::new("explorer").arg(path).spawn().ok();
        }
        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open").arg(path).spawn().ok();
        }
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg(path).spawn().ok();
        }
    }
}
