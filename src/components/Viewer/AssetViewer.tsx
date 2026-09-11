import { useEffect, useCallback } from "react";
import { useViewerStore, useViewerEngineStore, useCompareStore } from "../../stores";
import type { Asset } from "../../types";
import { getViewerComponent, detectViewerKind } from "./SpecializedViewers";
import { ColorIndicator } from "../ColorIndicator";

interface Props {
  assets: Asset[];
}

export default function AssetViewer({ assets }: Props) {
  const { open, assetId, zoom, closeViewer, setZoom } = useViewerStore();
  const { activeViewer, setViewer, setZoom: setEngineZoom } = useViewerEngineStore();
  const currentIndex = assets.findIndex((a) => a.id === assetId);
  const asset = assets[currentIndex];

  useEffect(() => {
    if (asset) {
      const kind = detectViewerKind(asset.name, asset.mime_type);
      setViewer(kind);
    }
  }, [asset, setViewer]);

  const goNext = useCallback(() => {
    if (currentIndex < assets.length - 1) {
      useViewerStore.getState().openViewer(assets[currentIndex + 1].id);
    }
  }, [currentIndex, assets]);

  const goPrev = useCallback(() => {
    if (currentIndex > 0) {
      useViewerStore.getState().openViewer(assets[currentIndex - 1].id);
    }
  }, [currentIndex, assets]);

  useEffect(() => {
    if (!open) return;
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") closeViewer();
      if (e.key === "ArrowRight") goNext();
      if (e.key === "ArrowLeft") goPrev();
      if (e.key === "+" || e.key === "=") { setZoom(zoom + 0.25); setEngineZoom(zoom + 0.25); }
      if (e.key === "-") { const z = Math.max(0.25, zoom - 0.25); setZoom(z); setEngineZoom(z); }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [open, zoom, closeViewer, goNext, goPrev, setZoom, setEngineZoom]);

  const handleOpenCompare = useCallback(() => {
    if (!asset) return;
    const compare = useCompareStore.getState();
    if (!compare.open) {
      compare.openCompare(asset.id, assets[(currentIndex + 1) % assets.length]?.id || asset.id);
    }
  }, [asset, assets, currentIndex]);

  if (!open || !asset) return null;

  const ViewerComponent = getViewerComponent(activeViewer);

  return (
    <div className="fixed inset-0 z-50 flex flex-col animate-fade-in" style={{ background: "rgba(0,0,0,0.92)" }}>
      <div className="flex items-center justify-between px-5 py-3"
        style={{ background: "rgba(10, 10, 15, 0.8)", backdropFilter: "blur(20px)", borderBottom: "1px solid var(--evoury-border)" }}>
        <div className="flex items-center gap-3 min-w-0">
          <div className="w-6 h-6 rounded-lg flex items-center justify-center text-white text-[10px] font-bold flex-shrink-0"
            style={{ background: "linear-gradient(135deg, var(--evoury-accent), var(--evoury-accent2))" }}>
            E
          </div>
          <span className="text-sm truncate font-medium" style={{ color: "var(--evoury-text)" }}>{asset.name}</span>
          <span className="text-xs font-mono flex-shrink-0" style={{ color: "var(--evoury-text-dim)" }}>
            {currentIndex + 1} / {assets.length}
          </span>
          <span className="text-[10px] px-1.5 py-0.5 rounded font-mono"
            style={{ background: "var(--evoury-elevated)", color: "var(--evoury-accent)" }}>
            {activeViewer}
          </span>
        </div>
        <div className="flex items-center gap-3 flex-shrink-0">
          <ColorIndicator />
          <div className="flex items-center gap-1 rounded-lg p-0.5"
            style={{ background: "var(--evoury-elevated)", border: "1px solid var(--evoury-border)" }}>
            <button onClick={() => { setZoom(zoom - 0.25); setEngineZoom(zoom - 0.25); }}
              className="w-7 h-7 flex items-center justify-center rounded-md text-xs transition-all"
              style={{ color: "var(--evoury-text-dim)" }}
              onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-bg)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
              onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
              <i className="fas fa-minus" />
            </button>
            <span className="px-2 text-xs font-mono" style={{ color: "var(--evoury-text)" }}>
              {Math.round(zoom * 100)}%
            </span>
            <button onClick={() => { setZoom(zoom + 0.25); setEngineZoom(zoom + 0.25); }}
              className="w-7 h-7 flex items-center justify-center rounded-md text-xs transition-all"
              style={{ color: "var(--evoury-text-dim)" }}
              onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-bg)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
              onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
              <i className="fas fa-plus" />
            </button>
          </div>
          <button onClick={handleOpenCompare}
            className="w-7 h-7 flex items-center justify-center rounded-md text-xs transition-all"
            style={{ color: "var(--evoury-text-dim)" }}
            onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-accent)"; }}
            onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
            <i className="fas fa-not-equal" />
          </button>
          <button onClick={closeViewer}
            className="w-7 h-7 flex items-center justify-center rounded-lg transition-all duration-200"
            style={{ color: "var(--evoury-text-dim)" }}
            onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
            onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
            <i className="fas fa-times" />
          </button>
        </div>
      </div>

      <div className="flex-1 flex items-center justify-center overflow-auto p-8"
        style={{ transform: `scale(${activeViewer === "default-image" ? zoom : 1})` }}>
        <ViewerComponent src={asset.preview?.path} assetName={asset.name} />
      </div>

      <div className="flex items-center justify-center gap-4 px-4 py-3"
        style={{ background: "rgba(10, 10, 15, 0.8)", backdropFilter: "blur(20px)", borderTop: "1px solid var(--evoury-border)" }}>
        <button onClick={goPrev}
          className="px-4 py-2 text-sm rounded-lg transition-all duration-200 flex items-center gap-2"
          style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}
          onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
          <i className="fas fa-chevron-left text-[10px]" /> Previous
        </button>
        <button onClick={closeViewer}
          className="px-4 py-2 text-sm rounded-lg transition-all duration-200"
          style={{ background: "transparent", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border-light)" }}
          onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
          <i className="fas fa-times mr-1.5 text-[10px]" /> Close (Esc)
        </button>
        <button onClick={goNext}
          className="px-4 py-2 text-sm rounded-lg transition-all duration-200 flex items-center gap-2"
          style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}
          onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
          Next <i className="fas fa-chevron-right text-[10px]" />
        </button>
      </div>
    </div>
  );
}
