use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::stage::{PipelineStage, StageAction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineStatus { Idle, Running, Completed, Failed(String) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStageStatus {
    pub stage_id: usize,
    pub name: String,
    pub status: String, // "pending", "running", "completed", "skipped", "failed"
    pub items_processed: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub stages: Vec<PipelineStage>,
    pub status: PipelineStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Pipeline {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.to_string(),
            stages: Vec::new(),
            status: PipelineStatus::Idle,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn add_stage(&mut self, stage: PipelineStage) { self.stages.push(stage); }
    pub fn remove_stage(&mut self, stage_id: usize) { self.stages.retain(|s| s.id != stage_id); }
    pub fn reorder_stages(&mut self, new_order: &[usize]) {
        let mut reordered: Vec<PipelineStage> = new_order.iter().filter_map(|id| self.stages.iter().find(|s| s.id == *id).cloned()).collect();
        let remaining: Vec<PipelineStage> = self.stages.iter().filter(|s| !new_order.contains(&s.id)).cloned().collect();
        reordered.extend(remaining);
        self.stages = reordered;
    }
    pub fn toggle_stage(&mut self, stage_id: usize) {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.id == stage_id) { stage.enabled = !stage.enabled; }
    }
}

pub struct PipelineManager {
    pipelines: HashMap<Uuid, Pipeline>,
}

impl PipelineManager {
    pub fn new() -> Self { Self { pipelines: HashMap::new() } }

    pub fn create_pipeline(&mut self, name: &str, description: &str) -> Pipeline {
        let pipeline = Pipeline::new(name, description);
        self.pipelines.insert(pipeline.id, pipeline.clone());
        pipeline
    }

    pub fn add_pipeline(&mut self, pipeline: Pipeline) { self.pipelines.insert(pipeline.id, pipeline); }
    pub fn get_pipeline(&self, id: &Uuid) -> Option<&Pipeline> { self.pipelines.get(id) }
    pub fn get_pipeline_mut(&mut self, id: &Uuid) -> Option<&mut Pipeline> { self.pipelines.get_mut(id) }
    pub fn remove_pipeline(&mut self, id: &Uuid) { self.pipelines.remove(id); }
    pub fn all_pipelines(&self) -> Vec<&Pipeline> { self.pipelines.values().collect() }
    pub fn count(&self) -> usize { self.pipelines.len() }

    pub fn create_import_pipeline(&mut self) -> Pipeline {
        let mut p = Pipeline::new("Quick Import", "Import assets with dedup and classification");
        p.add_stage(PipelineStage::new(1, "Detect Duplicates", StageAction::DetectDuplicates));
        p.add_stage(PipelineStage::new(2, "Skip Existing", StageAction::SkipExisting));
        p.add_stage(PipelineStage::new(3, "Classify", StageAction::Classify));
        p.add_stage(PipelineStage::new(4, "Generate Previews", StageAction::GeneratePreviews));
        self.add_pipeline(p.clone());
        p
    }

    pub fn create_export_pipeline(&mut self) -> Pipeline {
        let mut p = Pipeline::new("Quick Export", "Export assets with validation");
        p.add_stage(PipelineStage::new(1, "Validate Sources", StageAction::ValidateSources));
        p.add_stage(PipelineStage::new(2, "Export", StageAction::Export { format: "original".into(), destination: "~/exports".into() }));
        self.add_pipeline(p.clone());
        p
    }
}
