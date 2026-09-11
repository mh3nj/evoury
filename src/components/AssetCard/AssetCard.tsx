import { useState, useRef, useEffect } from "react";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import type { Asset } from "../../types";
import { useBasketStore } from "../../stores";

interface Props {
  asset: Asset;
  onOpenExternal?: (asset: Asset) => void;
  onOpenPath?: (asset: Asset) => void;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

export default function AssetCard({ asset, onOpenExternal, onOpenPath }: Props) {
  const [showBasketMenu, setShowBasketMenu] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const isArchive = asset.archive_type !== "Unknown";
  const previewUrl = asset.preview?.path ? convertFileSrc(asset.preview.path) : null;
  const assetFile = asset.archive_path;
  const baskets = useBasketStore((s) => s.baskets);
  const addToBasket = useBasketStore((s) => s.addToBasket);

  useEffect(() => {
    function handler(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setShowBasketMenu(false);
      }
    }
    if (showBasketMenu) window.addEventListener("mousedown", handler);
    return () => window.removeEventListener("mousedown", handler);
  }, [showBasketMenu]);

  return (
    <div
      onClick={() => { if (assetFile) onOpenExternal?.(asset); }}
      className="group rounded-xl cursor-pointer transition-all duration-200"
      style={{
        background: "var(--evoury-surface)",
        border: "1px solid var(--evoury-border)",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.borderColor = "var(--evoury-border-light)";
        e.currentTarget.style.transform = "translateY(-1px)";
        e.currentTarget.style.boxShadow = "0 4px 16px rgba(0,0,0,0.2)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.borderColor = "var(--evoury-border)";
        e.currentTarget.style.transform = "translateY(0)";
        e.currentTarget.style.boxShadow = "none";
      }}
    >
      {/* Thumbnail */}
      <div className="relative overflow-hidden" style={{ paddingBottom: "75%", borderRadius: "11px 11px 0 0" }}>
        {previewUrl ? (
          <img
            src={previewUrl}
            alt={asset.name}
            className="absolute inset-0 w-full h-full object-cover"
            loading="lazy"
            onError={(e) => { (e.target as HTMLImageElement).style.display = "none"; }}
          />
        ) : (
          <div className="absolute inset-0 flex items-center justify-center" style={{ background: "var(--evoury-elevated)" }}>
            <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: "rgba(0,0,0,0.3)" }}>
              <i className={`fas ${isArchive ? "fa-file-archive" : "fa-file"} text-lg`} style={{ color: "var(--evoury-text-dim)" }} />
            </div>
          </div>
        )}

        {/* Hover actions */}
        <div className="absolute inset-0 bg-black/0 group-hover:bg-black/40 transition-all duration-200 flex items-center justify-center gap-1.5 opacity-0 group-hover:opacity-100">
          <button onClick={(e) => { e.stopPropagation(); if (assetFile) onOpenExternal?.(asset); }}
            className="w-7 h-7 rounded-lg flex items-center justify-center text-[10px] text-white transition-all"
            style={{ background: "rgba(255,255,255,0.15)", backdropFilter: "blur(4px)" }}
            title="Open with default app">
            <i className="fas fa-external-link-alt" />
          </button>
          <button onClick={(e) => { e.stopPropagation(); onOpenPath?.(asset); }}
            className="w-7 h-7 rounded-lg flex items-center justify-center text-[10px] text-white transition-all"
            style={{ background: "rgba(255,255,255,0.15)", backdropFilter: "blur(4px)" }}
            title="Show in Explorer">
            <i className="fas fa-folder-open" />
          </button>
          <div className="relative">
            <button onClick={(e) => { e.stopPropagation(); setShowBasketMenu(!showBasketMenu); }}
              className="w-7 h-7 rounded-lg flex items-center justify-center text-[10px] text-white transition-all"
              style={{ background: showBasketMenu ? "var(--evoury-accent)" : "rgba(255,255,255,0.15)", backdropFilter: "blur(4px)" }}
              title="Add to basket">
              <i className="fas fa-shopping-basket" />
            </button>
            {showBasketMenu && (
              <div ref={menuRef} className="absolute bottom-full left-1/2 -translate-x-1/2 mb-1 z-50 animate-slide-up"
                style={{ background: "var(--evoury-surface)", border: "1px solid var(--evoury-border)", borderRadius: 8, minWidth: 140, boxShadow: "0 8px 32px rgba(0,0,0,0.4)" }}>
                <div className="py-1">
                  <div className="px-3 py-1 text-[9px] font-semibold uppercase tracking-wider" style={{ color: "var(--evoury-text-dim)" }}>Add to basket</div>
                  {baskets.length === 0 && (
                    <div className="px-3 py-2 text-[10px]" style={{ color: "var(--evoury-border-light)" }}>No baskets yet</div>
                  )}
                  {baskets.map((b) => (
                    <button key={b.id} onClick={(e) => { e.stopPropagation(); addToBasket(b.id, asset); setShowBasketMenu(false); }}
                      className="w-full text-left px-3 py-1.5 text-[10px] transition-all flex items-center gap-2"
                      style={{ color: "var(--evoury-text-dim)" }}
                      onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-accent-glow)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
                      onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
                      <i className="fas fa-shopping-basket text-[8px]" style={{ color: "var(--evoury-accent)" }} />
                      {b.name}
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Archive badge */}
        <div className="absolute top-2 left-2">
          <span className="px-1.5 py-0.5 rounded text-[8px] font-medium uppercase tracking-wider"
            style={{ background: "rgba(0,0,0,0.55)", color: "var(--evoury-text-dim)", backdropFilter: "blur(6px)" }}>
            {asset.archive_type}
          </span>
        </div>
      </div>

      {/* Info */}
      <div className="p-2">
        <p className="text-[11px] font-medium truncate" style={{ color: "var(--evoury-text)" }}>{asset.name}</p>
        <div className="flex items-center gap-2 mt-0.5">
          <span className="text-[9px]" style={{ color: "var(--evoury-text-dim)" }}>
            <i className="fas fa-hard-drive mr-0.5" />{formatSize(asset.file_size)}
          </span>
        </div>
      </div>
    </div>
  );
}
