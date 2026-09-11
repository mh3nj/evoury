use uuid::Uuid;
use crate::preview::PreviewDescriptor;
use crate::state::PreviewStatus;

#[derive(Debug, Clone)]
pub struct ProviderOutput {
    pub descriptor: PreviewDescriptor,
    pub bytes: Option<Vec<u8>>,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
}

pub trait PreviewProvider: Send + Sync {
    fn name(&self) -> &str;
    fn can_handle(&self, asset_id: &Uuid, extension: &str) -> bool;
    fn generate(&self, asset_id: &Uuid, file_path: &str) -> Result<ProviderOutput, String>;
    fn supported_extensions(&self) -> Vec<String>;
    fn priority(&self) -> u8;
}

#[derive(Default)]
pub struct PreviewProviderRegistry {
    providers: Vec<Box<dyn PreviewProvider>>,
}

impl PreviewProviderRegistry {
    pub fn new() -> Self {
        Self { providers: Vec::new() }
    }

    pub fn register(&mut self, provider: Box<dyn PreviewProvider>) {
        self.providers.push(provider);
    }

    pub fn find_provider(&self, asset_id: &Uuid, extension: &str) -> Option<&Box<dyn PreviewProvider>> {
        let mut candidates: Vec<&Box<dyn PreviewProvider>> = self.providers.iter()
            .filter(|p| p.can_handle(asset_id, extension))
            .collect();
        candidates.sort_by_key(|p| std::cmp::Reverse(p.priority()));
        candidates.into_iter().next()
    }

    pub fn all_providers(&self) -> &[Box<dyn PreviewProvider>] {
        &self.providers
    }
}

pub struct DefaultImageProvider;

impl PreviewProvider for DefaultImageProvider {
    fn name(&self) -> &str { "Default Image Provider" }
    fn can_handle(&self, _asset_id: &Uuid, extension: &str) -> bool {
        matches!(extension.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp" | "avif" | "bmp" | "gif")
    }
    fn generate(&self, _asset_id: &Uuid, file_path: &str) -> Result<ProviderOutput, String> {
        Ok(ProviderOutput {
            descriptor: PreviewDescriptor {
                path: std::path::PathBuf::from(file_path),
                format: crate::preview::PreviewFormat::Png,
                status: PreviewStatus::Available,
            },
            bytes: None,
            mime_type: "image/png".into(),
            width: 0,
            height: 0,
        })
    }
    fn supported_extensions(&self) -> Vec<String> {
        vec!["png".into(), "jpg".into(), "jpeg".into(), "webp".into(), "avif".into(), "bmp".into(), "gif".into()]
    }
    fn priority(&self) -> u8 { 0 }
}
