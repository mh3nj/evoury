use std::path::{Path, PathBuf};
use std::fs;
use crate::manifest::BackupManifest;
use crate::content::BackupContents;

pub struct BackupManager {
    pub backup_dir: PathBuf,
}

impl BackupManager {
    pub fn new(backup_dir: PathBuf) -> Self {
        fs::create_dir_all(&backup_dir).ok();
        Self { backup_dir }
    }

    pub fn create_backup(
        &self,
        name: &str,
        contents: &BackupContents,
        data_dir: &Path,
    ) -> Result<PathBuf, String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = self.backup_dir.join(format!("{}_{}", name, timestamp));
        fs::create_dir_all(&backup_path).map_err(|e| e.to_string())?;

        let manifest = BackupManifest {
            version: "1.0".into(),
            created: chrono::Utc::now(),
            includes: contents.clone(),
        };

        if contents.workspace {
            let ws_src = data_dir.join("workspace.json");
            if ws_src.exists() {
                fs::copy(&ws_src, backup_path.join("workspace.json")).map_err(|e| e.to_string())?;
            }
        }
        if contents.collections {
            let col_src = data_dir.join("collections.json");
            if col_src.exists() {
                fs::copy(&col_src, backup_path.join("collections.json")).map_err(|e| e.to_string())?;
            }
        }
        if contents.metadata {
            let meta_src = data_dir.join("metadata.json");
            if meta_src.exists() {
                fs::copy(&meta_src, backup_path.join("metadata.json")).map_err(|e| e.to_string())?;
            }
        }
        if contents.baskets {
            let basket_src = data_dir.join("baskets.json");
            if basket_src.exists() {
                fs::copy(&basket_src, backup_path.join("baskets.json")).map_err(|e| e.to_string())?;
            }
        }
        if contents.preferences {
            let pref_src = data_dir.join("preferences.json");
            if pref_src.exists() {
                fs::copy(&pref_src, backup_path.join("preferences.json")).map_err(|e| e.to_string())?;
            }
        }

        let manifest_path = backup_path.join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
        fs::write(&manifest_path, manifest_json).map_err(|e| e.to_string())?;

        Ok(backup_path)
    }

    pub fn restore_backup(&self, backup_path: &Path, data_dir: &Path) -> Result<(), String> {
        let manifest_path = backup_path.join("manifest.json");
        let manifest_json = fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;
        let manifest: BackupManifest = serde_json::from_str(&manifest_json).map_err(|e| e.to_string())?;

        if manifest.includes.workspace {
            let src = backup_path.join("workspace.json");
            if src.exists() {
                fs::copy(&src, data_dir.join("workspace.json")).map_err(|e| e.to_string())?;
            }
        }
        if manifest.includes.collections {
            let src = backup_path.join("collections.json");
            if src.exists() {
                fs::copy(&src, data_dir.join("collections.json")).map_err(|e| e.to_string())?;
            }
        }
        if manifest.includes.metadata {
            let src = backup_path.join("metadata.json");
            if src.exists() {
                fs::copy(&src, data_dir.join("metadata.json")).map_err(|e| e.to_string())?;
            }
        }
        if manifest.includes.baskets {
            let src = backup_path.join("baskets.json");
            if src.exists() {
                fs::copy(&src, data_dir.join("baskets.json")).map_err(|e| e.to_string())?;
            }
        }
        if manifest.includes.preferences {
            let src = backup_path.join("preferences.json");
            if src.exists() {
                fs::copy(&src, data_dir.join("preferences.json")).map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub fn list_backups(&self) -> Result<Vec<PathBuf>, String> {
        let mut backups = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() && entry.path().join("manifest.json").exists() {
                    backups.push(entry.path());
                }
            }
        }
        backups.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
        Ok(backups)
    }

    pub fn load_manifest(&self, backup_path: &Path) -> Result<BackupManifest, String> {
        let path = backup_path.join("manifest.json");
        let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_list_backups() {
        let dir = std::env::temp_dir().join("evoury_backup_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let data_dir = dir.join("data");
        fs::create_dir_all(&data_dir).unwrap();
        fs::write(data_dir.join("preferences.json"), "{}").unwrap();

        let manager = BackupManager::new(dir.join("backups"));
        let contents = BackupContents {
            workspace: false,
            collections: false,
            metadata: false,
            baskets: false,
            preferences: true,
        };

        let result = manager.create_backup("test_backup", &contents, &data_dir);
        assert!(result.is_ok(), "backup should succeed: {:?}", result.err());

        let backups = manager.list_backups().unwrap();
        assert!(!backups.is_empty(), "should have at least one backup");
        assert_eq!(manager.load_manifest(&backups[0]).unwrap().version, "1.0");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_restore_backup() {
        let dir = std::env::temp_dir().join("evoury_restore_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let data_dir = dir.join("data");
        fs::create_dir_all(&data_dir).unwrap();
        fs::write(data_dir.join("preferences.json"), r#"{"theme":"Dark"}"#).unwrap();

        let manager = BackupManager::new(dir.join("backups"));
        let contents = BackupContents {
            workspace: false, collections: false, metadata: false, baskets: false, preferences: true,
        };
        let backup_path = manager.create_backup("restore_test", &contents, &data_dir).unwrap();

        let restore_dir = dir.join("restore");
        fs::create_dir_all(&restore_dir).unwrap();
        manager.restore_backup(&backup_path, &restore_dir).unwrap();

        let restored = fs::read_to_string(restore_dir.join("preferences.json")).unwrap();
        assert_eq!(restored, r#"{"theme":"Dark"}"#);

        let _ = fs::remove_dir_all(&dir);
    }
}
