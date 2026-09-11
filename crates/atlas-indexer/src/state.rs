use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexProgress {
    pub total: usize,
    pub completed: usize,
}

impl IndexProgress {
    pub fn percentage(&self) -> f32 {
        if self.total == 0 {
            return 100.0;
        }
        (self.completed as f32 / self.total as f32) * 100.0
    }
}
