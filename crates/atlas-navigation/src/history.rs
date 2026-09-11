use serde::{Deserialize, Serialize};
use crate::location::NavLocation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationEntry {
    pub location: NavLocation,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct NavigationHistory {
    entries: Vec<NavigationEntry>,
    index: usize,
    max_size: usize,
}

impl NavigationHistory {
    pub fn new(max_size: usize) -> Self {
        let root = NavigationEntry {
            location: NavLocation::root(),
            timestamp: chrono::Utc::now(),
        };
        Self {
            entries: vec![root],
            index: 0,
            max_size,
        }
    }

    pub fn push(&mut self, location: NavLocation) {
        // Truncate forward history
        self.entries.truncate(self.index + 1);
        self.entries.push(NavigationEntry {
            location,
            timestamp: chrono::Utc::now(),
        });
        if self.entries.len() > self.max_size {
            self.entries.remove(0);
        }
        self.index = self.entries.len() - 1;
    }

    pub fn back(&mut self) -> Option<&NavLocation> {
        if self.can_go_back() {
            self.index -= 1;
            Some(&self.entries[self.index].location)
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<&NavLocation> {
        if self.can_go_forward() {
            self.index += 1;
            Some(&self.entries[self.index].location)
        } else {
            None
        }
    }

    pub fn current(&self) -> &NavLocation {
        &self.entries[self.index].location
    }

    pub fn can_go_back(&self) -> bool {
        self.index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.index < self.entries.len() - 1
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn position(&self) -> usize {
        self.index
    }

    pub fn entries(&self) -> &[NavigationEntry] {
        &self.entries
    }
}
