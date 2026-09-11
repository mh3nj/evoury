use std::collections::HashMap;
use atlas_core::Capability;
use crate::traits::{Plugin, PluginManifest, PluginEvent, PluginResult};

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    manifests: HashMap<String, PluginManifest>,
    event_log: Vec<PluginEvent>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            manifests: HashMap::new(),
            event_log: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>, manifest: PluginManifest) -> PluginResult {
        let id = manifest.id.clone();
        let mut plugin = plugin;

        if let Err(e) = plugin.on_load() {
            self.event_log.push(PluginEvent::Error(e.clone()));
            return Err(e);
        }

        self.plugins.insert(id.clone(), plugin);
        self.manifests.insert(id.clone(), manifest);
        self.event_log.push(PluginEvent::Loaded);
        Ok(())
    }

    pub fn unregister(&mut self, id: &str) -> PluginResult {
        if let Some(mut plugin) = self.plugins.remove(id) {
            if let Err(e) = plugin.on_unload() {
                self.event_log.push(PluginEvent::Error(e.clone()));
                return Err(e);
            }
            self.manifests.remove(id);
            self.event_log.push(PluginEvent::Unloaded);
        }
        Ok(())
    }

    pub fn get_plugin(&self, id: &str) -> Option<&dyn Plugin> {
        self.plugins.get(id).map(|p| p.as_ref())
    }

    pub fn get_manifest(&self, id: &str) -> Option<&PluginManifest> {
        self.manifests.get(id)
    }

    pub fn all_manifests(&self) -> Vec<&PluginManifest> {
        self.manifests.values().collect()
    }

    pub fn all_capabilities(&self) -> Vec<Capability> {
        self.plugins
            .values()
            .flat_map(|p| p.capabilities())
            .collect()
    }

    pub fn event_log(&self) -> &[PluginEvent] {
        &self.event_log
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyPlugin {
        id: String,
        name: String,
        version: String,
    }

    impl Plugin for DummyPlugin {
        fn id(&self) -> &str { &self.id }
        fn name(&self) -> &str { &self.name }
        fn version(&self) -> &str { &self.version }
    }

    #[test]
    fn test_register_and_get() {
        let mut reg = PluginRegistry::new();
        let manifest = PluginManifest::new("test.plugin", "Test Plugin", "1.0.0");
        let plugin = DummyPlugin { id: "test.plugin".into(), name: "Test Plugin".into(), version: "1.0.0".into() };

        assert!(reg.register(Box::new(plugin), manifest).is_ok());
        assert_eq!(reg.plugin_count(), 1);

        let m = reg.get_manifest("test.plugin").unwrap();
        assert_eq!(m.name, "Test Plugin");
    }

    #[test]
    fn test_unregister() {
        let mut reg = PluginRegistry::new();
        let manifest = PluginManifest::new("test.plugin", "Test Plugin", "1.0.0");
        let plugin = DummyPlugin { id: "test.plugin".into(), name: "Test Plugin".into(), version: "1.0.0".into() };

        reg.register(Box::new(plugin), manifest).unwrap();
        assert!(reg.unregister("test.plugin").is_ok());
        assert_eq!(reg.plugin_count(), 0);
    }

    #[test]
    fn test_event_log() {
        let mut reg = PluginRegistry::new();
        let manifest = PluginManifest::new("test.plugin", "Test Plugin", "1.0.0");
        let plugin = DummyPlugin { id: "test.plugin".into(), name: "Test Plugin".into(), version: "1.0.0".into() };

        reg.register(Box::new(plugin), manifest).unwrap();
        reg.unregister("test.plugin").unwrap();

        let log = reg.event_log();
        assert_eq!(log.len(), 2);
    }
}
