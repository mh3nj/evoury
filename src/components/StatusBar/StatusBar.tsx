import { useScanStore } from "../../stores";

interface Props {
  assetCount?: number;
  status?: string;
}

export default function StatusBar({ assetCount = 0, status = "Ready" }: Props) {
  const { scanning, totalFiles, scannedFiles, currentFile } = useScanStore();
  const progress = totalFiles > 0 ? Math.round((scannedFiles / totalFiles) * 100) : 0;

  return (
    <div className="flex items-center justify-between px-4 py-1.5 text-[11px]"
      style={{ background: "var(--evoury-surface)", borderTop: "1px solid var(--evoury-border)", color: "var(--evoury-text-dim)" }}>
      <div className="flex items-center gap-4 flex-1">
        <span className="flex items-center gap-1.5 shrink-0">
          <span className="w-1.5 h-1.5 rounded-full"
            style={{ background: scanning ? "var(--evoury-accent)" : status === "Ready" ? "var(--evoury-success)" : "var(--evoury-warning)" }} />
          {scanning ? `Scanning ${scannedFiles}/${totalFiles}` : status}
        </span>
        {scanning && totalFiles > 0 && (
          <div className="flex-1 max-w-xs">
            <div className="h-1 rounded-full overflow-hidden" style={{ background: "var(--evoury-elevated)" }}>
              <div className="h-full rounded-full transition-all duration-300"
                style={{ width: `${progress}%`, background: "linear-gradient(90deg, var(--evoury-accent), var(--evoury-accent2))" }} />
            </div>
          </div>
        )}
        {currentFile && scanning && (
          <span className="truncate max-w-[200px] text-[10px]" style={{ color: "var(--evoury-border-light)" }}>
            {currentFile}
          </span>
        )}
        <span className="w-px h-3 shrink-0" style={{ background: "var(--evoury-border)" }} />
        <span className="shrink-0">{assetCount.toLocaleString()} assets</span>
        {assetCount > 0 && !scanning && (
          <>
            <span className="w-px h-3 shrink-0" style={{ background: "var(--evoury-border)" }} />
            <span className="text-gradient font-medium shrink-0">Enter the Flow</span>
          </>
        )}
      </div>
      <span className="shrink-0" style={{ color: "var(--evoury-text-dim)" }}>
        Evoury <span style={{ color: "var(--evoury-border-light)" }}>v0.1.0</span>
      </span>
    </div>
  );
}
