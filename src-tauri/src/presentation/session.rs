use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentationSession {
    pub active: bool,
    pub assets: Vec<String>,
    pub current_position: usize,
    pub displays: usize,
}
