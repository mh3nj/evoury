use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: Option<String>,
    pub asset_ids: Vec<Uuid>,
    pub notes: Vec<ProjectNote>,
    pub deliverables: Vec<Deliverable>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNote {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub pinned: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub path: Option<PathBuf>,
    pub asset_id: Option<Uuid>,
    pub status: DeliverableStatus,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliverableStatus {
    Planned,
    InProgress,
    Review,
    Approved,
    Delivered,
    Cancelled,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            icon: "fa-briefcase".into(),
            color: None,
            asset_ids: Vec::new(),
            notes: Vec::new(),
            deliverables: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
