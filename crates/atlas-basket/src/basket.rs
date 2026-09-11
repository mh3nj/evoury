use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basket {
    pub id: Uuid,
    pub name: String,
    pub assets: Vec<Uuid>,
    pub created: DateTime<Utc>,
}
