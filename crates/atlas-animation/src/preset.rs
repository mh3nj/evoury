use serde::{Deserialize, Serialize};
use crate::easing::EasingFunction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationPreset {
    pub id: String,
    pub name: String,
    pub duration_ms: u32,
    pub easing: EasingFunction,
    pub delay_ms: u32,
    pub property: String,
}

impl AnimationPreset {
    pub fn fade_in() -> Self {
        Self {
            id: "fade-in".into(),
            name: "Fade In".into(),
            duration_ms: 200,
            easing: EasingFunction::EaseOutCubic,
            delay_ms: 0,
            property: "opacity".into(),
        }
    }

    pub fn slide_up() -> Self {
        Self {
            id: "slide-up".into(),
            name: "Slide Up".into(),
            duration_ms: 300,
            easing: EasingFunction::EaseOutQuad,
            delay_ms: 0,
            property: "transform".into(),
        }
    }

    pub fn scale_in() -> Self {
        Self {
            id: "scale-in".into(),
            name: "Scale In".into(),
            duration_ms: 200,
            easing: EasingFunction::EaseOutCubic,
            delay_ms: 0,
            property: "transform".into(),
        }
    }

    pub fn quick() -> Self {
        Self {
            id: "quick".into(),
            name: "Quick".into(),
            duration_ms: 100,
            easing: EasingFunction::EaseOutQuad,
            delay_ms: 0,
            property: "all".into(),
        }
    }

    pub fn spring() -> Self {
        Self {
            id: "spring".into(),
            name: "Spring".into(),
            duration_ms: 400,
            easing: EasingFunction::Spring { stiffness: 0.5, damping: 0.7 },
            delay_ms: 0,
            property: "transform".into(),
        }
    }

    pub fn staggered(items: usize) -> Vec<Self> {
        (0..items).map(|i| Self {
            id: format!("stagger-{}", i),
            name: format!("Stagger {}", i + 1),
            duration_ms: 200,
            easing: EasingFunction::EaseOutCubic,
            delay_ms: (i as u32) * 50,
            property: "all".into(),
        }).collect()
    }
}
