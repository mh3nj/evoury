use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeLibraryConfig {
    pub chunk_size: usize,
    pub page_size: usize,
    pub prefetch_pages: usize,
    pub max_cache_entries: usize,
    pub enable_lazy_loading: bool,
    pub search_timeout_ms: u64,
    pub batch_scan_size: usize,
    pub thumbnail_batch_size: usize,
}

impl Default for LargeLibraryConfig {
    fn default() -> Self {
        Self {
            chunk_size: 500,
            page_size: 50,
            prefetch_pages: 3,
            max_cache_entries: 10_000,
            enable_lazy_loading: true,
            search_timeout_ms: 500,
            batch_scan_size: 200,
            thumbnail_batch_size: 50,
        }
    }
}

impl LargeLibraryConfig {
    pub fn small() -> Self {
        Self {
            chunk_size: 200,
            page_size: 25,
            prefetch_pages: 2,
            max_cache_entries: 2_000,
            enable_lazy_loading: false,
            batch_scan_size: 100,
            thumbnail_batch_size: 20,
            ..Default::default()
        }
    }

    pub fn medium() -> Self {
        Self::default()
    }

    pub fn large() -> Self {
        Self {
            chunk_size: 1000,
            page_size: 100,
            prefetch_pages: 5,
            max_cache_entries: 50_000,
            enable_lazy_loading: true,
            search_timeout_ms: 1000,
            batch_scan_size: 500,
            thumbnail_batch_size: 100,
        }
    }

    pub fn xlarge() -> Self {
        Self {
            chunk_size: 5000,
            page_size: 250,
            prefetch_pages: 10,
            max_cache_entries: 200_000,
            enable_lazy_loading: true,
            search_timeout_ms: 2000,
            batch_scan_size: 1000,
            thumbnail_batch_size: 200,
        }
    }
}
