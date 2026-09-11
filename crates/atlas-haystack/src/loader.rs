use atlas_core::Asset;
use uuid::Uuid;

pub struct ChunkedAssetLoader {
    all_assets: Vec<Asset>,
    chunk_size: usize,
}

impl ChunkedAssetLoader {
    pub fn new(assets: Vec<Asset>, chunk_size: usize) -> Self {
        Self { all_assets: assets, chunk_size }
    }

    pub fn chunk_count(&self) -> usize {
        (self.all_assets.len() + self.chunk_size - 1) / self.chunk_size
    }

    pub fn load_chunk(&self, index: usize) -> Vec<&Asset> {
        let start = index * self.chunk_size;
        let end = std::cmp::min(start + self.chunk_size, self.all_assets.len());
        if start >= self.all_assets.len() {
            Vec::new()
        } else {
            self.all_assets[start..end].iter().collect()
        }
    }

    pub fn load_by_ids(&self, ids: &[Uuid]) -> Vec<&Asset> {
        self.all_assets.iter().filter(|a| ids.contains(&a.id)).collect()
    }

    pub fn total(&self) -> usize {
        self.all_assets.len()
    }

    pub fn all(&self) -> &[Asset] {
        &self.all_assets
    }
}
