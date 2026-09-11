use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: CommandCategory,
    pub keywords: Vec<String>,
    pub shortcut: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandCategory {
    Navigation,
    Library,
    View,
    System,
    Selection,
    Metadata,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: Uuid,
    pub success: bool,
    pub message: Option<String>,
}

impl Command {
    pub fn new(name: &str, desc: &str, category: CommandCategory) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: desc.to_string(),
            category,
            keywords: vec![],
            shortcut: None,
        }
    }
}
