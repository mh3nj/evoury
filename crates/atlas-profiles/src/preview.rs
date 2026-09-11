use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewProfileEntry {
    pub id: String,
    pub name: String,
    pub background: String,
    pub zoom_mode: String,
    pub show_grid: bool,
    pub loop_animation: bool,
    pub asset_types: Vec<String>,
}

impl PreviewProfileEntry {
    pub fn transparent_bg() -> Self {
        Self {
            id: "transparent-bg".into(),
            name: "Transparent Background".into(),
            background: "checkerboard".into(),
            zoom_mode: "fit".into(),
            show_grid: false,
            loop_animation: true,
            asset_types: vec!["logo".into(), "icon".into(), "vector".into()],
        }
    }

    pub fn dark_bg() -> Self {
        Self {
            id: "dark-bg".into(),
            name: "Dark Background".into(),
            background: "#1a1a1a".into(),
            zoom_mode: "fill".into(),
            show_grid: false,
            loop_animation: true,
            asset_types: vec!["photo".into(), "render".into()],
        }
    }

    pub fn light_bg() -> Self {
        Self {
            id: "light-bg".into(),
            name: "Light Background".into(),
            background: "#f5f5f5".into(),
            zoom_mode: "fit".into(),
            show_grid: true,
            loop_animation: false,
            asset_types: vec!["screenshot".into(), "ui".into()],
        }
    }

    pub fn mockup() -> Self {
        Self {
            id: "mockup".into(),
            name: "Mockup Preview".into(),
            background: "#ffffff".into(),
            zoom_mode: "fit".into(),
            show_grid: false,
            loop_animation: false,
            asset_types: vec!["mockup".into(), "dieline".into()],
        }
    }

    pub fn detailed() -> Self {
        Self {
            id: "detailed".into(),
            name: "Detailed View".into(),
            background: "#2a2a2a".into(),
            zoom_mode: "actual".into(),
            show_grid: true,
            loop_animation: false,
            asset_types: vec![],
        }
    }
}
