pub mod easing;
pub mod manager;
pub mod preset;
pub mod state;

pub use easing::EasingFunction;
pub use manager::AnimationManager;
pub use preset::AnimationPreset;
pub use state::{AnimationState, MotionPreference};
