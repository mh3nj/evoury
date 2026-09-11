use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

pub struct PreviewCache {
    entries: HashMap<Uuid, PathBuf>,
}

impl PreviewCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: Uuid, path: PathBuf) {
        self.entries.insert(id, path);
    }

    pub fn get(&self, id: &Uuid) -> Option<&PathBuf> {
        self.entries.get(id)
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.entries.remove(id);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
