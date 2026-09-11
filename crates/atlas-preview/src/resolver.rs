use std::path::PathBuf;
use atlas_core::Asset;
use uuid::Uuid;

pub struct PreviewResolver;

impl PreviewResolver {
    pub fn resolve(asset: &Asset) -> Option<PathBuf> {
        asset.preview.as_ref().map(|preview| preview.path.clone())
    }

    pub fn resolve_path(&self, _asset_id: &Uuid) -> Option<&PathBuf> {
        None
    }
}
