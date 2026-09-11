import { useEffect, useState, useRef } from "react";
import type { Asset } from "../../types";

interface Props {
  asset: Asset | null;
  onClose: () => void;
  onNext?: () => void;
  onPrev?: () => void;
}

export default function QuickPreview({ asset, onClose, onNext, onPrev }: Props) {
  const [loaded, setLoaded] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const imgRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    if (!asset) return;
    setLoaded(false);
    timerRef.current = setTimeout(() => setLoaded(true), 80);
    return () => { if (timerRef.current) clearTimeout(timerRef.current); };
  }, [asset]);

  useEffect(() => {
    if (!asset) return;
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") { e.preventDefault(); onClose(); }
      if (e.key === "ArrowRight" && onNext) { e.preventDefault(); onNext(); }
      if (e.key === "ArrowLeft" && onPrev) { e.preventDefault(); onPrev(); }
      if (e.key === " " || e.key === "q") { e.preventDefault(); onClose(); }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [asset, onClose, onNext, onPrev]);

  if (!asset) return null;

  const showImage = loaded && asset.preview?.path;

  return (
    <div
      className="fixed inset-0 z-50 flex flex-col animate-fade-in"
      style={{ background: "#1a1a2e" }}
    >
      <div className="flex items-center justify-between px-4 py-2.5"
        style={{ background: "rgba(15,15,25,0.9)", borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
        <div className="flex items-center gap-3">
          <button onClick={onClose}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs transition-all"
            style={{ background: "rgba(255,255,255,0.06)", color: "#c0c0d0" }}
            onMouseEnter={(e) => { e.currentTarget.style.background = "rgba(255,255,255,0.12)"; e.currentTarget.style.color = "#fff"; }}
            onMouseLeave={(e) => { e.currentTarget.style.background = "rgba(255,255,255,0.06)"; e.currentTarget.style.color = "#c0c0d0"; }}>
            <i className="fas fa-arrow-left text-[10px]" />
            Back
          </button>
          <span className="w-px h-4" style={{ background: "rgba(255,255,255,0.08)" }} />
          <span className="text-xs font-medium truncate max-w-[300px]" style={{ color: "#e0e0f0" }}>{asset.name}</span>
        </div>
        <div className="flex items-center gap-2 text-[10px]" style={{ color: "#8888a0" }}>
          <span><i className="fas fa-arrow-left mr-1" />Prev</span>
          <span className="mx-1">·</span>
          <span>Esc to close</span>
          <span className="mx-1">·</span>
          <span>Next<i className="fas fa-arrow-right ml-1" /></span>
        </div>
      </div>
      <div className="flex-1 flex items-center justify-center p-6">
        {showImage ? (
          <img
            ref={imgRef}
            src={asset.preview!.path}
            alt={asset.name}
            className="max-w-[90vw] max-h-[80vh] object-contain select-none animate-fade-in"
            style={{ borderRadius: "8px" }}
            onError={() => setLoaded(false)}
          />
        ) : (
          <div className="flex flex-col items-center gap-3">
            <div className="w-14 h-14 rounded-2xl flex items-center justify-center"
              style={{ background: "rgba(255,255,255,0.06)" }}>
              <i className="fas fa-image text-xl" style={{ color: "#6666a0" }} />
            </div>
            <span className="text-sm" style={{ color: "#8888a0" }}>{asset.name}</span>
            {!loaded && (
              <div className="w-6 h-6 rounded-full border-2 border-t-transparent animate-spin"
                style={{ borderColor: "#6366f1", borderTopColor: "transparent" }} />
            )}
          </div>
        )}
      </div>
      <div className="flex items-center justify-center gap-3 px-4 py-3"
        style={{ background: "rgba(15,15,25,0.9)", borderTop: "1px solid rgba(255,255,255,0.06)" }}>
        <button onClick={onPrev}
          className="px-4 py-1.5 text-xs rounded-lg transition-all"
          style={{ background: onPrev ? "rgba(255,255,255,0.08)" : "transparent", color: onPrev ? "#c0c0d0" : "#444460", cursor: onPrev ? "pointer" : "default" }}>
          <i className="fas fa-chevron-left mr-1.5 text-[10px]" />Previous
        </button>
        <button onClick={onClose}
          className="px-4 py-1.5 text-xs rounded-lg transition-all"
          style={{ background: "rgba(255,255,255,0.08)", color: "#c0c0d0" }}
          onMouseEnter={(e) => { e.currentTarget.style.background = "rgba(255,255,255,0.14)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.background = "rgba(255,255,255,0.08)"; }}>
          <i className="fas fa-times mr-1.5 text-[10px]" />Close
        </button>
        <button onClick={onNext}
          className="px-4 py-1.5 text-xs rounded-lg transition-all"
          style={{ background: onNext ? "rgba(255,255,255,0.08)" : "transparent", color: onNext ? "#c0c0d0" : "#444460", cursor: onNext ? "pointer" : "default" }}>
          Next<i className="fas fa-chevron-right ml-1.5 text-[10px]" />
        </button>
      </div>
    </div>
  );
}
