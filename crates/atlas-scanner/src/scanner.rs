use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use crossbeam_channel::Sender;
use atlas_core::{
    ArchiveType, Asset, AssetState, Preview,
    scan::{ScanFingerprint, ScanMode, ScanProgress, ScanStats},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use walkdir::WalkDir;

pub struct Scanner {
    scan_id: Uuid,
    mode: ScanMode,
    event_sender: Option<Sender<atlas_events::AtlasEvent>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            scan_id: Uuid::new_v4(),
            mode: ScanMode::FullScan,
            event_sender: None,
        }
    }

    pub fn with_mode(mut self, mode: ScanMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_event_sender(mut self, sender: Sender<atlas_events::AtlasEvent>) -> Self {
        self.event_sender = Some(sender);
        self
    }

    pub fn scan(&self, root: impl AsRef<Path>, fingerprints: &[ScanFingerprint]) -> (Vec<Asset>, ScanStats) {
        let start = Instant::now();
        let root = root.as_ref();

        self.emit(atlas_events::AtlasEvent::ScanStarted {
            scan_id: self.scan_id,
            mode: format!("{:?}", self.mode),
            root: root.to_path_buf(),
        });

        let mut archives: HashMap<String, PathBuf> = HashMap::new();
        let mut previews: HashMap<String, PathBuf> = HashMap::new();
        let mut errors: Vec<String> = Vec::new();
        let mut scanned_archives = 0u64;
        let mut scanned_previews = 0u64;

        let is_incremental = matches!(self.mode, ScanMode::IncrementalScan | ScanMode::StartupScan);

        let entries: Vec<_> = WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();

        let total = entries.len() as u64;

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let parent = path.parent().unwrap_or(root);

            if is_incremental {
                if let Some(fp) = fingerprints.iter().find(|f| f.folder == parent) {
                    let meta = match entry.metadata() {
                        Ok(m) => m,
                        Err(_) => continue,
                    };
                    let modified = match meta.modified() {
                        Ok(t) => t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                        Err(_) => 0,
                    };
                    if modified <= fp.last_modified && meta.len() <= fp.total_size / std::cmp::max(fp.file_count, 1) {
                        continue;
                    }
                }
            }

            let stem = match path.file_stem() {
                Some(v) => v.to_string_lossy().to_string(),
                None => continue,
            };
            let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();

            match ext.as_str() {
                "zip" | "rar" | "7z" | "tar" | "tgz" | "gz" => {
                    if ext == "gz" && !stem.ends_with(".tar") { continue; }
                    archives.entry(stem.clone()).or_insert(path.to_path_buf());
                    scanned_archives += 1;
                }
                "avif" | "png" | "jpg" | "jpeg" | "webp" => {
                    previews.entry(stem.clone()).or_insert(path.to_path_buf());
                    scanned_previews += 1;
                }
                _ => {}
            }

            if i % 100 == 0 {
                self.emit(atlas_events::AtlasEvent::ScanProgress(ScanProgress {
                    scan_id: self.scan_id,
                    mode: self.mode,
                    total_files: total,
                    scanned_files: i as u64 + 1,
                    archives_found: scanned_archives,
                    previews_found: scanned_previews,
                    assets_created: 0,
                    errors: errors.clone(),
                    current_file: Some(path.to_string_lossy().to_string()),
                }));
            }
        }

        let mut assets = Vec::new();
        for (name, archive_path) in &archives {
            let metadata = match fs::metadata(archive_path) {
                Ok(m) => m,
                Err(e) => {
                    errors.push(format!("Failed to read metadata for {}: {}", archive_path.display(), e));
                    continue;
                }
            };
            let modified: DateTime<Utc> = match metadata.modified() {
                Ok(t) => t.into(),
                Err(_) => Utc::now(),
            };
            let preview = previews.get(name).map(|path| Preview { path: path.clone() });

            assets.push(Asset {
                id: Uuid::new_v4(),
                name: name.clone(),
                archive_type: archive_type(archive_path),
                archive_path: archive_path.clone(),
                preview,
                file_size: metadata.len(),
                modified_at: modified,
                collection_id: None,
                category_id: None,
                state: AssetState::Ready,
            });
        }

        let duration = start.elapsed().as_millis() as u64;
        let stats = ScanStats {
            total_archives: archives.len() as u64,
            total_previews: previews.len() as u64,
            paired: assets.iter().filter(|a| a.has_preview()).count() as u64,
            missing_previews: assets.iter().filter(|a| !a.has_preview()).count() as u64,
            errors: errors.len() as u64,
            duration_ms: duration,
        };

        self.emit(atlas_events::AtlasEvent::ScanFinished { scan_id: self.scan_id });

        (assets, stats)
    }

    pub fn build_fingerprints(&self, root: impl AsRef<Path>) -> Vec<ScanFingerprint> {
        let mut fingerprints = Vec::new();
        let mut folder_map: HashMap<PathBuf, (u64, u64, u64)> = HashMap::new();

        for entry in WalkDir::new(root.as_ref()).follow_links(false).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
            let path = entry.path().to_path_buf();
            let parent = path.parent().unwrap_or(root.as_ref()).to_path_buf();
            let meta = match entry.metadata() { Ok(m) => m, Err(_) => continue };
            let modified = match meta.modified() {
                Ok(t) => t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                Err(_) => 0,
            };
            let entry = folder_map.entry(parent).or_insert((0, 0, 0));
            entry.0 += 1;
            entry.1 += meta.len();
            entry.2 = std::cmp::max(entry.2, modified);
        }

        for (folder, (file_count, total_size, last_modified)) in folder_map {
            fingerprints.push(ScanFingerprint {
                folder,
                file_count,
                total_size,
                last_modified,
            });
        }

        fingerprints
    }

    fn emit(&self, event: atlas_events::AtlasEvent) {
        if let Some(ref sender) = self.event_sender {
            let _ = sender.send(event);
        }
    }
}

fn archive_type(path: &Path) -> ArchiveType {
    match path.extension().unwrap_or_default().to_string_lossy().to_lowercase().as_str() {
        "zip" => ArchiveType::Zip,
        "rar" => ArchiveType::Rar,
        "7z" => ArchiveType::SevenZip,
        "tar" | "tgz" | "gz" => ArchiveType::Tar,
        _ => ArchiveType::Unknown,
    }
}
