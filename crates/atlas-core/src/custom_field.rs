use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Date,
    Dropdown(Vec<String>),
    Url,
    Color,
    Dimensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldDefinition {
    pub id: Uuid,
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: String,
    pub category: String,
}

impl CustomFieldDefinition {
    pub fn new(name: &str, field_type: FieldType, category: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            field_type,
            required: false,
            default_value: None,
            description: String::new(),
            category: category.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldValue {
    pub definition_id: Uuid,
    pub value: String,
}
