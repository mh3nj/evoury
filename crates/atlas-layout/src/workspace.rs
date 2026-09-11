use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::dock::DockConfig;
use crate::panel::PanelConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dock: DockConfig,
    pub panels: Vec<PanelConfig>,
    pub focus_mode: bool,
    pub sidebar_sections: Vec<String>,
}

impl Default for LayoutProfile {
    fn default() -> Self {
        Self {
            id: "default".into(),
            name: "Default".into(),
            description: "Standard layout with sidebar, gallery, and inspector".into(),
            dock: DockConfig::default(),
            panels: vec![],
            focus_mode: false,
            sidebar_sections: vec![
                "libraries".into(),
                "collections".into(),
                "smart_collections".into(),
                "tags".into(),
                "history".into(),
                "trash".into(),
            ],
        }
    }
}

impl LayoutProfile {
    pub fn minimal() -> Self {
        Self {
            id: "minimal".into(),
            name: "Minimal".into(),
            description: "Gallery-only view with floating panels".into(),
            dock: DockConfig {
                show_borders: false,
                allow_docking: false,
                ..Default::default()
            },
            panels: vec![],
            focus_mode: false,
            sidebar_sections: vec![],
        }
    }

    pub fn presentation() -> Self {
        Self {
            id: "presentation".into(),
            name: "Presentation".into(),
            description: "Fullscreen presentation layout".into(),
            dock: DockConfig::default(),
            panels: vec![],
            focus_mode: true,
            sidebar_sections: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LayoutRegistry {
    profiles: HashMap<String, LayoutProfile>,
}

impl LayoutRegistry {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        let default = LayoutProfile::default();
        profiles.insert(default.id.clone(), default);
        let minimal = LayoutProfile::minimal();
        profiles.insert(minimal.id.clone(), minimal);
        let presentation = LayoutProfile::presentation();
        profiles.insert(presentation.id.clone(), presentation);
        Self { profiles }
    }

    pub fn register(&mut self, profile: LayoutProfile) {
        self.profiles.insert(profile.id.clone(), profile);
    }

    pub fn get(&self, id: &str) -> Option<&LayoutProfile> {
        self.profiles.get(id)
    }

    pub fn all(&self) -> Vec<&LayoutProfile> {
        self.profiles.values().collect()
    }
}
