use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProfileEntry {
    pub id: String,
    pub name: String,
    pub indexed_fields: Vec<String>,
    pub visible_filters: Vec<String>,
    pub suggested_queries: Vec<String>,
}

impl SearchProfileEntry {
    pub fn logo_design() -> Self {
        Self {
            id: "logo-design".into(),
            name: "Logo Design".into(),
            indexed_fields: vec!["name".into(), "tags".into(), "notes".into()],
            visible_filters: vec!["type".into(), "tags".into(), "rating".into()],
            suggested_queries: vec!["logo".into(), "brand".into(), "vector".into()],
        }
    }

    pub fn packaging() -> Self {
        Self {
            id: "packaging".into(),
            name: "Packaging".into(),
            indexed_fields: vec!["name".into(), "tags".into(), "notes".into(), "category".into()],
            visible_filters: vec!["type".into(), "date".into(), "tags".into()],
            suggested_queries: vec!["dieline".into(), "mockup".into(), "label".into()],
        }
    }

    pub fn ui_design() -> Self {
        Self {
            id: "ui-design".into(),
            name: "UI Design".into(),
            indexed_fields: vec!["name".into(), "tags".into(), "notes".into()],
            visible_filters: vec!["type".into(), "tags".into(), "rating".into(), "favorite".into()],
            suggested_queries: vec!["icon".into(), "button".into(), "screen".into()],
        }
    }

    pub fn photography() -> Self {
        Self {
            id: "photography".into(),
            name: "Photography".into(),
            indexed_fields: vec!["name".into(), "tags".into(), "notes".into()],
            visible_filters: vec!["date".into(), "rating".into(), "type".into()],
            suggested_queries: vec!["portrait".into(), "landscape".into(), "product".into()],
        }
    }

    pub fn archival() -> Self {
        Self {
            id: "archival".into(),
            name: "Archival".into(),
            indexed_fields: vec!["name".into(), "tags".into(), "notes".into(), "category".into()],
            visible_filters: vec!["type".into(), "date".into(), "tags".into(), "rating".into()],
            suggested_queries: vec!["archive".into(), "backup".into(), "export".into()],
        }
    }
}
