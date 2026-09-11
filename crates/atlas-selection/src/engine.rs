use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Replace,
    Toggle,
    Add,
    Remove,
    Range,
    Invert,
}

#[derive(Debug, Clone)]
pub struct SelectionState {
    pub selected: HashSet<Uuid>,
    pub last_selected: Option<Uuid>,
    pub anchor: Option<Uuid>,
    pub mode: SelectionMode,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            selected: HashSet::new(),
            last_selected: None,
            anchor: None,
            mode: SelectionMode::Replace,
        }
    }
}

pub struct SelectionEngine {
    pub state: SelectionState,
}

impl SelectionEngine {
    pub fn new() -> Self {
        Self {
            state: SelectionState::default(),
        }
    }

    pub fn select(&mut self, id: Uuid, ordered_ids: &[Uuid]) {
        self.apply(id, SelectionMode::Replace, ordered_ids);
    }

    pub fn apply(&mut self, id: Uuid, mode: SelectionMode, ordered_ids: &[Uuid]) {
        self.state.mode = mode;
        match mode {
            SelectionMode::Replace => {
                self.state.selected.clear();
                self.state.selected.insert(id);
                self.state.last_selected = Some(id);
                self.state.anchor = Some(id);
            }
            SelectionMode::Toggle => {
                if self.state.selected.contains(&id) {
                    self.state.selected.remove(&id);
                } else {
                    self.state.selected.insert(id);
                }
                self.state.last_selected = Some(id);
            }
            SelectionMode::Add => {
                self.state.selected.insert(id);
                self.state.last_selected = Some(id);
            }
            SelectionMode::Remove => {
                self.state.selected.remove(&id);
            }
            SelectionMode::Range => {
                if let Some(anchor) = self.state.anchor {
                    let anchor_pos = ordered_ids.iter().position(|x| *x == anchor);
                    let current_pos = ordered_ids.iter().position(|x| *x == id);
                    if let (Some(ap), Some(cp)) = (anchor_pos, current_pos) {
                        let start = ap.min(cp);
                        let end = ap.max(cp);
                        for i in start..=end {
                            self.state.selected.insert(ordered_ids[i]);
                        }
                    }
                }
                self.state.last_selected = Some(id);
            }
            SelectionMode::Invert => {
                let set: HashSet<Uuid> = ordered_ids.iter().copied().collect();
                self.state.selected = set.difference(&self.state.selected).copied().collect();
            }
        }
    }

    pub fn clear(&mut self) {
        self.state.selected.clear();
        self.state.last_selected = None;
        self.state.anchor = None;
    }

    pub fn is_selected(&self, id: &Uuid) -> bool {
        self.state.selected.contains(id)
    }

    pub fn count(&self) -> usize {
        self.state.selected.len()
    }

    pub fn selected_ids(&self) -> Vec<Uuid> {
        self.state.selected.iter().copied().collect()
    }

    pub fn first(&self) -> Option<Uuid> {
        self.state.selected.iter().next().copied()
    }

    pub fn set_anchor(&mut self, id: Uuid) {
        self.state.anchor = Some(id);
    }
}
