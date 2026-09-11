use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
    Retina,
}

impl ThumbnailSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ThumbnailSize::Small => (128, 128),
            ThumbnailSize::Medium => (256, 256),
            ThumbnailSize::Large => (512, 512),
            ThumbnailSize::Retina => (1024, 1024),
        }
    }

    pub fn suffix(&self) -> &str {
        match self {
            ThumbnailSize::Small => "_sm",
            ThumbnailSize::Medium => "_md",
            ThumbnailSize::Large => "_lg",
            ThumbnailSize::Retina => "_retina",
        }
    }

    pub fn from_zoom(zoom: f32) -> Self {
        if zoom <= 0.5 { Self::Small }
        else if zoom <= 1.0 { Self::Medium }
        else if zoom <= 2.0 { Self::Large }
        else { Self::Retina }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailEntry {
    pub asset_id: Uuid,
    pub size: ThumbnailSize,
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub source_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct ThumbnailCache {
    entries: HashMap<(Uuid, ThumbnailSize), ThumbnailEntry>,
    cache_dir: PathBuf,
}

impl ThumbnailCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            entries: HashMap::new(),
            cache_dir,
        }
    }

    pub fn get(&self, asset_id: &Uuid, size: ThumbnailSize) -> Option<&ThumbnailEntry> {
        self.entries.get(&(*asset_id, size))
    }

    pub fn insert(&mut self, entry: ThumbnailEntry) {
        self.entries.insert((entry.asset_id, entry.size), entry);
    }

    pub fn remove(&mut self, asset_id: &Uuid) {
        self.entries.retain(|(id, _), _| id != asset_id);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn all(&self) -> Vec<&ThumbnailEntry> {
        self.entries.values().collect()
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn path_for(&self, asset_id: &Uuid, size: ThumbnailSize) -> PathBuf {
        let filename = format!("{}{}.webp", asset_id, size.suffix());
        self.cache_dir.join("thumbnails").join(filename)
    }
}
