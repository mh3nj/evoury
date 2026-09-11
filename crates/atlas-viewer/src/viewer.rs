use serde::{Deserialize, Serialize};
use crate::descriptor::ViewerKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerCapability {
    pub name: String,
    pub available: bool,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ViewerContext {
    pub asset_id: uuid::Uuid,
    pub file_path: String,
    pub viewer_kind: ViewerKind,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub rotation: f32,
    pub playhead: f32,
    pub current_page: u32,
    pub current_layer: u32,
}

pub trait ViewerProvider: Send + Sync {
    fn name(&self) -> &str;
    fn kind(&self) -> ViewerKind;
    fn capabilities(&self) -> Vec<ViewerCapability>;
    fn can_view(&self, path: &str) -> bool;
    fn render(&self, context: &ViewerContext) -> Result<Vec<u8>, String>;
}
