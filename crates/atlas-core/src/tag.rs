use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchicalTag {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub color: Option<String>,
    pub description: String,
    pub namespace: String,
}

impl HierarchicalTag {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent_id: None,
            color: None,
            description: String::new(),
            namespace: namespace.to_string(),
        }
    }

    pub fn child_of(name: &str, parent_id: Uuid, namespace: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent_id: Some(parent_id),
            color: None,
            description: String::new(),
            namespace: namespace.to_string(),
        }
    }
}
