use serde::{Deserialize, Serialize};
use crate::panel::PanelId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DockNode {
    Leaf { panel: PanelId },
    Split {
        direction: SplitDirection,
        children: Vec<DockNode>,
        ratios: Vec<f32>,
    },
    TabGroup { panels: Vec<PanelId>, active: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DockArea {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockConfig {
    pub root: DockNode,
    pub sidebar_width: u32,
    pub inspector_width: u32,
    pub show_borders: bool,
    pub allow_docking: bool,
    pub allow_floating: bool,
}

impl Default for DockConfig {
    fn default() -> Self {
        Self {
            root: DockNode::Split {
                direction: SplitDirection::Horizontal,
                children: vec![
                    DockNode::Leaf { panel: PanelId::Sidebar },
                    DockNode::TabGroup {
                        panels: vec![PanelId::Gallery, PanelId::Search],
                        active: 0,
                    },
                    DockNode::Leaf { panel: PanelId::Inspector },
                ],
                ratios: vec![0.18, 0.60, 0.22],
            },
            sidebar_width: 256,
            inspector_width: 288,
            show_borders: true,
            allow_docking: true,
            allow_floating: false,
        }
    }
}
