use std::path::Path;
use crate::types::FileType;

pub fn detect(path: &str) -> FileType {
    let p = Path::new(path);
    if p.is_dir() {
        return FileType::Folder;
    }
    match p.extension().and_then(|e| e.to_str()) {
        Some("zip") | Some("rar") | Some("7z") => FileType::Archive,
        Some("png") | Some("jpg") | Some("jpeg") | Some("avif") => FileType::Image,
        Some("psd") | Some("ai") | Some("indd") => FileType::DesignFile,
        _ => FileType::Unknown,
    }
}
