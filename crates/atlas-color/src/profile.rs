use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorSpace {
    Srgb,
    AdobeRgb,
    DisplayP3,
    ProPhotoRgb,
    Rec2020,
    LinearSrgb,
    Unmanaged,
}

impl ColorSpace {
    pub fn label(&self) -> &str {
        match self {
            Self::Srgb => "sRGB",
            Self::AdobeRgb => "Adobe RGB",
            Self::DisplayP3 => "Display P3",
            Self::ProPhotoRgb => "ProPhoto RGB",
            Self::Rec2020 => "Rec. 2020",
            Self::LinearSrgb => "Linear sRGB",
            Self::Unmanaged => "Unmanaged",
        }
    }

    pub fn is_wide_gamut(&self) -> bool {
        matches!(self, Self::AdobeRgb | Self::DisplayP3 | Self::ProPhotoRgb | Self::Rec2020)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DisplayColorSpace {
    Native,
    Srgb,
    DisplayP3,
    Hdr10,
}
