use std::collections::HashMap;
use std::sync::RwLock;
use atlas_core::Asset;
use uuid::Uuid;

pub struct Library {
    assets: RwLock<HashMap<Uuid, Asset>>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            assets: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, asset: Asset) {
        if let Ok(mut map) = self.assets.write() {
            map.insert(asset.id, asset);
        }
    }

    pub fn remove(&self, id: &Uuid) {
        if let Ok(mut map) = self.assets.write() {
            map.remove(id);
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<Asset> {
        if let Ok(map) = self.assets.read() {
            map.get(id).cloned()
        } else {
            None
        }
    }

    pub fn all(&self) -> Vec<Asset> {
        if let Ok(map) = self.assets.read() {
            map.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        if let Ok(map) = self.assets.read() {
            map.len()
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&self) {
        if let Ok(mut map) = self.assets.write() {
            map.clear();
        }
    }
}
