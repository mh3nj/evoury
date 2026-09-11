use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityEventType {
    View,
    Open,
    Export,
    Edit,
    TagAdd,
    TagRemove,
    RatingChange,
    FavoriteToggle,
    NoteUpdate,
    MetadataChange,
    RelationshipAdd,
    RelationshipRemove,
    ProjectAdd,
    ProjectRemove,
    CollectionAdd,
    CollectionRemove,
    VersionCreate,
    VersionRestore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub event_type: ActivityEventType,
    pub details: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ActivityEvent {
    pub fn new(asset_id: Uuid, event_type: ActivityEventType, details: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            asset_id,
            event_type,
            details: details.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}
