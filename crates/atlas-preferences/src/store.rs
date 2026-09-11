use crate::preferences::Preferences;

pub struct PreferenceStore {
    preferences: Preferences,
}

impl PreferenceStore {
    pub fn new(preferences: Preferences) -> Self {
        Self { preferences }
    }

    pub fn get(&self) -> &Preferences {
        &self.preferences
    }

    pub fn update(&mut self, preferences: Preferences) {
        self.preferences = preferences;
    }
}
