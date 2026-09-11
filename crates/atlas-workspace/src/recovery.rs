use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Workspace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub workspace: Workspace,
    pub reason: String,
}

pub struct SessionRecovery {
    snapshots_dir: PathBuf,
    #[allow(dead_code)]
    auto_save_interval_ms: u64,
}

impl SessionRecovery {
    pub fn new(snapshots_dir: PathBuf) -> Self {
        Self {
            snapshots_dir,
            auto_save_interval_ms: 30_000,
        }
    }

    pub fn save_snapshot(&self, workspace: &Workspace, reason: &str) -> Result<(), String> {
        let snapshot = SessionSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            workspace: workspace.clone(),
            reason: reason.to_string(),
        };

        let data = serde_json::to_string_pretty(&snapshot).map_err(|e| e.to_string())?;
        let filename = format!("snapshot_{}.json", snapshot.id);

        std::fs::create_dir_all(&self.snapshots_dir).map_err(|e| e.to_string())?;
        std::fs::write(self.snapshots_dir.join(&filename), data).map_err(|e| e.to_string())?;

        self.prune_snapshots(20)
    }

    pub fn latest_snapshot(&self) -> Option<SessionSnapshot> {
        let mut entries: Vec<_> = std::fs::read_dir(&self.snapshots_dir).ok()?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or(false))
            .collect();

        entries.sort_by_key(|e| e.metadata().ok()?.created().ok());

        let latest = entries.last()?;
        let data = std::fs::read_to_string(latest.path()).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn recover(&self) -> Option<Workspace> {
        self.latest_snapshot().map(|s| s.workspace)
    }

    pub fn clear_snapshots(&self) -> Result<(), String> {
        if self.snapshots_dir.exists() {
            for entry in std::fs::read_dir(&self.snapshots_dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                if entry.path().extension().map(|ext| ext == "json").unwrap_or(false) {
                    std::fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    fn prune_snapshots(&self, max_count: usize) -> Result<(), String> {
        let mut entries: Vec<_> = std::fs::read_dir(&self.snapshots_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or(false))
            .collect();

        entries.sort_by_key(|e| e.metadata().ok()?.created().ok());

        while entries.len() > max_count {
            if let Some(oldest) = entries.first() {
                std::fs::remove_file(oldest.path()).map_err(|e| e.to_string())?;
                entries.remove(0);
            }
        }
        Ok(())
    }
}
