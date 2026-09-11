use std::collections::HashMap;
use crate::mode::DetectionMode;
use crate::signature::FileSignature;
use crate::group::DuplicateGroup;
use uuid::Uuid;

pub struct DuplicateDetector;

impl DuplicateDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(
        &self,
        assets: &[(Uuid, String, FileSignature)],
        mode: DetectionMode,
    ) -> Vec<DuplicateGroup> {
        match mode {
            DetectionMode::ExactHash => self.by_hash(assets),
            DetectionMode::Filename => self.by_filename(assets),
            DetectionMode::FileSize => self.by_size(assets),
            DetectionMode::FilenameAndSize => self.by_filename_and_size(assets),
            DetectionMode::All => {
                let mut groups = self.by_hash(assets);
                groups.extend(self.by_filename(assets));
                groups.dedup();
                groups
            }
        }
    }

    fn by_hash(&self, assets: &[(Uuid, String, FileSignature)]) -> Vec<DuplicateGroup> {
        let mut map: HashMap<String, Vec<Uuid>> = HashMap::new();
        for (id, _, sig) in assets {
            map.entry(sig.hash.clone()).or_default().push(*id);
        }
        map.into_values()
            .filter(|ids| ids.len() > 1)
            .map(|ids| DuplicateGroup { id: Uuid::new_v4(), assets: ids })
            .collect()
    }

    fn by_filename(&self, assets: &[(Uuid, String, FileSignature)]) -> Vec<DuplicateGroup> {
        let mut map: HashMap<String, Vec<Uuid>> = HashMap::new();
        for (id, name, _) in assets {
            let stem = std::path::Path::new(name)
                .file_stem()
                .map(|s| s.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            map.entry(stem).or_default().push(*id);
        }
        map.into_values()
            .filter(|ids| ids.len() > 1)
            .map(|ids| DuplicateGroup { id: Uuid::new_v4(), assets: ids })
            .collect()
    }

    fn by_size(&self, assets: &[(Uuid, String, FileSignature)]) -> Vec<DuplicateGroup> {
        let mut map: HashMap<u64, Vec<Uuid>> = HashMap::new();
        for (id, _, sig) in assets {
            map.entry(sig.size).or_default().push(*id);
        }
        map.into_values()
            .filter(|ids| ids.len() > 1)
            .map(|ids| DuplicateGroup { id: Uuid::new_v4(), assets: ids })
            .collect()
    }

    fn by_filename_and_size(&self, assets: &[(Uuid, String, FileSignature)]) -> Vec<DuplicateGroup> {
        let mut map: HashMap<(String, u64), Vec<Uuid>> = HashMap::new();
        for (id, name, sig) in assets {
            let stem = std::path::Path::new(name)
                .file_stem()
                .map(|s| s.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            map.entry((stem, sig.size)).or_default().push(*id);
        }
        map.into_values()
            .filter(|ids| ids.len() > 1)
            .map(|ids| DuplicateGroup { id: Uuid::new_v4(), assets: ids })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_hash() {
        let assets = vec![
            (Uuid::new_v4(), "a.zip".into(), FileSignature { hash: "abc".into(), size: 100 }),
            (Uuid::new_v4(), "b.zip".into(), FileSignature { hash: "abc".into(), size: 100 }),
            (Uuid::new_v4(), "c.zip".into(), FileSignature { hash: "def".into(), size: 200 }),
        ];
        let d = DuplicateDetector::new();
        let groups = d.detect(&assets, DetectionMode::ExactHash);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].assets.len(), 2);
    }
}
