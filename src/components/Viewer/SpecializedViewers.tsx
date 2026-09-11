import { useViewerEngineStore, type ViewerKind } from "../../stores/viewer-engine-store";

interface ViewerProps {
  src?: string;
  assetName: string;
}

function DefaultImageViewer({ src, assetName }: ViewerProps) {
  return src ? (
    <img
      src={src} alt={assetName}
      className="object-contain max-w-full max-h-full transition-transform duration-200"
      style={{ borderRadius: "12px", boxShadow: "0 8px 48px rgba(0,0,0,0.4)" }}
    />
  ) : (
    <Placeholder icon="fa-image" text="No Preview Available" />
  );
}

function PsdLayerViewer({ src, assetName }: ViewerProps) {
  return (
    <div className="flex flex-col items-center gap-3">
      {src ? <img src={src} alt={assetName} className="object-contain max-w-full max-h-[80vh] rounded-xl" /> : <Placeholder icon="fa-layer-group" text="PSD Layers" />}
      <div className="flex items-center gap-2 text-xs" style={{ color: "var(--evoury-text-dim)" }}>
        <i className="fas fa-layer-group" /> Layer {useViewerEngineStore.getState().currentLayer + 1}
      </div>
    </div>
  );
}

function SvgVectorViewer({ src, assetName }: ViewerProps) {
  return src ? (
    <object data={src} type="image/svg+xml" className="max-w-full max-h-full" aria-label={assetName}>
      <Placeholder icon="fa-vector-square" text="SVG unavailable" />
    </object>
  ) : <Placeholder icon="fa-vector-square" text="Vector Graphic" />;
}

function PdfPageViewer({ src, assetName }: ViewerProps) {
  return (
    <div className="flex flex-col items-center gap-3">
      {src ? (
        <object data={src} type="application/pdf" className="w-full h-[80vh] rounded-xl" aria-label={assetName}>
          <Placeholder icon="fa-file-pdf" text="PDF Viewer unavailable" />
        </object>
      ) : <Placeholder icon="fa-file-pdf" text="PDF Document" />}
    </div>
  );
}

function VideoTimelineViewer({ src }: ViewerProps) {
  if (!src) return <Placeholder icon="fa-video" text="Video" />;
  return (
    <video controls className="max-w-full max-h-full rounded-xl" style={{ boxShadow: "0 8px 48px rgba(0,0,0,0.4)" }}>
      <source src={src} />
    </video>
  );
}

function AudioWaveformViewer({ src, assetName }: ViewerProps) {
  if (!src) return <Placeholder icon="fa-music" text="Audio" />;
  return (
    <div className="flex flex-col items-center gap-4 w-full max-w-lg">
      <div className="w-full h-24 rounded-xl flex items-end justify-center gap-[2px] px-4"
        style={{ background: "var(--evoury-elevated)" }}>
        {Array.from({ length: 60 }).map((_, i) => (
          <div
            key={i}
            className="w-[3px] rounded-full transition-all"
            style={{
              height: `${20 + Math.sin(i * 0.5 + Date.now() * 0.001) * 30}px`,
              background: "var(--evoury-accent)",
              opacity: 0.3 + Math.sin(i * 0.3) * 0.3,
            }}
          />
        ))}
      </div>
      <audio controls className="w-full" src={src} aria-label={assetName} />
    </div>
  );
}

function ThreeDimensionViewer({ src, assetName }: ViewerProps) {
  return (
    <div className="flex flex-col items-center gap-3">
      {src ? (
        <div className="w-64 h-64 rounded-2xl flex items-center justify-center"
          style={{ background: "var(--evoury-elevated)", border: "1px solid var(--evoury-border)" }}>
          <div className="w-24 h-24 rounded-full relative flex items-center justify-center"
            style={{
              background: "conic-gradient(from 0deg, var(--evoury-accent), var(--evoury-accent2), var(--evoury-accent))",
              animation: "spin 4s linear infinite",
            }}>
            <div className="w-16 h-16 rounded-full" style={{ background: "var(--evoury-bg)" }} />
          </div>
        </div>
      ) : <Placeholder icon="fa-cube" text="3D Model" />}
      <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>{assetName}.glb</span>
    </div>
  );
}

function FontGlyphViewer({ assetName }: ViewerProps) {
  return (
    <div className="flex flex-col items-center gap-4">
      <Placeholder icon="fa-font" text={`Font: ${assetName}`} />
      <div className="grid grid-cols-4 gap-2">
        {"ABCDEFGH".split("").map((ch, i) => (
          <div key={i} className="w-12 h-12 rounded-lg flex items-center justify-center text-lg"
            style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", fontFamily: assetName }}>
            {ch}
          </div>
        ))}
      </div>
    </div>
  );
}

