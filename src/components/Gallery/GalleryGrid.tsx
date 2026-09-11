import { useMemo, useRef, useState, useEffect } from "react";
import type { Asset } from "../../types";
import { AssetCard } from "../AssetCard";
import { useGalleryStore, useScanStore } from "../../stores";

interface Props {
  assets: Asset[];
  onOpenExternal?: (asset: Asset) => void;
  onOpenPath?: (asset: Asset) => void;
}

const PADDING = 32;
const GAP = 12;

function isPsd(name: string, mime?: string): boolean {
  return name.toLowerCase().endsWith(".psd") || (mime?.includes("photoshop") ?? false);
}

export default function GalleryGrid({ assets, onOpenExternal, onOpenPath }: Props) {
  const gridSize = useGalleryStore((s) => s.gridSize);
  const viewMode = useGalleryStore((s) => s.viewMode);
  const scanning = useScanStore((s) => s.scanning);
  const containerRef = useRef<HTMLDivElement>(null);
  const gridRef = useRef<HTMLDivElement>(null);
  const [containerWidth, setContainerWidth] = useState(800);

  const filteredAssets = useMemo(
    () => assets.filter((a) => !isPsd(a.name, a.mime_type)),
    [assets]
  );

  useEffect(() => {
    const el = containerRef.current;
    if (!el) return;
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) setContainerWidth(entry.contentRect.width);
    });
    observer.observe(el);
    return () => observer.disconnect();
  }, []);

  // Card size based on viewMode
  const cardSize = useMemo(() => {
    if (viewMode === "compact") return Math.max(90, Math.min(160, gridSize * 0.5));
    if (viewMode === "comfortable") return Math.max(220, Math.min(500, gridSize * 1.3));
    return Math.max(140, Math.min(350, gridSize));
  }, [viewMode, gridSize]);

  // Columns that fit
  const columns = useMemo(() => {
    const available = containerWidth - PADDING;
    return Math.max(2, Math.floor(available / (cardSize + GAP)));
  }, [containerWidth, cardSize]);

  // Ctrl+scroll zoom
  useEffect(() => {
    const el = gridRef.current;
    if (!el) return;
    function handler(e: WheelEvent) {
      if (e.ctrlKey || e.metaKey) {
        e.preventDefault();
        const step = e.deltaY < 0 ? 12 : -12;
        const gs = useGalleryStore.getState();
        gs.setGridSize(Math.max(80, Math.min(600, gs.gridSize + step)));
      }
    }
    el.addEventListener("wheel", handler, { passive: false });
    return () => el.removeEventListener("wheel", handler);
  }, []);

  if (filteredAssets.length === 0 && !scanning) {
    return (
      <div className="flex-1 flex items-center justify-center"
        style={{ background: "var(--evoury-bg)" }}>
        <div className="text-center animate-fade-in">
          <div className="w-16 h-16 rounded-2xl flex items-center justify-center mx-auto mb-4"
            style={{ background: "var(--evoury-elevated)" }}>
            <i className="fas fa-images text-2xl" style={{ color: "var(--evoury-text-dim)" }} />
          </div>
          <p className="text-sm font-medium" style={{ color: "var(--evoury-text-dim)" }}>No assets yet</p>
          <p className="text-xs mt-1" style={{ color: "var(--evoury-border-light)" }}>Scan a folder to get started</p>
        </div>
      </div>
    );
  }

  return (
    <div ref={containerRef} className="flex-1 flex flex-col overflow-hidden">
      {scanning && (
        <div className="px-5 py-2 text-[10px] flex items-center gap-2 animate-fade-in"
          style={{ background: "var(--evoury-surface)", borderBottom: "1px solid var(--evoury-border)", color: "var(--evoury-accent)" }}>
          <i className="fas fa-spinner fa-pulse" />
          Scanning...
        </div>
      )}
      <div ref={gridRef} className="flex-1 overflow-auto" style={{ background: "var(--evoury-bg)" }}>
        <div className="p-4"
          style={{
            display: "grid",
            gridTemplateColumns: `repeat(${columns}, 1fr)`,
            gap: `${GAP}px`,
          }}>
          {filteredAssets.map((asset) => (
            <AssetCard
              key={asset.id}
              asset={asset}
              onOpenExternal={onOpenExternal}
              onOpenPath={onOpenPath}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
