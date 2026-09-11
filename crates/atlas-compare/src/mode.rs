use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareMode {
    SideBySide,
    Overlay,
    Slider,
    Diff,
    Swipe,
    Split,
    Toggle,
    Difference,
    Blend,
}

impl CompareMode {
    pub fn label(&self) -> &str {
        match self {
            Self::SideBySide => "Side by Side",
            Self::Overlay => "Overlay",
            Self::Slider => "Slider",
            Self::Diff => "Difference",
            Self::Swipe => "Swipe",
            Self::Split => "Split",
            Self::Toggle => "Toggle",
            Self::Difference => "Pixel Diff",
            Self::Blend => "Blend",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::SideBySide => "fa-columns",
            Self::Overlay => "fa-layer-group",
            Self::Slider => "fa-arrows-left-right",
            Self::Diff => "fa-not-equal",
            Self::Swipe => "fa-arrow-right-arrow-left",
            Self::Split => "fa-grip-lines-vertical",
            Self::Toggle => "fa-repeat",
            Self::Difference => "fa-eye",
            Self::Blend => "fa-droplet",
        }
    }
}

impl Default for CompareMode {
    fn default() -> Self {
        Self::SideBySide
    }
}
