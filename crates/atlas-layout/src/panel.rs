use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PanelId {
    Sidebar,
    Inspector,
    Gallery,
    Basket,
    Health,
    Viewer,
    Presentation,
    Search,
    Tags,
    History,
    Trash,
    Library,
    SmartCollections,
    Tabs,
}

impl PanelId {
    pub fn label(&self) -> &str {
        match self {
            Self::Sidebar => "Sidebar",
            Self::Inspector => "Inspector",
            Self::Gallery => "Gallery",
            Self::Basket => "Basket",
            Self::Health => "Health",
            Self::Viewer => "Viewer",
            Self::Presentation => "Presentation",
            Self::Search => "Search",
            Self::Tags => "Tags",
            Self::History => "History",
            Self::Trash => "Trash",
            Self::Library => "Library",
            Self::SmartCollections => "Smart Collections",
            Self::Tabs => "Tabs",
        }
    }

    pub fn default_visible(&self) -> bool {
        matches!(self, Self::Sidebar | Self::Inspector | Self::Gallery)
    }

    pub fn default_width(&self) -> u32 {
        match self {
            Self::Sidebar => 256,
            Self::Inspector => 288,
            Self::Gallery => 0,
            _ => 200,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    pub visible: bool,
    pub width: u32,
    pub height: u32,
    pub order: usize,
    pub collapsed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub id: PanelId,
    pub state: PanelState,
    pub pinned: bool,
    pub floating: bool,
    pub zone: String,
}

impl PanelConfig {
    pub fn new(id: PanelId) -> Self {
        let width = id.default_width();
        Self {
            state: PanelState {
                visible: id.default_visible(),
                width,
                height: 0,
                order: 0,
                collapsed: false,
            },
            pinned: true,
            floating: false,
            zone: "left".into(),
            id,
        }
    }
}
