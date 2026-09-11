use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateField {
    pub name: String,
    pub value: String,
}

impl TemplateField {
    pub fn new(name: &str, value: &str) -> Self {
        Self { name: name.to_string(), value: value.to_string() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub custom_fields: Vec<TemplateField>,
    pub notes_template: String,
    pub rating: Option<u8>,
    pub favorite: Option<bool>,
}

impl MetadataTemplate {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: String::new(),
            tags: Vec::new(),
            custom_fields: Vec::new(),
            notes_template: String::new(),
            rating: None,
            favorite: None,
        }
    }
}
