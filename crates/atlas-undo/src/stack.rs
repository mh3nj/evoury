use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UndoError = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoEntry {
    pub id: Uuid,
    pub label: String,
    pub redo_data: String,
    pub undo_data: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct UndoStack {
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
    max_size: usize,
    current_label: String,
}

impl UndoStack {
    pub fn new(max_size: usize) -> Self {
        Self { undo_stack: Vec::new(), redo_stack: Vec::new(), max_size, current_label: String::new() }
    }

    pub fn push(&mut self, label: &str, undo_data: &str, redo_data: &str) {
        let entry = UndoEntry {
            id: Uuid::new_v4(),
            label: label.to_string(),
            redo_data: redo_data.to_string(),
            undo_data: undo_data.to_string(),
            timestamp: chrono::Utc::now(),
        };
        self.undo_stack.push(entry);
        self.redo_stack.clear();
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
        self.current_label = label.to_string();
    }

    pub fn undo(&mut self) -> Result<UndoEntry, UndoError> {
        self.undo_stack.pop().ok_or_else(|| "Nothing to undo".to_string()).map(|entry| {
            self.redo_stack.push(entry.clone());
            entry
        })
    }

    pub fn redo(&mut self) -> Result<UndoEntry, UndoError> {
        self.redo_stack.pop().ok_or_else(|| "Nothing to redo".to_string()).map(|entry| {
            self.undo_stack.push(entry.clone());
            entry
        })
    }

    pub fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }
    pub fn undo_count(&self) -> usize { self.undo_stack.len() }
    pub fn redo_count(&self) -> usize { self.redo_stack.len() }
    pub fn current_label(&self) -> &str { &self.current_label }

    pub fn peek_undo(&self) -> Option<&UndoEntry> { self.undo_stack.last() }
    pub fn peek_redo(&self) -> Option<&UndoEntry> { self.redo_stack.last() }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    pub fn set_max_size(&mut self, max: usize) { self.max_size = max; }

    pub fn all_undo_entries(&self) -> Vec<&UndoEntry> { self.undo_stack.iter().rev().collect() }
    pub fn all_redo_entries(&self) -> Vec<&UndoEntry> { self.redo_stack.iter().rev().collect() }
}
