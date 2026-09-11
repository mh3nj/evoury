use std::path::PathBuf;
use crate::thumbnail::ThumbnailCache;
use crate::entry::CacheEntry;

pub struct CacheManager {
    thumbnail_cache: ThumbnailCache,
    entry_log: Vec<CacheEntry>,
    max_size_mb: u64,
}

impl CacheManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            thumbnail_cache: ThumbnailCache::new(cache_dir),
            entry_log: Vec::new(),
            max_size_mb: 1024,
        }
    }

    pub fn set_max_size_mb(&mut self, mb: u64) {
        self.max_size_mb = mb;
    }

    pub fn thumbnail_cache(&self) -> &ThumbnailCache {
        &self.thumbnail_cache
    }

    pub fn thumbnail_cache_mut(&mut self) -> &mut ThumbnailCache {
        &mut self.thumbnail_cache
    }

    pub fn log_entry(&mut self, entry: CacheEntry) {
        self.entry_log.push(entry);
        if self.entry_log.len() > 1000 {
            self.entry_log.remove(0);
        }
    }

    pub fn entry_log(&self) -> &[CacheEntry] {
        &self.entry_log
    }

    pub fn clear_all(&mut self) {
        self.thumbnail_cache.clear();
        self.entry_log.clear();
    }

    pub fn total_cached(&self) -> usize {
        self.thumbnail_cache.count()
    }
}
