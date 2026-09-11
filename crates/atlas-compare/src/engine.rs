use crate::mode::CompareMode;
use crate::diff::DiffResult;

pub struct CompareEngine {
    pub mode: CompareMode,
    pub left_asset_id: uuid::Uuid,
    pub right_asset_id: uuid::Uuid,
    pub slider_position: f32,
    pub zoom: f32,
    pub linked_pan: bool,
    pub synchronized_zoom: bool,
}

impl CompareEngine {
    pub fn new(left: uuid::Uuid, right: uuid::Uuid) -> Self {
        Self {
            mode: CompareMode::SideBySide,
            left_asset_id: left,
            right_asset_id: right,
            slider_position: 0.5,
            zoom: 1.0,
            linked_pan: true,
            synchronized_zoom: true,
        }
    }

    pub fn set_mode(&mut self, mode: CompareMode) {
        self.mode = mode;
    }

    pub fn set_slider(&mut self, pos: f32) {
        self.slider_position = pos.clamp(0.0, 1.0);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 10.0);
    }

    pub fn diff(&self, _left_pixels: &[u8], _right_pixels: &[u8], _width: u32, _height: u32) -> DiffResult {
        DiffResult {
            total_pixels: 0,
            changed_pixels: 0,
            difference_pct: 0.0,
            regions: Vec::new(),
        }
    }
}
