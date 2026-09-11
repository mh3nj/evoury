use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureState {
    Loading,
    Resident,
    Evicted,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub bytes: u64,
    pub mip_count: u32,
}

#[derive(Debug, Clone)]
pub struct TextureHandle {
    pub id: Uuid,
    pub info: TextureInfo,
    pub state: TextureState,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub asset_id: Uuid,
    pub priority: u8,
}

impl TextureHandle {
    pub fn new(asset_id: Uuid, info: TextureInfo) -> Self {
        Self {
            id: Uuid::new_v4(),
            info,
            state: TextureState::Loading,
            last_accessed: chrono::Utc::now(),
            asset_id,
            priority: 5,
        }
    }

    pub fn mark_evicted(&mut self) {
        self.state = TextureState::Evicted;
    }

    pub fn mark_resident(&mut self) {
        self.state = TextureState::Resident;
        self.last_accessed = chrono::Utc::now();
    }

    pub fn touch(&mut self) {
        self.last_accessed = chrono::Utc::now();
    }

    pub fn estimated_vram(&self) -> u64 {
        self.info.bytes * self.info.mip_count as u64
    }
}
