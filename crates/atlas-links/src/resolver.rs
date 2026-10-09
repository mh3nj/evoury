use crate::link::AssetLink;
use std::path::Path;

pub struct LinkResolver;

impl LinkResolver {
    pub fn source_exists(link: &AssetLink) -> bool {
        Path::new(&link.source_path).exists()
    }

    pub fn preview_exists(link: &AssetLink) -> bool {
        Path::new(&link.preview_path).exists()
    }
}
