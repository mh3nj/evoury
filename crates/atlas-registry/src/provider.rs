use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    Command,
    Theme,
    Capability,
    PreviewProfile,
    ViewProfile,
    SearchProfile,
    Workstation,
    Preview,
    Metadata,
    Search,
    Health,
    Import,
    Export,
    Worker,
    Cache,
    Scanner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub kind: ProviderKind,
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Provider {
    pub fn new(id: &str, kind: ProviderKind, name: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
        }
    }
}
