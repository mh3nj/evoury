use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoClassifyRule {
    pub id: Uuid,
    pub name: String,
    pub extensions: Vec<String>,
    pub mime_patterns: Vec<String>,
    pub assign_tags: Vec<String>,
    pub assign_collection_id: Option<Uuid>,
    pub assign_rating: Option<u8>,
    pub assign_favorite: Option<bool>,
    pub priority: i32,
}

impl AutoClassifyRule {
    pub fn new(name: &str, extensions: &[&str]) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
            mime_patterns: Vec::new(),
            assign_tags: Vec::new(),
            assign_collection_id: None,
            assign_rating: None,
            assign_favorite: None,
            priority: 0,
        }
    }

    pub fn matches_extension(&self, ext: &str) -> bool {
        let ext = ext.trim_start_matches('.').to_lowercase();
        self.extensions.iter().any(|e| e.to_lowercase() == ext)
    }

    pub fn matches_mime(&self, mime: &str) -> bool {
        let mime = mime.to_lowercase();
        self.mime_patterns.iter().any(|p| mime.contains(&p.to_lowercase()))
    }
}
