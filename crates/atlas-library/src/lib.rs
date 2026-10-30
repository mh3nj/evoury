use atlas_core::Asset;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use uuid::Uuid;

/// What a [`Library::sync`] changed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SyncReport {
    pub added: usize,
    pub updated: usize,
    pub removed: usize,
}

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

    /// Bring the library in line with a fresh scan of the library folder `root_id`.
    ///
    /// * assets that are new are added, and ones that already exist (same stable id)
    ///   are refreshed in place, keeping the collection and category they were given
    /// * when `prune` is set, assets of this library folder that the scan no longer found
    ///   are removed (deleted, renamed or moved archives). Pass `false` for partial scans
    ///   (a single sub-folder), which only see part of the tree. Other library folders are
    ///   never touched.
    ///
    /// Scanning twice therefore never duplicates anything.
    pub fn sync(&self, scanned: Vec<Asset>, root_id: Uuid, prune: bool) -> SyncReport {
        let mut report = SyncReport::default();
        let Ok(mut map) = self.assets.write() else {
            return report;
        };

        let seen: HashSet<Uuid> = scanned.iter().map(|a| a.id).collect();

        for mut asset in scanned {
            match map.get(&asset.id) {
                Some(existing) => {
                    asset.collection_id = asset.collection_id.or(existing.collection_id);
                    asset.category_id = asset.category_id.or(existing.category_id);
                    report.updated += 1;
                }
                None => report.added += 1,
            }
            map.insert(asset.id, asset);
        }

        if prune {
            let gone: Vec<Uuid> = map
                .values()
                .filter(|a| a.root_id == root_id && !seen.contains(&a.id))
                .map(|a| a.id)
                .collect();
            for id in gone {
                map.remove(&id);
                report.removed += 1;
            }
        }
        report
    }

    /// Assets that belong to one library folder.
    pub fn by_root(&self, root_id: Uuid) -> Vec<Asset> {
        self.assets
            .read()
            .map(|m| {
                m.values()
                    .filter(|a| a.root_id == root_id)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove every asset of one library folder; returns how many there were.
    pub fn remove_root(&self, root_id: Uuid) -> usize {
        let Ok(mut map) = self.assets.write() else {
            return 0;
        };
        let before = map.len();
        map.retain(|_, a| a.root_id != root_id);
        before - map.len()
    }

    pub fn clear(&self) {
        if let Ok(mut map) = self.assets.write() {
            map.clear();
        }
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_core::{ArchiveType, AssetState};
    use chrono::Utc;
    use std::path::PathBuf;

    fn asset(id: u128, path: &str) -> Asset {
        asset_in(Uuid::nil(), id, path)
    }

    fn asset_in(root_id: Uuid, id: u128, path: &str) -> Asset {
        Asset {
            id: Uuid::from_u128(id),
            name: path.to_string(),
            archive_type: ArchiveType::SevenZip,
            archive_path: PathBuf::from(path),
            preview: None,
            file_size: 1,
            modified_at: Utc::now(),
            collection_id: None,
            category_id: None,
            state: AssetState::Ready,
            root_id,
        }
    }

    #[test]
    fn scanning_twice_does_not_duplicate() {
        let lib = Library::new();
        let root = Uuid::nil();
        let r1 = lib.sync(
            vec![asset(1, "/lib/a.7z"), asset(2, "/lib/b.7z")],
            root,
            true,
        );
        let r2 = lib.sync(
            vec![asset(1, "/lib/a.7z"), asset(2, "/lib/b.7z")],
            root,
            true,
        );
        assert_eq!((r1.added, r2.added, r2.updated), (2, 0, 2));
        assert_eq!(lib.len(), 2);
    }

    #[test]
    fn collection_and_category_survive_a_rescan() {
        let lib = Library::new();
        let root = Uuid::nil();
        lib.sync(vec![asset(1, "/lib/a.7z")], root, true);
        let mut assigned = lib.get(&Uuid::from_u128(1)).unwrap();
        assigned.collection_id = Some(Uuid::from_u128(77));
        assigned.category_id = Some(Uuid::from_u128(88));
        lib.insert(assigned);
        lib.sync(vec![asset(1, "/lib/a.7z")], root, true);
        let a = lib.get(&Uuid::from_u128(1)).unwrap();
        assert_eq!(
            (a.collection_id, a.category_id),
            (Some(Uuid::from_u128(77)), Some(Uuid::from_u128(88)))
        );
    }

    #[test]
    fn vanished_files_are_pruned_only_within_the_scanned_library() {
        let lib = Library::new();
        let other = Uuid::from_u128(5);
        lib.sync(
            vec![asset(1, "/lib/a.7z"), asset(2, "/lib/gone.7z")],
            Uuid::nil(),
            true,
        );
        lib.sync(vec![asset_in(other, 3, "/elsewhere/c.7z")], other, true);
        let r = lib.sync(vec![asset(1, "/lib/a.7z")], Uuid::nil(), true);
        assert_eq!(r.removed, 1);
        assert!(lib.get(&Uuid::from_u128(2)).is_none());
        assert!(lib.get(&Uuid::from_u128(3)).is_some());
    }

    #[test]
    fn libraries_can_be_listed_and_removed_one_by_one() {
        let lib = Library::new();
        let other = Uuid::from_u128(5);
        lib.sync(vec![asset(1, "/lib/a.7z")], Uuid::nil(), true);
        lib.sync(
            vec![asset_in(other, 2, "/o/b.7z"), asset_in(other, 3, "/o/c.7z")],
            other,
            true,
        );
        assert_eq!(lib.by_root(other).len(), 2);
        assert_eq!(lib.remove_root(other), 2);
        assert_eq!(lib.len(), 1);
        assert_eq!(lib.remove_root(other), 0);
    }

    #[test]
    fn partial_scans_never_prune() {
        let lib = Library::new();
        lib.sync(
            vec![asset(1, "/lib/a.7z"), asset(2, "/lib/b.7z")],
            Uuid::nil(),
            true,
        );
        let r = lib.sync(vec![asset(1, "/lib/a.7z")], Uuid::nil(), false);
        assert_eq!(r.removed, 0);
        assert_eq!(lib.len(), 2);
    }

    #[test]
    fn a_converted_archive_keeps_its_asset() {
        let lib = Library::new();
        let root = Uuid::nil();
        lib.sync(vec![asset(1, "/lib/p.rar")], root, true);
        lib.sync(vec![asset(1, "/lib/p.7z")], root, true);
        assert_eq!(lib.len(), 1);
        assert!(lib
            .get(&Uuid::from_u128(1))
            .unwrap()
            .archive_path
            .ends_with("p.7z"));
    }
}
