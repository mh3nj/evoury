use std::collections::HashMap;
use crate::descriptor::{ViewerDescriptor, ViewerKind};

#[derive(Debug, Clone, Default)]
pub struct ViewerRegistry {
    viewers: HashMap<ViewerKind, ViewerDescriptor>,
}

impl ViewerRegistry {
    pub fn new() -> Self {
        let mut reg = Self { viewers: HashMap::new() };
        for kind in &[
            ViewerKind::DefaultImage, ViewerKind::PsdLayers, ViewerKind::AiArtboards,
            ViewerKind::SvgVector, ViewerKind::PdfPages, ViewerKind::VideoTimeline,
            ViewerKind::AudioWaveform, ViewerKind::ThreeDOrbit, ViewerKind::FontGlyphs,
            ViewerKind::ZipTree, ViewerKind::CodeSyntax,
        ] {
            reg.register(ViewerDescriptor::for_kind(kind.clone()));
        }
        reg
    }

    pub fn register(&mut self, desc: ViewerDescriptor) {
        self.viewers.insert(desc.kind.clone(), desc);
    }

    pub fn get(&self, kind: &ViewerKind) -> Option<&ViewerDescriptor> {
        self.viewers.get(kind)
    }

    pub fn all(&self) -> Vec<&ViewerDescriptor> {
        self.viewers.values().collect()
    }

    pub fn find_for_extension(&self, ext: &str) -> Option<&ViewerDescriptor> {
        let ext = ext.to_lowercase();
        self.viewers.values().find(|v| v.kind.supported_extensions().contains(&ext.as_str()))
    }

    pub fn count(&self) -> usize {
        self.viewers.len()
    }
}
