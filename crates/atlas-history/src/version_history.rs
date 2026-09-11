use atlas_core::VersionRecord;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VersionHistory {
    pub history: HashMap<Uuid, Vec<VersionRecord>>,
}

impl VersionHistory {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    pub fn commit_version(
        &mut self,
        asset_id: Uuid,
        label: &str,
        file_path: &str,
        file_size: u64,
    ) -> VersionRecord {
        let previous = self
            .history
            .get(&asset_id)
            .and_then(|v| v.last())
            .map(|v| v.id);

        let mut record = VersionRecord::new(asset_id, label, file_path, file_size);
        record.previous_version_id = previous;

        self.history.entry(asset_id).or_default().push(record.clone());
        record
    }

    pub fn get_history(&self, asset_id: &Uuid) -> Vec<&VersionRecord> {
        self.history
            .get(asset_id)
            .map(|v| v.iter().rev().collect())
            .unwrap_or_default()
    }

    pub fn get_version(&self, version_id: &Uuid) -> Option<&VersionRecord> {
        self.history
            .values()
            .flatten()
            .find(|v| &v.id == version_id)
    }

    pub fn restore(&mut self, asset_id: &Uuid, version_id: &Uuid) -> Option<VersionRecord> {
        let record = self
            .history
            .get(asset_id)?
            .iter()
            .find(|v| &v.id == version_id)?
            .clone();

        let restored = self.commit_version(
            *asset_id,
            "Restored",
            &record.file_path,
            record.file_size,
        );
        Some(restored)
    }

    pub fn add_external_version(
        &mut self,
        asset_id: Uuid,
        label: &str,
        file_path: &str,
        file_size: u64,
    ) -> VersionRecord {
        let record = VersionRecord::new(asset_id, label, file_path, file_size);
        self.history.entry(asset_id).or_default().push(record.clone());
        record
    }

    pub fn delete_version(&mut self, version_id: &Uuid) {
        for versions in self.history.values_mut() {
            versions.retain(|v| &v.id != version_id);
        }
    }

    pub fn version_count(&self, asset_id: &Uuid) -> usize {
        self.history
            .get(asset_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    pub fn prune_versions(&mut self, asset_id: &Uuid, keep: usize) {
        if let Some(versions) = self.history.get_mut(asset_id) {
            if versions.len() > keep {
                let new_len = versions.len() - keep;
                versions.drain(..new_len);
            }
        }
    }

    pub fn latest_version(&self, asset_id: &Uuid) -> Option<&VersionRecord> {
        self.history.get(asset_id).and_then(|v| v.last())
    }
}
