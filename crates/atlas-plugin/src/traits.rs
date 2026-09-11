use serde::{Deserialize, Serialize};
use atlas_core::Capability;

pub trait Plugin: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str { "" }
    fn author(&self) -> &str { "" }

    fn on_load(&mut self) -> PluginResult { Ok(()) }
    fn on_unload(&mut self) -> PluginResult { Ok(()) }
    fn capabilities(&self) -> Vec<Capability> { Vec::new() }
}

pub type PluginResult = Result<(), PluginError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginError {
    pub code: String,
    pub message: String,
}

impl PluginError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
}

impl PluginManifest {
    pub fn new(id: &str, name: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            author: String::new(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    Loaded,
    Unloaded,
    Enabled,
    Disabled,
    Error(PluginError),
    Custom(String),
}
