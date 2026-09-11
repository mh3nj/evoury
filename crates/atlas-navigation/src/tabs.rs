use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::location::NavLocation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabEntry {
    pub id: Uuid,
    pub title: String,
    pub location: NavLocation,
    pub pinned: bool,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct TabManager {
    tabs: Vec<TabEntry>,
    active_id: Uuid,
    max_tabs: usize,
}

impl TabManager {
    pub fn new() -> Self {
        let root_tab = TabEntry {
            id: Uuid::new_v4(),
            title: "All Assets".into(),
            location: NavLocation::root(),
            pinned: true,
            modified: false,
        };
        let active_id = root_tab.id;
        Self {
            tabs: vec![root_tab],
            active_id,
            max_tabs: 20,
        }
    }

    pub fn open(&mut self, location: NavLocation) -> Uuid {
        // Reuse existing tab for same location title
        if let Some(existing) = self.tabs.iter_mut().find(|t| t.title == location.title) {
            existing.location = location;
            self.active_id = existing.id;
            return existing.id;
        }

        if self.tabs.len() >= self.max_tabs {
            // Close least-recently-used non-pinned tab
            if let Some(pos) = self.tabs.iter().position(|t| !t.pinned) {
                self.tabs.remove(pos);
            }
        }

        let id = Uuid::new_v4();
        self.tabs.push(TabEntry {
            id,
            title: location.title.clone(),
            location,
            pinned: false,
            modified: false,
        });
        self.active_id = id;
        id
    }

    pub fn close(&mut self, id: Uuid) {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            if self.tabs[pos].pinned { return; }
            self.tabs.remove(pos);
            if self.active_id == id {
                self.active_id = self.tabs.last().map(|t| t.id).unwrap_or_default();
            }
        }
    }

    pub fn activate(&mut self, id: Uuid) {
        if self.tabs.iter().any(|t| t.id == id) {
            self.active_id = id;
        }
    }

    pub fn active(&self) -> Option<&TabEntry> {
        self.tabs.iter().find(|t| t.id == self.active_id)
    }

    pub fn active_mut(&mut self) -> Option<&mut TabEntry> {
        let id = self.active_id;
        self.tabs.iter_mut().find(|t| t.id == id)
    }

    pub fn all(&self) -> &[TabEntry] {
        &self.tabs
    }

    pub fn count(&self) -> usize {
        self.tabs.len()
    }
}
