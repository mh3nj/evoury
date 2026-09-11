use std::collections::HashMap;
use crate::shortcut::{KeyCombo, ShortcutEntry};
use crate::category::ShortcutCategory;


#[derive(Debug, Clone)]
pub struct ShortcutRegistry {
    shortcuts: HashMap<String, ShortcutEntry>,
}

impl ShortcutRegistry {
    pub fn new() -> Self {
        let mut registry = Self { shortcuts: HashMap::new() };
        registry.register_defaults();
        registry
    }

    pub fn register_defaults(&mut self) {
        let defaults: Vec<ShortcutEntry> = vec![
            // Navigation
            ShortcutEntry {
                id: "nav.command-palette".into(), name: "Command Palette".into(),
                description: "Open the command palette".into(), category: ShortcutCategory::Navigation,
                default_combo: KeyCombo::ctrl("k"), user_combo: None,
                command: "command-palette.open".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "nav.back".into(), name: "Go Back".into(),
                description: "Navigate to previous location".into(), category: ShortcutCategory::Navigation,
                default_combo: KeyCombo::ctrl("["), user_combo: None,
                command: "navigation.back".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "nav.forward".into(), name: "Go Forward".into(),
                description: "Navigate to next location".into(), category: ShortcutCategory::Navigation,
                default_combo: KeyCombo::ctrl("]"), user_combo: None,
                command: "navigation.forward".into(), scope: "global".into(), enabled: true,
            },
            // Selection
            ShortcutEntry {
                id: "sel.select-all".into(), name: "Select All".into(),
                description: "Select all visible assets".into(), category: ShortcutCategory::Selection,
                default_combo: KeyCombo::ctrl("a"), user_combo: None,
                command: "selection.select-all".into(), scope: "gallery".into(), enabled: true,
            },
            ShortcutEntry {
                id: "sel.invert".into(), name: "Invert Selection".into(),
                description: "Invert current selection".into(), category: ShortcutCategory::Selection,
                default_combo: KeyCombo::ctrl_shift("i"), user_combo: None,
                command: "selection.invert".into(), scope: "gallery".into(), enabled: true,
            },
            ShortcutEntry {
                id: "sel.clear".into(), name: "Clear Selection".into(),
                description: "Deselect all assets".into(), category: ShortcutCategory::Selection,
                default_combo: KeyCombo::new("Escape"), user_combo: None,
                command: "selection.clear".into(), scope: "gallery".into(), enabled: true,
            },
            // View
            ShortcutEntry {
                id: "view.toggle-sidebar".into(), name: "Toggle Sidebar".into(),
                description: "Show or hide the sidebar".into(), category: ShortcutCategory::View,
                default_combo: KeyCombo::ctrl("b"), user_combo: None,
                command: "layout.toggle-sidebar".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "view.toggle-inspector".into(), name: "Toggle Inspector".into(),
                description: "Show or hide the inspector panel".into(), category: ShortcutCategory::View,
                default_combo: KeyCombo::ctrl("i"), user_combo: None,
                command: "layout.toggle-inspector".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "view.focus-mode".into(), name: "Focus Mode".into(),
                description: "Toggle focus mode".into(), category: ShortcutCategory::View,
                default_combo: KeyCombo::ctrl_shift("f"), user_combo: None,
                command: "layout.focus-mode".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "view.fullscreen".into(), name: "Fullscreen".into(),
                description: "Toggle fullscreen".into(), category: ShortcutCategory::View,
                default_combo: KeyCombo::new("F11"), user_combo: None,
                command: "view.fullscreen".into(), scope: "global".into(), enabled: true,
            },
            // Metadata
            ShortcutEntry {
                id: "meta.favorite".into(), name: "Toggle Favorite".into(),
                description: "Toggle favorite on selected assets".into(), category: ShortcutCategory::Metadata,
                default_combo: KeyCombo::ctrl("d"), user_combo: None,
                command: "metadata.toggle-favorite".into(), scope: "gallery".into(), enabled: true,
            },
            ShortcutEntry {
                id: "meta.rate-1".into(), name: "Rate 1 Star".into(),
                description: "Set rating to 1 star".into(), category: ShortcutCategory::Metadata,
                default_combo: KeyCombo::ctrl("1"), user_combo: None,
                command: "metadata.rate".into(), scope: "gallery".into(), enabled: true,
            },
            ShortcutEntry {
                id: "meta.rate-5".into(), name: "Rate 5 Stars".into(),
                description: "Set rating to 5 stars".into(), category: ShortcutCategory::Metadata,
                default_combo: KeyCombo::ctrl("5"), user_combo: None,
                command: "metadata.rate".into(), scope: "gallery".into(), enabled: true,
            },
            // Gallery
            ShortcutEntry {
                id: "gallery.open-viewer".into(), name: "Open Viewer".into(),
                description: "Open selected asset in viewer".into(), category: ShortcutCategory::Gallery,
                default_combo: KeyCombo::new("Enter"), user_combo: None,
                command: "gallery.open-viewer".into(), scope: "gallery".into(), enabled: true,
            },
            ShortcutEntry {
                id: "gallery.rename".into(), name: "Rename Asset".into(),
                description: "Rename selected asset".into(), category: ShortcutCategory::Gallery,
                default_combo: KeyCombo::new("F2"), user_combo: None,
                command: "gallery.rename".into(), scope: "gallery".into(), enabled: true,
            },
            // System
            ShortcutEntry {
                id: "sys.settings".into(), name: "Open Settings".into(),
                description: "Open application settings".into(), category: ShortcutCategory::System,
                default_combo: KeyCombo::ctrl(","), user_combo: None,
                command: "settings.open".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "sys.search".into(), name: "Focus Search".into(),
                description: "Focus the search bar".into(), category: ShortcutCategory::System,
                default_combo: KeyCombo::ctrl("f"), user_combo: None,
                command: "search.focus".into(), scope: "global".into(), enabled: true,
            },
            ShortcutEntry {
                id: "sys.escape".into(), name: "Close / Cancel".into(),
                description: "Close current view or cancel operation".into(), category: ShortcutCategory::System,
                default_combo: KeyCombo::new("Escape"), user_combo: None,
                command: "system.escape".into(), scope: "global".into(), enabled: true,
            },
        ];

        for s in defaults {
            self.shortcuts.insert(s.id.clone(), s);
        }
    }

    pub fn get(&self, id: &str) -> Option<&ShortcutEntry> {
        self.shortcuts.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ShortcutEntry> {
        self.shortcuts.get_mut(id)
    }

    pub fn all(&self) -> Vec<&ShortcutEntry> {
        self.shortcuts.values().collect()
    }

    pub fn rebind(&mut self, id: &str, combo: KeyCombo) {
        if let Some(entry) = self.shortcuts.get_mut(id) {
            entry.user_combo = Some(combo);
        }
    }

    pub fn toggle(&mut self, id: &str, enabled: bool) {
        if let Some(entry) = self.shortcuts.get_mut(id) {
            entry.enabled = enabled;
        }
    }

    pub fn find_by_combo(&self, combo: &KeyCombo, scope: &str) -> Vec<&ShortcutEntry> {
        self.shortcuts.values()
            .filter(|s| s.enabled && (s.scope == scope || s.scope == "global"))
            .filter(|s| {
                let c = s.user_combo.as_ref().unwrap_or(&s.default_combo);
                c.key == combo.key && c.ctrl == combo.ctrl && c.shift == combo.shift && c.alt == combo.alt && c.meta == combo.meta
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        self.shortcuts.len()
    }
}
