use std::collections::HashMap;
use uuid::Uuid;
use crate::metadata::AssetMetadata;

pub struct MetadataStore {
    data: HashMap<Uuid, AssetMetadata>,
}

impl MetadataStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, metadata: AssetMetadata) {
        self.data.insert(metadata.asset_id, metadata);
    }

    pub fn get(&self, id: &Uuid) -> Option<&AssetMetadata> {
        self.data.get(id)
    }

    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut AssetMetadata> {
        self.data.get_mut(id)
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.data.remove(id);
    }

    pub fn all(&self) -> Vec<AssetMetadata> {
        self.data.values().cloned().collect()
    }
}
