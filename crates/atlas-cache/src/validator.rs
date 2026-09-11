use crate::entry::CacheEntry;

pub struct CacheValidator;

impl CacheValidator {
    pub fn valid(entry: &CacheEntry, current_hash: &str) -> bool {
        entry.source_hash == current_hash
    }
}
