use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageAction {
    DetectDuplicates,
    Deduplicate,
    Classify,
    ApplyTemplate { template_id: Option<uuid::Uuid> },
    AddTags { tags: Vec<String> },
    AddToCollection { collection_id: Option<uuid::Uuid> },
    GeneratePreviews,
    AddToBasket { basket_id: Option<uuid::Uuid> },
    Export { format: String, destination: String },
    SkipExisting,
    ValidateSources,
    RemoveOriginalsAfterImport,
    Custom { name: String, config: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: usize,
    pub name: String,
    pub action: StageAction,
    pub enabled: bool,
    pub config: String,
}

impl PipelineStage {
    pub fn new(id: usize, name: &str, action: StageAction) -> Self {
        Self { id, name: name.to_string(), action, enabled: true, config: String::new() }
    }
}
