use crate::profile::ColorSpace;

pub struct ColorTransform;

impl ColorTransform {
    pub fn srgb_to_display_p3(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        // Simplified matrix transform sRGB -> Display P3
        let nr = 0.8225 * r + 0.1775 * g + 0.0 * b;
        let ng = 0.0332 * r + 0.9668 * g + 0.0 * b;
        let nb = 0.0171 * r + 0.0724 * g + 0.9105 * b;
        (nr.clamp(0.0, 1.0), ng.clamp(0.0, 1.0), nb.clamp(0.0, 1.0))
    }

    pub fn srgb_to_adobe_rgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let nr = 0.7152 * r + 0.2848 * g + 0.0 * b;
        let ng = 0.0 * r + 1.0 * g + 0.0 * b;
        let nb = 0.0 * r + 0.0 * g + 1.0 * b;
        (nr.clamp(0.0, 1.0), ng.clamp(0.0, 1.0), nb.clamp(0.0, 1.0))
    }

    pub fn convert(pixel: &[u8; 4], from: ColorSpace, to: ColorSpace) -> [u8; 4] {
        if from == to {
            return *pixel;
        }
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let (nr, ng, nb) = match (from, to) {
            (ColorSpace::Srgb, ColorSpace::DisplayP3) => Self::srgb_to_display_p3(r, g, b),
            (ColorSpace::Srgb, ColorSpace::AdobeRgb) => Self::srgb_to_adobe_rgb(r, g, b),
            _ => (r, g, b),
        };
        [(nr * 255.0) as u8, (ng * 255.0) as u8, (nb * 255.0) as u8, pixel[3]]
    }
}
