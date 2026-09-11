use serde::{Deserialize, Serialize};
use crate::profile::ColorSpace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IccProfile {
    pub data: Vec<u8>,
    pub description: String,
    pub color_space: ColorSpace,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub copyright: Option<String>,
}

impl IccProfile {
    pub fn srgb() -> Self {
        Self {
            data: Vec::new(),
            description: "sRGB IEC61966-2.1".into(),
            color_space: ColorSpace::Srgb,
            manufacturer: Some("IEC".into()),
            model: Some("IEC 61966-2.1".into()),
            copyright: Some("Copyright © IEC".into()),
        }
    }

    pub fn display_p3() -> Self {
        Self {
            data: Vec::new(),
            description: "Display P3".into(),
            color_space: ColorSpace::DisplayP3,
            manufacturer: Some("Apple".into()),
            model: Some("Display P3".into()),
            copyright: None,
        }
    }

    pub fn adobe_rgb() -> Self {
        Self {
            data: Vec::new(),
            description: "Adobe RGB (1998)".into(),
            color_space: ColorSpace::AdobeRgb,
            manufacturer: Some("Adobe".into()),
            model: Some("Adobe RGB (1998)".into()),
            copyright: Some("Copyright © Adobe Systems".into()),
        }
    }
}
