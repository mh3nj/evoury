use std::collections::HashMap;
use atlas_core::VersionRecord;
use uuid::Uuid;

pub struct VersionManager {
    versions: HashMap<Uuid, Vec<VersionRecord>>,
}

impl VersionManager {
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    pub fn add_version(&mut self, asset_id: &Uuid, record: VersionRecord) -> Result<(), String> {
        let list = self.versions.entry(*asset_id).or_default();
        if list.iter().any(|v| v.id == record.id) {
            return Err(format!("Version with id {} already exists", record.id));
        }
        list.push(record);
        Ok(())
    }

    pub fn get_versions(&self, asset_id: &Uuid) -> Vec<&VersionRecord> {
        let mut versions: Vec<&VersionRecord> = self
            .versions
            .get(asset_id)
            .map(|list| list.iter().collect())
            .unwrap_or_default();
        versions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        versions
    }

    pub fn get_latest_version(&self, asset_id: &Uuid) -> Option<&VersionRecord> {
        let versions = self.get_versions(asset_id);
        versions.into_iter().next()
    }

    pub fn get_version(&self, id: &Uuid) -> Option<&VersionRecord> {
        self.versions
            .values()
            .flat_map(|list| list.iter())
            .find(|v| v.id == *id)
    }

    pub fn remove_version(&mut self, id: &Uuid) {
        for list in self.versions.values_mut() {
            list.retain(|v| v.id != *id);
        }
        self.versions.retain(|_, list| !list.is_empty());
    }

    pub fn restore_version(&mut self, asset_id: &Uuid, version_id: &Uuid) -> Option<VersionRecord> {
        let old = self.versions.get(asset_id)?.iter().find(|v| v.id == *version_id)?.clone();
        let restored = VersionRecord {
            id: Uuid::new_v4(),
            asset_id: *asset_id,
            label: format!("Restored from v{}", old.label),
            description: old.description.clone(),
            previous_version_id: Some(old.id),
            file_path: old.file_path.clone(),
            file_size: old.file_size,
            created_at: chrono::Utc::now(),
        };
        self.versions.entry(*asset_id).or_default().push(restored.clone());
        Some(restored)
    }

    pub fn get_version_history(&self, asset_id: &Uuid) -> Vec<&VersionRecord> {
        self.get_versions(asset_id)
    }

    pub fn count_versions(&self, asset_id: &Uuid) -> usize {
        self.versions
            .get(asset_id)
            .map(|list| list.len())
            .unwrap_or(0)
    }
}
