use std::collections::HashMap;
use crate::view::ViewProfileEntry;
use crate::preview::PreviewProfileEntry;
use crate::search::SearchProfileEntry;

#[derive(Debug, Clone, Default)]
pub struct ProfileManager {
    view_profiles: HashMap<String, ViewProfileEntry>,
    preview_profiles: HashMap<String, PreviewProfileEntry>,
    search_profiles: HashMap<String, SearchProfileEntry>,
    active_view: String,
    active_preview: String,
    active_search: String,
}

impl ProfileManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            view_profiles: HashMap::new(),
            preview_profiles: HashMap::new(),
            search_profiles: HashMap::new(),
            active_view: "grid".into(),
            active_preview: "transparent-bg".into(),
            active_search: "logo-design".into(),
        };

        mgr.register_view("grid", ViewProfileEntry::grid());
        mgr.register_view("compact-grid", ViewProfileEntry::compact_grid());
        mgr.register_view("list", ViewProfileEntry::list());
        mgr.register_view("filmstrip", ViewProfileEntry::filmstrip());

        mgr.register_preview("transparent-bg", PreviewProfileEntry::transparent_bg());
        mgr.register_preview("dark-bg", PreviewProfileEntry::dark_bg());
        mgr.register_preview("light-bg", PreviewProfileEntry::light_bg());
        mgr.register_preview("mockup", PreviewProfileEntry::mockup());
        mgr.register_preview("detailed", PreviewProfileEntry::detailed());

        mgr.register_search("logo-design", SearchProfileEntry::logo_design());
        mgr.register_search("packaging", SearchProfileEntry::packaging());
        mgr.register_search("ui-design", SearchProfileEntry::ui_design());
        mgr.register_search("photography", SearchProfileEntry::photography());
        mgr.register_search("archival", SearchProfileEntry::archival());

        mgr
    }

    pub fn register_view(&mut self, id: &str, profile: ViewProfileEntry) {
        self.view_profiles.insert(id.to_string(), profile);
    }

    pub fn register_preview(&mut self, id: &str, profile: PreviewProfileEntry) {
        self.preview_profiles.insert(id.to_string(), profile);
    }

    pub fn register_search(&mut self, id: &str, profile: SearchProfileEntry) {
        self.search_profiles.insert(id.to_string(), profile);
    }

    pub fn get_view(&self, id: &str) -> Option<&ViewProfileEntry> {
        self.view_profiles.get(id)
    }

    pub fn get_preview(&self, id: &str) -> Option<&PreviewProfileEntry> {
        self.preview_profiles.get(id)
    }

    pub fn get_search(&self, id: &str) -> Option<&SearchProfileEntry> {
        self.search_profiles.get(id)
    }

    pub fn all_views(&self) -> Vec<&ViewProfileEntry> {
        self.view_profiles.values().collect()
    }

    pub fn all_previews(&self) -> Vec<&PreviewProfileEntry> {
        self.preview_profiles.values().collect()
    }

    pub fn all_searches(&self) -> Vec<&SearchProfileEntry> {
        self.search_profiles.values().collect()
    }

    pub fn set_active_view(&mut self, id: &str) {
        if self.view_profiles.contains_key(id) {
            self.active_view = id.to_string();
        }
    }

    pub fn set_active_preview(&mut self, id: &str) {
        if self.preview_profiles.contains_key(id) {
            self.active_preview = id.to_string();
        }
    }

    pub fn set_active_search(&mut self, id: &str) {
        if self.search_profiles.contains_key(id) {
            self.active_search = id.to_string();
        }
    }

    pub fn active_view(&self) -> Option<&ViewProfileEntry> {
        self.view_profiles.get(&self.active_view)
    }

    pub fn active_preview(&self) -> Option<&PreviewProfileEntry> {
        self.preview_profiles.get(&self.active_preview)
    }

    pub fn active_search(&self) -> Option<&SearchProfileEntry> {
        self.search_profiles.get(&self.active_search)
    }
}
