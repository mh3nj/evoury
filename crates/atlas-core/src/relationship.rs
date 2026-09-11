use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationshipKind {
    Reference,
    DerivedFrom,
    Version,
    Parent,
    Child,
    Dependency,
    Font,
    Image,
    Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub kind: RelationshipKind,
    pub label: String,
    pub metadata: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Relationship {
    pub fn new(source_id: Uuid, target_id: Uuid, kind: RelationshipKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            kind,
            label: String::new(),
            metadata: String::new(),
            created_at: chrono::Utc::now(),
        }
    }
}