function ZipTreeViewer(_props: ViewerProps) {
  return (
    <div className="flex flex-col items-center gap-3 w-full max-w-sm">
      <Placeholder icon="fa-file-archive" text="Archive Contents" />
      <div className="w-full rounded-xl p-3 font-mono text-xs" style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)" }}>
        <div className="flex items-center gap-2"><i className="fas fa-folder" /> root/</div>
        <div className="ml-4 flex items-center gap-2"><i className="fas fa-folder" /> images/</div>
        <div className="ml-8 flex items-center gap-2"><i className="fas fa-file-image" /> photo1.jpg</div>
        <div className="ml-8 flex items-center gap-2"><i className="fas fa-file-image" /> photo2.png</div>
        <div className="ml-4 flex items-center gap-2"><i className="fas fa-file" /> manifest.json</div>
      </div>
    </div>
  );
}

function CodeSyntaxViewer({ assetName }: ViewerProps) {
  return (
    <div className="flex flex-col items-center gap-3 w-full max-w-xl">
      <Placeholder icon="fa-code" text={`Source: ${assetName}`} />
      <pre className="w-full rounded-xl p-4 text-xs leading-relaxed overflow-x-auto"
        style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px solid var(--evoury-border)" }}>
        <code>{`// Syntax highlighting coming soon
function greet(name: string): string {
  return \`Hello, \${name}!\`;
}`}</code>
      </pre>
    </div>
  );
}

function Placeholder({ icon, text }: { icon: string; text: string }) {
  return (
    <div className="text-center">
      <div className="w-16 h-16 rounded-2xl flex items-center justify-center mx-auto mb-3"
        style={{ background: "var(--evoury-elevated)" }}>
        <i className={`fas ${icon} text-2xl`} style={{ color: "var(--evoury-text-dim)" }} />
      </div>
      <span className="text-sm" style={{ color: "var(--evoury-text-dim)" }}>{text}</span>
    </div>
  );
}

const viewerMap: Record<ViewerKind, React.FC<ViewerProps>> = {
  "default-image": DefaultImageViewer,
  "psd-layers": PsdLayerViewer,
  "ai-artboards": DefaultImageViewer,
  "svg-vector": SvgVectorViewer,
  "pdf-pages": PdfPageViewer,
  "video-timeline": VideoTimelineViewer,
  "audio-waveform": AudioWaveformViewer,
  "3d-orbit": ThreeDimensionViewer,
  "font-glyphs": FontGlyphViewer,
  "zip-tree": ZipTreeViewer,
  "code-syntax": CodeSyntaxViewer,
};

export function getViewerComponent(kind: ViewerKind): React.FC<ViewerProps> {
  return viewerMap[kind] || DefaultImageViewer;
}

export function detectViewerKind(assetName: string, mimeType?: string): ViewerKind {
  if (mimeType) {
    if (mimeType.startsWith("video/")) return "video-timeline";
    if (mimeType.startsWith("audio/")) return "audio-waveform";
    if (mimeType === "application/pdf") return "pdf-pages";
    if (mimeType === "image/svg+xml") return "svg-vector";
    if (mimeType === "application/x-font-ttf" || mimeType === "font/ttf") return "font-glyphs";
    if (mimeType === "application/zip") return "zip-tree";
    if (mimeType.startsWith("text/") || mimeType.includes("javascript") || mimeType.includes("json")) return "code-syntax";
    if (mimeType === "image/vnd.adobe.photoshop" || mimeType === "application/x-photoshop") return "psd-layers";
    if (mimeType === "application/postscript" || mimeType === "application/illustrator") return "ai-artboards";
    if (mimeType.includes("model/")) return "3d-orbit";
  }
  const ext = assetName.split(".").pop()?.toLowerCase();
  if (ext) {
    if (["mp4", "webm", "mov", "avi", "mkv"].includes(ext)) return "video-timeline";
    if (["mp3", "wav", "flac", "ogg", "m4a"].includes(ext)) return "audio-waveform";
    if (["pdf"].includes(ext)) return "pdf-pages";
    if (["svg"].includes(ext)) return "svg-vector";
    if (["ttf", "otf", "woff", "woff2"].includes(ext)) return "font-glyphs";
    if (["zip", "tar", "gz", "rar", "7z"].includes(ext)) return "zip-tree";
    if (["js", "ts", "rs", "py", "go", "c", "cpp", "h", "hpp", "java", "css", "html", "json", "xml", "toml", "yaml", "md"].includes(ext)) return "code-syntax";
    if (["psd"].includes(ext)) return "psd-layers";
    if (["ai"].includes(ext)) return "ai-artboards";
    if (["glb", "gltf", "obj", "fbx", "stl"].includes(ext)) return "3d-orbit";
  }
  return "default-image";
}
