use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressiveLevel {
    Placeholder,
    Small,
    Medium,
    Full,
}

impl ProgressiveLevel {
    pub fn max_dimension(&self) -> u32 {
        match self {
            Self::Placeholder => 32,
            Self::Small => 256,
            Self::Medium => 1024,
            Self::Full => 0,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Placeholder => "placeholder",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Full => "full",
        }
    }

    pub fn bytes_estimate(&self) -> u64 {
        match self {
            Self::Placeholder => 256,
            Self::Small => 16_384,
            Self::Medium => 131_072,
            Self::Full => 1_048_576,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelImage {
    pub level: ProgressiveLevel,
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ProgressivePipeline {
    pub levels: Vec<LevelImage>,
    pub asset_id: uuid::Uuid,
    pub completed: bool,
}

impl ProgressivePipeline {
    pub fn new(asset_id: uuid::Uuid) -> Self {
        Self {
            levels: Vec::new(),
            asset_id,
            completed: false,
        }
    }

    pub fn add_level(&mut self, image: LevelImage) {
        self.levels.push(image);
        self.levels.sort_by_key(|l| l.level.max_dimension());
        if self.levels.iter().any(|l| l.level == ProgressiveLevel::Full) {
            self.completed = true;
        }
    }

    pub fn best_available(&self, max_dimension: u32) -> Option<&LevelImage> {
        self.levels.iter()
            .filter(|l| l.level.max_dimension() >= max_dimension || l.level == ProgressiveLevel::Full)
            .next()
            .or_else(|| self.levels.last())
    }

    pub fn placeholder(&self) -> Option<&LevelImage> {
        self.levels.iter().find(|l| l.level == ProgressiveLevel::Placeholder)
    }

    pub fn thumbnail(&self) -> Option<&LevelImage> {
        self.levels.iter().find(|l| l.level == ProgressiveLevel::Small)
    }

    pub fn progress_pct(&self) -> f32 {
        if self.completed { return 1.0; }
        let weights: [(ProgressiveLevel, f32); 4] = [
            (ProgressiveLevel::Placeholder, 0.05),
            (ProgressiveLevel::Small, 0.25),
            (ProgressiveLevel::Medium, 0.60),
            (ProgressiveLevel::Full, 1.0),
        ];
        for (level, pct) in &weights {
            if !self.levels.iter().any(|l| l.level == *level) {
                return *pct;
            }
        }
        1.0
    }
}
