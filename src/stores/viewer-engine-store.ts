import { create } from "zustand";

export type ViewerKind =
  | "default-image" | "psd-layers" | "ai-artboards" | "svg-vector"
  | "pdf-pages" | "video-timeline" | "audio-waveform" | "3d-orbit"
  | "font-glyphs" | "zip-tree" | "code-syntax";

export interface ViewerCapability {
  supportsZoom: boolean;
  supportsPan: boolean;
  supportsRotate: boolean;
  supportsFullscreen: boolean;
}

interface ViewerEngineState {
  activeViewer: ViewerKind;
  zoom: number;
  panX: number;
  panY: number;
  rotation: number;
  playhead: number;
  currentPage: number;
  currentLayer: number;
  capability: ViewerCapability;
  setViewer: (kind: ViewerKind) => void;
  setZoom: (z: number) => void;
  setPan: (x: number, y: number) => void;
  setRotation: (r: number) => void;
  setPlayhead: (p: number) => void;
  setPage: (p: number) => void;
  setLayer: (l: number) => void;
  reset: () => void;
}

export const useViewerEngineStore = create<ViewerEngineState>((set) => ({
  activeViewer: "default-image",
  zoom: 1,
  panX: 0,
  panY: 0,
  rotation: 0,
  playhead: 0,
  currentPage: 0,
  currentLayer: 0,
  capability: { supportsZoom: true, supportsPan: true, supportsRotate: true, supportsFullscreen: true },
  setViewer: (kind) => {
    const caps: Record<ViewerKind, ViewerCapability> = {
      "default-image": { supportsZoom: true, supportsPan: true, supportsRotate: true, supportsFullscreen: true },
      "psd-layers": { supportsZoom: true, supportsPan: true, supportsRotate: false, supportsFullscreen: true },
      "ai-artboards": { supportsZoom: true, supportsPan: true, supportsRotate: false, supportsFullscreen: true },
      "svg-vector": { supportsZoom: true, supportsPan: true, supportsRotate: false, supportsFullscreen: true },
      "pdf-pages": { supportsZoom: true, supportsPan: true, supportsRotate: false, supportsFullscreen: true },
      "video-timeline": { supportsZoom: false, supportsPan: false, supportsRotate: false, supportsFullscreen: true },
      "audio-waveform": { supportsZoom: false, supportsPan: false, supportsRotate: false, supportsFullscreen: true },
      "3d-orbit": { supportsZoom: true, supportsPan: true, supportsRotate: true, supportsFullscreen: true },
      "font-glyphs": { supportsZoom: true, supportsPan: true, supportsRotate: false, supportsFullscreen: false },
      "zip-tree": { supportsZoom: false, supportsPan: false, supportsRotate: false, supportsFullscreen: false },
      "code-syntax": { supportsZoom: false, supportsPan: true, supportsRotate: false, supportsFullscreen: false },
    };
    set({ activeViewer: kind, capability: caps[kind], zoom: 1, panX: 0, panY: 0, rotation: 0 });
  },
  setZoom: (zoom) => set({ zoom: Math.max(0.1, Math.min(10, zoom)) }),
  setPan: (panX, panY) => set({ panX, panY }),
  setRotation: (rotation) => set({ rotation }),
  setPlayhead: (playhead) => set({ playhead }),
  setPage: (currentPage) => set({ currentPage }),
  setLayer: (currentLayer) => set({ currentLayer }),
  reset: () => set({ zoom: 1, panX: 0, panY: 0, rotation: 0, playhead: 0, currentPage: 0, currentLayer: 0 }),
}));
