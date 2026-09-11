
use atlas_core::Asset;
use atlas_events::{AtlasEvent, EventBus};
use crate::metadata::AssetMetadata;

#[derive(Debug)]
pub enum MetadataIssue {
    MissingField(String),
    InvalidValue(String),
    BrokenReference(String),
}

pub struct MetadataPipeline {
    event_bus: Option<EventBus>,
}

impl MetadataPipeline {
    pub fn new() -> Self {
        Self { event_bus: None }
    }

    pub fn with_event_bus(mut self, bus: EventBus) -> Self {
        self.event_bus = Some(bus);
        self
    }

    pub fn process(&self, metadata: &mut AssetMetadata, asset: Option<&Asset>) -> Vec<MetadataIssue> {
        let issues = self.validate(metadata, asset);
        self.store(metadata);
        self.notify(metadata);
        issues
    }

    pub fn extract(&self, asset: &Asset) -> AssetMetadata {
        let mut meta = AssetMetadata::default();
        meta.asset_id = asset.id;
        meta
    }

    pub fn normalize(&self, metadata: &mut AssetMetadata) {
        metadata.tags = metadata.tags.iter().map(|t| t.trim().to_lowercase()).filter(|t| !t.is_empty()).collect();
        metadata.notes = metadata.notes.trim().to_string();
        metadata.rating = metadata.rating.min(5);
    }

    pub fn validate(&self, metadata: &AssetMetadata, asset: Option<&Asset>) -> Vec<MetadataIssue> {
        let mut issues = Vec::new();

        if metadata.asset_id.is_nil() {
            issues.push(MetadataIssue::MissingField("asset_id".into()));
        }

        if let Some(asset) = asset {
            if !asset.archive_path.exists() {
                issues.push(MetadataIssue::BrokenReference(
                    format!("Archive path missing: {}", asset.archive_path.display())
                ));
            }
        }

        if metadata.rating > 5 {
            issues.push(MetadataIssue::InvalidValue("rating > 5".into()));
        }

        issues
    }

    pub fn store(&self, _metadata: &AssetMetadata) {
        // Storage happens at persistence layer; pipeline validates and normalizes beforehand
    }

    pub fn notify(&self, metadata: &AssetMetadata) {
        if let Some(ref bus) = self.event_bus {
            let sender = bus.sender();
            let _ = sender.send(AtlasEvent::MetadataChanged { asset_id: metadata.asset_id });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_normalize_tags() {
        let mut meta = AssetMetadata {
            asset_id: Uuid::new_v4(),
            tags: vec![" Logo ".into(), "MINIMAL".into(), "".into()],
            ..Default::default()
        };
        let pipeline = MetadataPipeline::new();
        pipeline.normalize(&mut meta);
        assert_eq!(meta.tags, vec!["logo", "minimal"]);
    }

    #[test]
    fn test_validate_rating() {
        let meta = AssetMetadata {
            asset_id: Uuid::new_v4(),
            rating: 7,
            ..Default::default()
        };
        let pipeline = MetadataPipeline::new();
        let issues = pipeline.validate(&meta, None);
        assert!(issues.iter().any(|i| matches!(i, MetadataIssue::InvalidValue(_))));
    }
}
