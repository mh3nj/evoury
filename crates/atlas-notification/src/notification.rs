use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationLevel { Info, Success, Warning, Error }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationCategory {
    Scan, Import, Export, BatchOp, Macro, Health, Maintenance, Pipeline, System, Undo, Metadata, Update,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub level: NotificationLevel,
    pub category: NotificationCategory,
    pub action_label: Option<String>,
    pub action_id: Option<String>,
    pub read: bool,
    pub dismissed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Notification {
    pub fn new(title: &str, message: &str, level: NotificationLevel, category: NotificationCategory) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            message: message.to_string(),
            level,
            category,
            action_label: None,
            action_id: None,
            read: false,
            dismissed: false,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_action(mut self, label: &str, action_id: &str) -> Self {
        self.action_label = Some(label.to_string());
        self.action_id = Some(action_id.to_string());
        self
    }
}
