use std::collections::HashMap;
use atlas_core::Capability;
use crate::{Provider, ProviderKind};

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub providers: Vec<Provider>,
    pub capabilities: Vec<Capability>,
}

pub struct Registry {
    by_kind: HashMap<ProviderKind, Vec<Provider>>,
    by_id: HashMap<String, Provider>,
    capabilities: Vec<Capability>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            by_kind: HashMap::new(),
            by_id: HashMap::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Provider) {
        self.by_kind
            .entry(provider.kind.clone())
            .or_default()
            .push(provider.clone());
        self.by_id.insert(provider.id.clone(), provider);
    }

    pub fn register_capability(&mut self, capability: Capability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    pub fn register_capabilities(&mut self, capabilities: Vec<Capability>) {
        for c in capabilities {
            self.register_capability(c);
        }
    }

    pub fn get_providers_by_kind(&self, kind: &ProviderKind) -> Vec<&Provider> {
        self.by_kind.get(kind).map(|v| v.iter().collect()).unwrap_or_default()
    }

    pub fn get_provider(&self, id: &str) -> Option<&Provider> {
        self.by_id.get(id)
    }

    pub fn all_providers(&self) -> Vec<&Provider> {
        self.by_id.values().collect()
    }

    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    pub fn entry(&self) -> RegistryEntry {
        RegistryEntry {
            providers: self.by_id.values().cloned().collect(),
            capabilities: self.capabilities.clone(),
        }
    }
}
