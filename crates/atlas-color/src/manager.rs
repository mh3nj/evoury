use crate::profile::{ColorSpace, DisplayColorSpace};
use crate::transform::ColorTransform;
use crate::icc::IccProfile;

#[derive(Debug, Clone)]
pub struct ColorManager {
    pub display_space: DisplayColorSpace,
    pub enable_icc: bool,
    pub enable_wide_gamut: bool,
    pub profiles: Vec<IccProfile>,
}

impl ColorManager {
    pub fn new() -> Self {
        Self {
            display_space: DisplayColorSpace::Srgb,
            enable_icc: true,
            enable_wide_gamut: false,
            profiles: vec![
                IccProfile::srgb(),
                IccProfile::display_p3(),
                IccProfile::adobe_rgb(),
            ],
        }
    }

    pub fn set_display_space(&mut self, space: DisplayColorSpace) {
        self.display_space = space;
    }

    pub fn enable_wide_gamut(&mut self, enabled: bool) {
        self.enable_wide_gamut = enabled;
    }

    pub fn convert_for_display(&self, pixel: &[u8; 4], source: ColorSpace) -> [u8; 4] {
        if !self.enable_icc || source == ColorSpace::Unmanaged {
            return *pixel;
        }
        let target = match self.display_space {
            DisplayColorSpace::Srgb => ColorSpace::Srgb,
            DisplayColorSpace::DisplayP3 => ColorSpace::DisplayP3,
            _ => ColorSpace::Srgb,
        };
        ColorTransform::convert(pixel, source, target)
    }
}
