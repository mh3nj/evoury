use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MotionPreference {
    Full,
    Reduced,
    NoPreference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub enabled: bool,
    pub motion_preference: MotionPreference,
    pub speed_multiplier: f32,
    pub interrupt_previous: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            enabled: true,
            motion_preference: MotionPreference::NoPreference,
            speed_multiplier: 1.0,
            interrupt_previous: true,
        }
    }
}

impl AnimationState {
    pub fn reduced() -> Self {
        Self {
            enabled: true,
            motion_preference: MotionPreference::Reduced,
            speed_multiplier: 0.3,
            interrupt_previous: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            motion_preference: MotionPreference::Reduced,
            speed_multiplier: 0.0,
            interrupt_previous: true,
        }
    }
}
