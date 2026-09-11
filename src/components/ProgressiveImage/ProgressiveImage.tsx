import { useState, useEffect, useRef } from "react";
import { useGpuStore } from "../../stores/gpu-store";

interface Props {
  src: string;
  alt: string;
  placeholderColor?: string;
  thumbnailSrc?: string;
  onLoad?: () => void;
  className?: string;
  style?: React.CSSProperties;
  priority?: number;
}

type Stage = "idle" | "loading-thumbnail" | "thumbnail-loaded" | "loading-full" | "full-loaded" | "error";

export default function ProgressiveImage({
  src, alt, placeholderColor = "#1a1a2e", thumbnailSrc, onLoad,
  className = "", style = {}, priority = 0,
}: Props) {
  const [stage, setStage] = useState<Stage>(thumbnailSrc ? "loading-thumbnail" : "loading-full");
  const [thumbLoaded, setThumbLoaded] = useState(false);
  const [fullLoaded, setFullLoaded] = useState(false);
  const trackTexture = useGpuStore((s) => s.trackTexture);
  const imgRef = useRef<HTMLImageElement>(null);
  const fullRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    if (!thumbnailSrc || thumbLoaded) return;
    const img = new Image();
    img.onload = () => { setThumbLoaded(true); setStage("thumbnail-loaded"); };
    img.onerror = () => { setStage("loading-full"); };
    img.src = thumbnailSrc;
  }, [thumbnailSrc, thumbLoaded]);

  useEffect(() => {
    if (stage !== "thumbnail-loaded" && stage !== "loading-full") return;
    if (fullLoaded) return;
    const img = new Image();
    img.onload = () => {
      setFullLoaded(true);
      setStage("full-loaded");
      trackTexture({
        id: src, assetId: alt, width: img.naturalWidth, height: img.naturalHeight,
        format: "img", bytes: img.naturalWidth * img.naturalHeight * 4,
        state: "resident", priority,
      });
      onLoad?.();
    };
    img.onerror = () => setStage("error");
    img.src = src;
    return () => { img.onload = null; img.onerror = null; };
  }, [stage, src, alt, onLoad, trackTexture, priority, fullLoaded]);

  return (
    <div className={`relative overflow-hidden ${className}`} style={{ background: placeholderColor, ...style }}>
      {thumbLoaded && thumbnailSrc && !fullLoaded && (
        <img ref={imgRef} src={thumbnailSrc} alt={alt}
          className="absolute inset-0 w-full h-full object-cover"
          style={{ filter: "blur(20px)", transform: "scale(1.1)", opacity: 0.6 }} />
      )}
      {fullLoaded && (
        <img ref={fullRef} src={src} alt={alt}
          className="w-full h-full object-contain animate-fade-in"
          style={{ opacity: 1 }} />
      )}
      {(stage === "idle" || stage === "loading-thumbnail") && !thumbLoaded && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="w-6 h-6 rounded-full border-2 border-t-transparent animate-spin"
            style={{ borderColor: "var(--evoury-accent)", borderTopColor: "transparent" }} />
        </div>
      )}
      {stage === "error" && (
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          <i className="fas fa-exclamation-triangle text-lg" style={{ color: "var(--evoury-text-dim)" }} />
          <span className="text-xs mt-1" style={{ color: "var(--evoury-text-dim)" }}>Failed to load</span>
        </div>
      )}
    </div>
  );
}
