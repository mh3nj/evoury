use std::collections::HashMap;
use crate::state::{AnimationState, MotionPreference};
use crate::preset::AnimationPreset;

#[derive(Debug, Clone)]
pub struct AnimationManager {
    pub state: AnimationState,
    presets: HashMap<String, AnimationPreset>,
}

impl AnimationManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            state: AnimationState::default(),
            presets: HashMap::new(),
        };
        mgr.register(AnimationPreset::fade_in());
        mgr.register(AnimationPreset::slide_up());
        mgr.register(AnimationPreset::scale_in());
        mgr.register(AnimationPreset::quick());
        mgr.register(AnimationPreset::spring());
        mgr
    }

    pub fn register(&mut self, preset: AnimationPreset) {
        self.presets.insert(preset.id.clone(), preset);
    }

    pub fn get(&self, id: &str) -> Option<&AnimationPreset> {
        self.presets.get(id)
    }

    pub fn all(&self) -> Vec<&AnimationPreset> {
        self.presets.values().collect()
    }

    pub fn effective_duration(&self, preset_id: &str) -> u32 {
        let base = self.presets.get(preset_id).map(|p| p.duration_ms).unwrap_or(200);
        (base as f32 * self.state.speed_multiplier) as u32
    }

    pub fn set_motion_preference(&mut self, pref: MotionPreference) {
        self.state.motion_preference = pref;
        match pref {
            MotionPreference::Reduced => self.state.speed_multiplier = 0.3,
            MotionPreference::NoPreference => self.state.speed_multiplier = 1.0,
            MotionPreference::Full => self.state.speed_multiplier = 1.0,
        }
    }

    pub fn should_animate(&self) -> bool {
        self.state.enabled && self.state.motion_preference != MotionPreference::Reduced
    }
}
