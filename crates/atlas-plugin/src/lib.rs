pub mod registry;
pub mod traits;

pub use registry::PluginRegistry;
pub use traits::{Plugin, PluginError, PluginEvent, PluginManifest, PluginResult};
