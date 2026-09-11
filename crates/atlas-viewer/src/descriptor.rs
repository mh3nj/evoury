use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViewerKind {
    DefaultImage,
    PsdLayers,
    AiArtboards,
    SvgVector,
    PdfPages,
    VideoTimeline,
    AudioWaveform,
    ThreeDOrbit,
    FontGlyphs,
    ZipTree,
    CodeSyntax,
}

impl ViewerKind {
    pub fn label(&self) -> &str {
        match self {
            Self::DefaultImage => "Image Viewer",
            Self::PsdLayers => "Photoshop Layers",
            Self::AiArtboards => "Illustrator Artboards",
            Self::SvgVector => "SVG Viewer",
            Self::PdfPages => "PDF Viewer",
            Self::VideoTimeline => "Video Timeline",
            Self::AudioWaveform => "Audio Waveform",
            Self::ThreeDOrbit => "3D Orbit",
            Self::FontGlyphs => "Font Glyphs",
            Self::ZipTree => "Archive Tree",
            Self::CodeSyntax => "Code Viewer",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::DefaultImage => "fa-image",
            Self::PsdLayers => "fa-layer-group",
            Self::AiArtboards => "fa-pen-ruler",
            Self::SvgVector => "fa-vector-square",
            Self::PdfPages => "fa-file-pdf",
            Self::VideoTimeline => "fa-video",
            Self::AudioWaveform => "fa-music",
            Self::ThreeDOrbit => "fa-cube",
            Self::FontGlyphs => "fa-font",
            Self::ZipTree => "fa-folder-tree",
            Self::CodeSyntax => "fa-code",
        }
    }

    pub fn supported_extensions(&self) -> &[&str] {
        match self {
            Self::DefaultImage => &["png", "jpg", "jpeg", "webp", "avif", "bmp", "gif", "tiff"],
            Self::PsdLayers => &["psd"],
            Self::AiArtboards => &["ai"],
            Self::SvgVector => &["svg"],
            Self::PdfPages => &["pdf"],
            Self::VideoTimeline => &["mp4", "mov", "avi", "mkv", "webm"],
            Self::AudioWaveform => &["mp3", "wav", "flac", "ogg", "aac"],
            Self::ThreeDOrbit => &["glb", "gltf", "obj", "stl", "fbx"],
            Self::FontGlyphs => &["ttf", "otf", "woff", "woff2"],
            Self::ZipTree => &["zip", "rar", "7z", "tar", "tgz", "gz"],
            Self::CodeSyntax => &["js", "ts", "jsx", "tsx", "rs", "py", "css", "html", "json", "xml", "md", "yaml", "toml"],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerDescriptor {
    pub kind: ViewerKind,
    pub name: String,
    pub description: String,
    pub supports_zoom: bool,
    pub supports_pan: bool,
    pub supports_rotate: bool,
    pub supports_fullscreen: bool,
    pub capabilities: Vec<String>,
}

impl ViewerDescriptor {
    pub fn for_kind(kind: ViewerKind) -> Self {
        let (supports_zoom, supports_pan, supports_rotate, supports_fullscreen) = match kind {
            ViewerKind::DefaultImage => (true, true, true, true),
            ViewerKind::PsdLayers => (true, true, false, true),
            ViewerKind::AiArtboards => (true, true, false, true),
            ViewerKind::SvgVector => (true, true, false, true),
            ViewerKind::PdfPages => (true, true, false, true),
            ViewerKind::VideoTimeline => (false, false, false, true),
            ViewerKind::AudioWaveform => (false, false, false, true),
            ViewerKind::ThreeDOrbit => (true, true, true, true),
            ViewerKind::FontGlyphs => (true, true, false, false),
            ViewerKind::ZipTree => (false, false, false, false),
            ViewerKind::CodeSyntax => (false, true, false, false),
        };
        Self {
            kind: kind.clone(),
            name: kind.label().to_string(),
            description: String::new(),
            supports_zoom,
            supports_pan,
            supports_rotate,
            supports_fullscreen,
            capabilities: Vec::new(),
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        let ext = ext.to_lowercase();
        for kind in &[
            ViewerKind::DefaultImage, ViewerKind::PsdLayers, ViewerKind::AiArtboards,
            ViewerKind::SvgVector, ViewerKind::PdfPages, ViewerKind::VideoTimeline,
            ViewerKind::AudioWaveform, ViewerKind::ThreeDOrbit, ViewerKind::FontGlyphs,
            ViewerKind::ZipTree, ViewerKind::CodeSyntax,
        ] {
            if kind.supported_extensions().contains(&ext.as_str()) {
                return Some(Self::for_kind(kind.clone()));
            }
        }
        None
    }
}
