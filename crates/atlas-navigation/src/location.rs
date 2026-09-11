use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavLocation {
    pub title: String,
    pub path: Vec<String>,
    pub collection_id: Option<Uuid>,
    pub category: Option<String>,
    pub query: Option<String>,
    pub asset_id: Option<Uuid>,
    pub view_profile: Option<String>,
}

impl NavLocation {
    pub fn root() -> Self {
        Self {
            title: "All Assets".into(),
            path: vec!["All Assets".into()],
            collection_id: None,
            category: None,
            query: None,
            asset_id: None,
            view_profile: None,
        }
    }

    pub fn collection(id: Uuid, name: &str) -> Self {
        Self {
            title: name.to_string(),
            path: vec!["Collections".into(), name.to_string()],
            collection_id: Some(id),
            category: None,
            query: None,
            asset_id: None,
            view_profile: None,
        }
    }

    pub fn search(query: &str) -> Self {
        Self {
            title: format!("Search: {}", query),
            path: vec!["Search".into(), query.to_string()],
            collection_id: None,
            category: None,
            query: Some(query.to_string()),
            asset_id: None,
            view_profile: None,
        }
    }

    pub fn asset(id: Uuid, name: &str) -> Self {
        Self {
            title: name.to_string(),
            path: vec!["All Assets".into(), name.to_string()],
            collection_id: None,
            category: None,
            query: None,
            asset_id: Some(id),
            view_profile: None,
        }
    }
}
