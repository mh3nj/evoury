use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInOutExpo,
    Spring { stiffness: f32, damping: f32 },
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::EaseInQuad => t * t,
            Self::EaseOutQuad => t * (2.0 - t),
            Self::EaseInOutQuad => if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t },
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Self::EaseInOutCubic => {
                if t < 0.5 { 4.0 * t * t * t }
                else {
                    let t = 2.0 * t - 2.0;
                    0.5 * t * t * t + 1.0
                }
            }
            Self::EaseInOutExpo => {
                if t == 0.0 || t == 1.0 { t }
                else if t < 0.5 { 0.5 * (2.0_f32.powf(20.0 * t - 10.0)) }
                else { 1.0 - 0.5 * (-2.0_f32.powf(-20.0 * t + 10.0)) }
            }
            Self::Spring { stiffness, damping } => {
                let t = t * std::f32::consts::PI * stiffness;
                1.0 - (-t * damping).exp() * t.cos()
            }
        }
    }

    pub fn css(&self) -> &str {
        match self {
            Self::Linear => "linear",
            Self::EaseInQuad => "cubic-bezier(0.55, 0.085, 0.68, 0.53)",
            Self::EaseOutQuad => "cubic-bezier(0.25, 0.46, 0.45, 0.94)",
            Self::EaseInOutQuad => "cubic-bezier(0.455, 0.03, 0.515, 0.955)",
            Self::EaseInCubic => "cubic-bezier(0.55, 0.055, 0.675, 0.19)",
            Self::EaseOutCubic => "cubic-bezier(0.215, 0.61, 0.355, 1)",
            Self::EaseInOutCubic => "cubic-bezier(0.645, 0.045, 0.355, 1)",
            Self::EaseInOutExpo => "cubic-bezier(0.87, 0, 0.13, 1)",
            Self::Spring { .. } => "cubic-bezier(0.34, 1.56, 0.64, 1)",
        }
    }
}
