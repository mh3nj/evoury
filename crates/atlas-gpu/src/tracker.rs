use std::collections::HashMap;
use uuid::Uuid;
use crate::texture::{TextureHandle, TextureState};

#[derive(Debug, Clone)]
pub struct MemoryBudget {
    pub max_vram_mb: u64,
    pub max_texture_count: u32,
    pub eviction_priority_threshold: u8,
}

impl Default for MemoryBudget {
    fn default() -> Self {
        Self {
            max_vram_mb: 1024,
            max_texture_count: 256,
            eviction_priority_threshold: 3,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total_vram_bytes: u64,
    pub texture_count: usize,
    pub resident_count: usize,
    pub evicted_count: usize,
    pub loading_count: usize,
    pub failed_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct GpuTracker {
    textures: HashMap<Uuid, TextureHandle>,
    budget: MemoryBudget,
}

impl GpuTracker {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            budget: MemoryBudget::default(),
        }
    }

    pub fn track(&mut self, handle: TextureHandle) {
        self.textures.insert(handle.id, handle);
    }

    pub fn untrack(&mut self, id: &Uuid) {
        self.textures.remove(id);
    }

    pub fn get(&self, id: &Uuid) -> Option<&TextureHandle> {
        self.textures.get(id)
    }

    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut TextureHandle> {
        self.textures.get_mut(id)
    }

    pub fn touch(&mut self, id: &Uuid) {
        if let Some(tex) = self.textures.get_mut(id) {
            tex.touch();
        }
    }

    pub fn mark_resident(&mut self, id: &Uuid) {
        if let Some(tex) = self.textures.get_mut(id) {
            tex.mark_resident();
        }
    }

    pub fn evict(&mut self, id: &Uuid) -> bool {
        if let Some(tex) = self.textures.get_mut(id) {
            tex.mark_evicted();
            true
        } else {
            false
        }
    }

    pub fn evict_oldest(&mut self, count: usize) -> usize {
        let mut candidates: Vec<(chrono::DateTime<chrono::Utc>, Uuid)> = self.textures.iter()
            .filter(|(_, t)| matches!(t.state, TextureState::Resident) && t.priority <= self.budget.eviction_priority_threshold)
            .map(|(id, t)| (t.last_accessed, *id))
            .collect();
        candidates.sort_by_key(|(time, _)| *time);

        let mut evicted = 0;
        for (_, id) in candidates.iter().take(count) {
            if self.evict(id) {
                evicted += 1;
            }
        }
        evicted
    }

    pub fn bytes_resident(&self) -> u64 {
        self.textures.values()
            .filter(|t| matches!(t.state, TextureState::Resident))
            .map(|t| t.estimated_vram())
            .sum()
    }

    pub fn over_budget(&self) -> bool {
        self.bytes_resident() > self.budget.max_vram_mb * 1024 * 1024
            || self.textures.len() > self.budget.max_texture_count as usize
    }

    pub fn stats(&self) -> MemoryStats {
        let mut stats = MemoryStats::default();
        for tex in self.textures.values() {
            stats.total_vram_bytes += tex.estimated_vram();
            match tex.state {
                TextureState::Resident => stats.resident_count += 1,
                TextureState::Evicted => stats.evicted_count += 1,
                TextureState::Loading => stats.loading_count += 1,
                TextureState::Failed => stats.failed_count += 1,
            }
        }
        stats.texture_count = self.textures.len();
        stats
    }
}
