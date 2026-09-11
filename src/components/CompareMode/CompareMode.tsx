import { useEffect, useCallback } from "react";
import { useCompareStore } from "../../stores/compare-store";
import type { CompareMode } from "../../stores/compare-store";
import type { Asset } from "../../types";

const modes: { value: CompareMode; label: string; icon: string }[] = [
  { value: "side-by-side", label: "Side by Side", icon: "fa-columns" },
  { value: "overlay", label: "Overlay", icon: "fa-layer-group" },
  { value: "slider", label: "Slider", icon: "fa-arrows-left-right" },
  { value: "diff", label: "Diff", icon: "fa-not-equal" },
  { value: "swipe", label: "Swipe", icon: "fa-hand-pointer" },
  { value: "split", label: "Split", icon: "fa-arrows-alt-h" },
  { value: "toggle", label: "Toggle", icon: "fa-exchange-alt" },
  { value: "difference", label: "Difference", icon: "fa-minus-circle" },
  { value: "blend", label: "Blend", icon: "fa-adjust" },
];

interface Props {
  assets: Asset[];
}

export default function CompareMode({ assets }: Props) {
  const {
    open, mode, leftAssetId, rightAssetId, sliderPosition, zoom,
    closeCompare, setMode, setSliderPosition, setZoom, swapAssets,
  } = useCompareStore();
  const leftAsset = assets.find((a) => a.id === leftAssetId) ?? null;
  const rightAsset = assets.find((a) => a.id === rightAssetId) ?? null;

  const onMouseMove = useCallback((e: MouseEvent) => {
    if (mode === "slider" || mode === "swipe" || mode === "split") {
      const rect = (e.currentTarget as HTMLElement)?.getBoundingClientRect();
      if (rect) {
        setSliderPosition((e.clientX - rect.left) / rect.width);
      }
    }
  }, [mode, setSliderPosition]);

  useEffect(() => {
    if (!open) return;
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") closeCompare();
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [open, closeCompare]);

  if (!open) return null;

  const renderImage = (asset: typeof leftAsset, label: string) => (
    asset?.preview?.path ? (
      <img
        src={asset.preview.path}
        alt={asset.name}
        style={{ transform: `scale(${zoom})`, objectFit: "contain", width: "100%", height: "100%" }}
        className="transition-transform duration-200"
      />
    ) : (
      <div className="flex flex-col items-center justify-center h-full">
        <i className="fas fa-image text-3xl mb-2" style={{ color: "var(--evoury-text-dim)" }} />
        <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>{label}</span>
      </div>
    )
  );

  const renderComparison = () => {
    switch (mode) {
      case "side-by-side":
        return (
          <div className="flex flex-1 gap-2">
            <div className="flex-1 rounded-xl overflow-hidden flex items-center justify-center" style={{ background: "var(--evoury-elevated)" }}>
              {renderImage(leftAsset, "Select left asset")}
            </div>
            <div className="flex-1 rounded-xl overflow-hidden flex items-center justify-center" style={{ background: "var(--evoury-elevated)" }}>
              {renderImage(rightAsset, "Select right asset")}
            </div>
          </div>
        );
      case "slider":
      case "swipe":
      case "split":
        return (
          <div className="flex-1 rounded-xl overflow-hidden relative" style={{ background: "var(--evoury-elevated)" }}
            onMouseMove={onMouseMove as any}>
            <div className="absolute inset-0">{renderImage(rightAsset, "Right")}</div>
            <div className="absolute inset-0 overflow-hidden" style={{ width: `${sliderPosition * 100}%` }}>
              {renderImage(leftAsset, "Left")}
            </div>
            <div className="absolute top-0 bottom-0 w-0.5 cursor-col-resize" style={{ left: `${sliderPosition * 100}%`, background: "var(--evoury-accent)", transform: "translateX(-50%)" }}>
              <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-6 h-6 rounded-full flex items-center justify-center"
                style={{ background: "var(--evoury-accent)" }}>
                <i className="fas fa-arrows-left-right text-[10px]" style={{ color: "#fff" }} />
              </div>
            </div>
          </div>
        );
      case "overlay":
        return (
          <div className="flex-1 rounded-xl overflow-hidden relative" style={{ background: "var(--evoury-elevated)" }}>
            <div className="absolute inset-0 opacity-50">{renderImage(leftAsset, "Left")}</div>
            <div className="absolute inset-0" style={{ opacity: sliderPosition }}>
              {renderImage(rightAsset, "Right")}
            </div>
            <input type="range" min="0" max="1" step="0.01" value={sliderPosition}
              onChange={(e) => setSliderPosition(Number(e.target.value))}
              className="absolute bottom-4 left-4 right-4 w-auto" />
          </div>
        );
      case "toggle":
        return (
          <div className="flex-1 rounded-xl overflow-hidden relative" style={{ background: "var(--evoury-elevated)" }}
            onDoubleClick={() => setSliderPosition(sliderPosition > 0.5 ? 0 : 1)}>
            {sliderPosition > 0.5 ? renderImage(leftAsset, "Left") : renderImage(rightAsset, "Right")}
            <div className="absolute bottom-4 left-1/2 -translate-x-1/2 px-3 py-1.5 rounded-lg text-xs"
              style={{ background: "rgba(0,0,0,0.6)", color: "var(--evoury-text)", backdropFilter: "blur(8px)" }}>
              Double-click to toggle
            </div>
          </div>
        );
      case "difference":
      case "blend":
        return (
          <div className="flex-1 rounded-xl overflow-hidden relative" style={{ background: "var(--evoury-elevated)" }}>
            <div className="absolute inset-0">{renderImage(leftAsset, "Left")}</div>
            <div className="absolute inset-0" style={{ mixBlendMode: mode === "difference" ? "difference" : "multiply", opacity: sliderPosition }}>
              {renderImage(rightAsset, "Right")}
            </div>
            <input type="range" min="0" max="1" step="0.01" value={sliderPosition}
              onChange={(e) => setSliderPosition(Number(e.target.value))}
              className="absolute bottom-4 left-4 right-4 w-auto" />
          </div>
        );
      case "diff":
        return (
          <div className="flex-1 rounded-xl overflow-hidden relative flex items-center justify-center" style={{ background: "var(--evoury-elevated)" }}>
            <canvas className="max-w-full max-h-full" ref={(el) => {
              if (!el || !leftAsset?.preview?.path || !rightAsset?.preview?.path) return;
              const ctx = el.getContext("2d");
              if (!ctx) return;
              loadImage(leftAsset.preview.path).then((img1) => {
                loadImage(rightAsset!.preview!.path).then((img2) => {
                  el.width = Math.max(img1.width, img2.width);
                  el.height = Math.max(img1.height, img2.height);
                  ctx.drawImage(img1, 0, 0);
                  const data1 = ctx.getImageData(0, 0, el.width, el.height);
                  ctx.drawImage(img2, 0, 0);
                  const data2 = ctx.getImageData(0, 0, el.width, el.height);
                  const diff = new ImageData(el.width, el.height);
                  for (let i = 0; i < data1.data.length; i += 4) {
                    diff.data[i] = Math.abs(data1.data[i] - data2.data[i]);
                    diff.data[i + 1] = Math.abs(data1.data[i + 1] - data2.data[i + 1]);
                    diff.data[i + 2] = Math.abs(data1.data[i + 2] - data2.data[i + 2]);
                    diff.data[i + 3] = 255;
                  }
                  ctx.putImageData(diff, 0, 0);
                });
              });
            }} />
          </div>
        );
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex flex-col" style={{ background: "rgba(0,0,0,0.92)" }}>
      <div className="flex items-center justify-between px-5 py-3"
        style={{ background: "rgba(10, 10, 15, 0.8)", backdropFilter: "blur(20px)", borderBottom: "1px solid var(--evoury-border)" }}>
        <div className="flex items-center gap-3">
          <i className="fas fa-not-equal" style={{ color: "var(--evoury-accent)" }} />
          <span className="text-sm font-medium" style={{ color: "var(--evoury-text)" }}>Compare</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1 rounded-lg p-0.5"
            style={{ background: "var(--evoury-elevated)", border: "1px solid var(--evoury-border)" }}>
            {modes.map((m) => (
              <button key={m.value} onClick={() => setMode(m.value)}
                className={`w-7 h-7 flex items-center justify-center rounded-md text-[10px] transition-all ${mode === m.value ? "text-white" : ""}`}
                style={mode === m.value ? { background: "var(--evoury-accent)" } : { color: "var(--evoury-text-dim)" }}
                title={m.label}>
                <i className={`fas ${m.icon}`} />
              </button>
            ))}
          </div>
          <button onClick={() => setZoom(zoom - 0.25)}
            className="w-7 h-7 flex items-center justify-center rounded-md text-xs"
            style={{ color: "var(--evoury-text-dim)" }}>
            <i className="fas fa-minus" />
          </button>
          <span className="px-2 text-xs font-mono" style={{ color: "var(--evoury-text)" }}>{Math.round(zoom * 100)}%</span>
          <button onClick={() => setZoom(zoom + 0.25)}
            className="w-7 h-7 flex items-center justify-center rounded-md text-xs"
            style={{ color: "var(--evoury-text-dim)" }}>
            <i className="fas fa-plus" />
          </button>
          <button onClick={swapAssets}
            className="w-7 h-7 flex items-center justify-center rounded-md text-xs"
            style={{ color: "var(--evoury-text-dim)" }} title="Swap assets">
            <i className="fas fa-arrows-alt-v" />
          </button>
          <button onClick={closeCompare}
            className="w-7 h-7 flex items-center justify-center rounded-lg transition-all"
            style={{ color: "var(--evoury-text-dim)" }}>
            <i className="fas fa-times" />
          </button>
        </div>
      </div>
      {renderComparison()}
    </div>
  );
}

function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = src;
  });
}
