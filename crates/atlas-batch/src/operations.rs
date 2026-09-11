use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchOperation {
    Delete { asset_ids: Vec<Uuid> },
    MoveToCollection { asset_ids: Vec<Uuid>, collection_id: Uuid },
    AddTag { asset_ids: Vec<Uuid>, tags: Vec<String> },
    RemoveTag { asset_ids: Vec<Uuid>, tags: Vec<String> },
    SetRating { asset_ids: Vec<Uuid>, rating: u8 },
    SetFavorite { asset_ids: Vec<Uuid>, favorite: bool },
    AddToBasket { asset_ids: Vec<Uuid>, basket_id: Uuid },
    Export { asset_ids: Vec<Uuid>, destination: String },
    Rename { pairs: Vec<(Uuid, String)> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProgress {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub current_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub success: Vec<Uuid>,
    pub failed: Vec<(Uuid, String)>,
}

impl BatchProgress {
    pub fn new(total: usize) -> Self { Self { total, completed: 0, failed: 0, current_label: String::new() } }
}
