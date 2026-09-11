use std::{fs, path::PathBuf};
use crate::workspace::Workspace;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceList {
    pub workspaces: Vec<Workspace>,
    pub active_id: Uuid,
}

impl WorkspaceList {
    pub fn new() -> Self {
        let default = Workspace::new("Default");
        Self {
            active_id: default.id,
            workspaces: vec![default],
        }
    }

    pub fn active(&self) -> Option<&Workspace> {
        self.workspaces.iter().find(|w| w.id == self.active_id)
    }

    pub fn active_mut(&mut self) -> Option<&mut Workspace> {
        let id = self.active_id;
        self.workspaces.iter_mut().find(|w| w.id == id)
    }

    pub fn switch_to(&mut self, id: Uuid) -> Option<&Workspace> {
        if self.workspaces.iter().any(|w| w.id == id) {
            self.active_id = id;
        }
        self.active()
    }

    pub fn add(&mut self, name: &str) -> Uuid {
        let ws = Workspace::new(name);
        let id = ws.id;
        self.workspaces.push(ws);
        id
    }

    pub fn remove(&mut self, id: Uuid) -> bool {
        if self.workspaces.len() <= 1 {
            return false;
        }
        if let Some(pos) = self.workspaces.iter().position(|w| w.id == id) {
            self.workspaces.remove(pos);
            if self.active_id == id {
                self.active_id = self.workspaces[0].id;
            }
            true
        } else {
            false
        }
    }

    pub fn rename(&mut self, id: Uuid, name: &str) {
        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.id == id) {
            ws.name = name.to_string();
        }
    }
}

use serde::{Deserialize, Serialize};

pub struct WorkspaceManager {
    path: PathBuf,
}

impl WorkspaceManager {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load_all(&self) -> WorkspaceList {
        if !self.path.exists() {
            return WorkspaceList::new();
        }
        fs::read_to_string(&self.path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_else(WorkspaceList::new)
    }

    pub fn save_all(&self, list: &WorkspaceList) -> Result<(), String> {
        let data = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
        fs::write(&self.path, data).map_err(|e| e.to_string())
    }

    pub fn load_active(&self) -> Workspace {
        self.load_all().active().cloned().unwrap_or_else(|| Workspace::new("Default"))
    }

    pub fn save(&self, workspace: &Workspace) -> Result<(), String> {
        let mut list = self.load_all();
        if let Some(existing) = list.workspaces.iter_mut().find(|w| w.id == workspace.id) {
            *existing = workspace.clone();
        }
        self.save_all(&list)
    }

    pub fn reset(&self) -> Workspace {
        let ws = Workspace::new("Default");
        let mut list = WorkspaceList::new();
        list.workspaces = vec![ws.clone()];
        list.active_id = ws.id;
        let _ = self.save_all(&list);
        ws
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
