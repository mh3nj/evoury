use serde::{Deserialize, Serialize};
use crate::roles::WorkstationRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkstationPreset {
    pub id: String,
    pub name: String,
    pub role: WorkstationRole,
    pub description: String,
    pub default_sidebar_sections: Vec<String>,
    pub default_view_profile: String,
    pub default_search_profile: String,
    pub default_preview_profile: String,
    pub default_thumbnail_size: u32,
    pub default_sort_field: String,
    pub default_sort_direction: String,
    pub enabled_filters: Vec<String>,
    pub welcome_message: String,
}

impl WorkstationPreset {
    pub fn logo_design() -> Self {
        Self {
            id: "logo-design".into(),
            name: "Logo Design".into(),
            role: WorkstationRole::LogoDesign,
            description: "Optimized for logo files, brand assets, and vector graphics".into(),
            default_sidebar_sections: vec![
                "libraries".into(), "collections".into(), "smart_collections".into(),
                "tags".into(), "history".into(), "trash".into(),
            ],
            default_view_profile: "grid".into(),
            default_search_profile: "logo-design".into(),
            default_preview_profile: "transparent-bg".into(),
            default_thumbnail_size: 256,
            default_sort_field: "name".into(),
            default_sort_direction: "asc".into(),
            enabled_filters: vec!["type".into(), "tags".into(), "rating".into(), "date".into()],
            welcome_message: "Welcome to Logo Design mode. Browse your brand assets organized by project.".into(),
        }
    }

    pub fn packaging() -> Self {
        Self {
            id: "packaging".into(),
            name: "Packaging".into(),
            role: WorkstationRole::Packaging,
            description: "Layout for packaging mockups, dielines, and print assets".into(),
            default_sidebar_sections: vec![
                "libraries".into(), "collections".into(), "tags".into(),
            ],
            default_view_profile: "grid".into(),
            default_search_profile: "packaging".into(),
            default_preview_profile: "mockup".into(),
            default_thumbnail_size: 320,
            default_sort_field: "modified".into(),
            default_sort_direction: "desc".into(),
            enabled_filters: vec!["type".into(), "tags".into(), "date".into()],
            welcome_message: "Packaging mode active. Organize your print-ready assets.".into(),
        }
    }

    pub fn ui_design() -> Self {
        Self {
            id: "ui-design".into(),
            name: "UI Design".into(),
            role: WorkstationRole::UiDesign,
            description: "Perfect for UI kits, icons, screenshots, and design system assets".into(),
            default_sidebar_sections: vec![
                "collections".into(), "smart_collections".into(), "tags".into(),
            ],
            default_view_profile: "compact-grid".into(),
            default_search_profile: "ui-design".into(),
            default_preview_profile: "light-bg".into(),
            default_thumbnail_size: 200,
            default_sort_field: "name".into(),
            default_sort_direction: "asc".into(),
            enabled_filters: vec!["type".into(), "tags".into(), "rating".into(), "favorite".into()],
            welcome_message: "UI Design mode loaded. Your design system assets at a glance.".into(),
        }
    }

    pub fn photography() -> Self {
        Self {
            id: "photography".into(),
            name: "Photography".into(),
            role: WorkstationRole::Photography,
            description: "Large previews, camera metadata, and date-based organization".into(),
            default_sidebar_sections: vec!["collections".into(), "tags".into(), "history".into()],
            default_view_profile: "filmstrip".into(),
            default_search_profile: "photography".into(),
            default_preview_profile: "dark-bg".into(),
            default_thumbnail_size: 400,
            default_sort_field: "modified".into(),
            default_sort_direction: "desc".into(),
            enabled_filters: vec!["type".into(), "date".into(), "rating".into()],
            welcome_message: "Photography mode. Browse your images with filmstrip view.".into(),
        }
    }

    pub fn archival() -> Self {
        Self {
            id: "archival".into(),
            name: "Archival".into(),
            role: WorkstationRole::Archival,
            description: "Focus on metadata, version history, and file organization".into(),
            default_sidebar_sections: vec![
                "libraries".into(), "history".into(), "trash".into(),
            ],
            default_view_profile: "list".into(),
            default_search_profile: "archival".into(),
            default_preview_profile: "detailed".into(),
            default_thumbnail_size: 128,
            default_sort_field: "name".into(),
            default_sort_direction: "asc".into(),
            enabled_filters: vec!["type".into(), "date".into(), "tags".into()],
            welcome_message: "Archival mode. Full metadata and organization tools active.".into(),
        }
    }
}
