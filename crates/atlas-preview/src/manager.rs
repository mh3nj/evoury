use uuid::Uuid;
use crate::cache::PreviewCache;
use crate::provider::PreviewProviderRegistry;
use crate::preview::PreviewDescriptor;
use crate::state::PreviewStatus;
use crate::resolver::PreviewResolver;

pub struct PreviewManager {
    pub cache: PreviewCache,
    pub providers: PreviewProviderRegistry,
    pub resolver: PreviewResolver,
}

impl PreviewManager {
    pub fn new() -> Self {
        Self {
            cache: PreviewCache::new(),
            providers: PreviewProviderRegistry::new(),
            resolver: PreviewResolver,
        }
    }

    pub fn resolve(&self, asset_id: &Uuid) -> Option<&std::path::PathBuf> {
        self.cache.get(asset_id).or_else(|| self.resolver.resolve_path(asset_id))
    }

    pub fn resolve_descriptor(&mut self, asset_id: &Uuid, file_path: &str, extension: &str) -> Result<PreviewDescriptor, String> {
        if let Some(cached) = self.cache.get(asset_id) {
            return Ok(PreviewDescriptor {
                path: cached.clone(),
                format: crate::preview::PreviewFormat::Png,
                status: PreviewStatus::Available,
            });
        }

        if let Some(provider) = self.providers.find_provider(asset_id, extension) {
            let output = provider.generate(asset_id, file_path)?;
            self.cache.insert(*asset_id, output.descriptor.path.clone());
            return Ok(output.descriptor);
        }

        Err(format!("No provider found for asset {}", asset_id))
    }

    pub fn status(&self, asset_id: &Uuid) -> PreviewStatus {
        if self.cache.get(asset_id).is_some() {
            PreviewStatus::Available
        } else {
            PreviewStatus::Missing
        }
    }
}
