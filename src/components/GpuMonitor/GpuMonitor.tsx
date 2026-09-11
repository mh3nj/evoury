import { useState } from "react";
import { useGpuStore } from "../../stores/gpu-store";

export default function GpuMonitor() {
  const [expanded, setExpanded] = useState(false);
  const { maxVramMb, textures, totalVramBytes, residentCount, evictTexture, setMaxVram, setMaxTextures } = useGpuStore();
  const totalMb = totalVramBytes / (1024 * 1024);
  const maxBytes = maxVramMb * 1024 * 1024;
  const usage = maxBytes > 0 ? Math.min(100, (totalVramBytes / maxBytes) * 100) : 0;

  return (
    <div style={{ borderTop: "1px solid var(--evoury-border)" }}>
      <button onClick={() => setExpanded(!expanded)}
        className="w-full flex items-center justify-between px-4 py-2 text-xs"
        style={{ color: "var(--evoury-text-dim)" }}>
        <div className="flex items-center gap-2">
          <i className="fas fa-microchip" />
          <span>GPU Memory</span>
          <span className="font-mono">{totalMb.toFixed(1)} MB / {maxVramMb} MB</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-20 h-1.5 rounded-full" style={{ background: "var(--evoury-elevated)" }}>
            <div className="h-full rounded-full transition-all" style={{
              width: `${usage}%`,
              background: usage > 80 ? "var(--evoury-error, #ef4444)" : usage > 50 ? "var(--evoury-warning, #f59e0b)" : "var(--evoury-accent)",
            }} />
          </div>
          <span className="font-mono">{residentCount}</span>
          <i className={`fas fa-chevron-${expanded ? "down" : "up"} text-[8px]`} />
        </div>
      </button>
      {expanded && (
        <div className="px-4 pb-2 space-y-2">
          <div className="flex items-center gap-2 text-[10px]" style={{ color: "var(--evoury-text-dim)" }}>
            <span>Max VRAM:</span>
            <input type="range" min="128" max="8192" step="128" value={maxVramMb}
              onChange={(e) => setMaxVram(Number(e.target.value))}
              className="flex-1" />
            <span className="font-mono w-12 text-right">{maxVramMb}MB</span>
          </div>
          <div className="flex items-center gap-2 text-[10px]" style={{ color: "var(--evoury-text-dim)" }}>
            <span>Max Textures:</span>
            <input type="range" min="16" max="1024" step="16" value={textures.length > 0 ? Math.max(textures.length, 16) : 256}
              onChange={(e) => setMaxTextures(Number(e.target.value))}
              className="flex-1" />
          </div>
          <div className="max-h-32 overflow-y-auto space-y-1">
            {textures.filter((t) => t.state === "resident").map((tex) => (
              <div key={tex.id} className="flex items-center justify-between text-[10px] rounded px-2 py-1"
                style={{ background: "var(--evoury-elevated)" }}>
                <span className="truncate flex-1" style={{ color: "var(--evoury-text)" }}>{tex.assetId}</span>
                <span className="font-mono mx-2" style={{ color: "var(--evoury-text-dim)" }}>
                  {tex.width}x{tex.height} ({(tex.bytes / 1024).toFixed(0)}KB)
                </span>
                <button onClick={() => evictTexture(tex.id)}
                  className="w-4 h-4 flex items-center justify-center rounded"
                  style={{ color: "var(--evoury-text-dim)" }}>
                  <i className="fas fa-trash-alt text-[8px]" />
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
