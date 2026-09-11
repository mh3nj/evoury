use std::collections::HashMap;
use crate::preset::WorkstationPreset;

#[derive(Debug, Clone, Default)]
pub struct WorkstationRegistry {
    presets: HashMap<String, WorkstationPreset>,
}

impl WorkstationRegistry {
    pub fn new() -> Self {
        let mut presets = HashMap::new();
        for preset in Self::builtins() {
            presets.insert(preset.id.clone(), preset);
        }
        Self { presets }
    }

    pub fn builtins() -> Vec<WorkstationPreset> {
        vec![
            WorkstationPreset::logo_design(),
            WorkstationPreset::packaging(),
            WorkstationPreset::ui_design(),
            WorkstationPreset::photography(),
            WorkstationPreset::archival(),
        ]
    }

    pub fn register(&mut self, preset: WorkstationPreset) {
        self.presets.insert(preset.id.clone(), preset);
    }

    pub fn get(&self, id: &str) -> Option<&WorkstationPreset> {
        self.presets.get(id)
    }

    pub fn all(&self) -> Vec<&WorkstationPreset> {
        self.presets.values().collect()
    }

    pub fn count(&self) -> usize {
        self.presets.len()
    }
}
