use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkstationRole {
    LogoDesign,
    Packaging,
    UiDesign,
    Photography,
    Illustration,
    VideoEditing,
    Archival,
    General,
}

impl WorkstationRole {
    pub fn label(&self) -> &str {
        match self {
            Self::LogoDesign => "Logo Design",
            Self::Packaging => "Packaging",
            Self::UiDesign => "UI Design",
            Self::Photography => "Photography",
            Self::Illustration => "Illustration",
            Self::VideoEditing => "Video Editing",
            Self::Archival => "Archival",
            Self::General => "General",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::LogoDesign => "fa-pen-fancy",
            Self::Packaging => "fa-box-open",
            Self::UiDesign => "fa-mobile-screen",
            Self::Photography => "fa-camera",
            Self::Illustration => "fa-paint-brush",
            Self::VideoEditing => "fa-film",
            Self::Archival => "fa-archive",
            Self::General => "fa-briefcase",
        }
    }
}
