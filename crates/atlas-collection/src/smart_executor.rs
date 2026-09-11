use std::collections::HashMap;
use uuid::Uuid;
use atlas_core::Asset;
use atlas_metadata::AssetMetadata;
use crate::smart::SmartCollection;

pub struct SmartCollectionExecutor;

impl SmartCollectionExecutor {
    pub fn evaluate(
        collection: &SmartCollection,
        assets: &[Asset],
        metadata_map: &HashMap<Uuid, AssetMetadata>,
    ) -> Vec<Uuid> {
        assets
            .iter()
            .filter(|asset| {
                let metadata = metadata_map.get(&asset.id);
                let tags: Vec<String> = metadata.map(|m| m.tags.clone()).unwrap_or_default();
                let rating = metadata.map(|m| m.rating).unwrap_or(0);
                let favorite = metadata.map(|m| m.favorite).unwrap_or(false);
                let file_type = asset
                    .archive_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                let notes = metadata.map(|m| &m.notes[..]).unwrap_or("");
                collection.matches(
                    &tags,
                    rating,
                    favorite,
                    file_type,
                    &asset.name,
                    notes,
                    None,
                    Some(asset.modified_at),
                    asset.file_size,
                )
            })
            .map(|asset| asset.id)
            .collect()
    }

    pub fn evaluate_all(
        smart_collections: &[SmartCollection],
        assets: &[Asset],
        metadata_map: &HashMap<Uuid, AssetMetadata>,
    ) -> HashMap<Uuid, Vec<Uuid>> {
        smart_collections
            .iter()
            .map(|sc| {
                let ids = Self::evaluate(sc, assets, metadata_map);
                (sc.id, ids)
            })
            .collect()
    }
}
