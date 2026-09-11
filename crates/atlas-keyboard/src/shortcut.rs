use serde::{Deserialize, Serialize};
use crate::category::ShortcutCategory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCombo {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyCombo {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            ctrl: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }

    pub fn ctrl(key: &str) -> Self {
        Self { key: key.to_string(), ctrl: true, shift: false, alt: false, meta: false }
    }

    pub fn ctrl_shift(key: &str) -> Self {
        Self { key: key.to_string(), ctrl: true, shift: true, alt: false, meta: false }
    }

    pub fn description(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl { parts.push("Ctrl"); }
        if self.meta { parts.push("Cmd"); }
        if self.shift { parts.push("Shift"); }
        if self.alt { parts.push("Alt"); }
        let key_upper = self.key.to_uppercase();
        parts.push(&key_upper);
        parts.join("+")
    }

    pub fn matches(&self, event_key: &str, ctrl: bool, shift: bool, alt: bool, meta: bool) -> bool {
        self.key.eq_ignore_ascii_case(event_key)
            && self.ctrl == ctrl
            && self.shift == shift
            && self.alt == alt
            && self.meta == meta
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ShortcutCategory,
    pub default_combo: KeyCombo,
    pub user_combo: Option<KeyCombo>,
    pub command: String,
    pub scope: String,
    pub enabled: bool,
}
