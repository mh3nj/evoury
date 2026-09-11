import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface DiagnosticsSnapshot {
  timestamp: string;
  worker_count: number;
  active_workers: number;
  pending_jobs: number;
  running_jobs: number;
  registered_commands: number;
  asset_count: number;
  cache_entries: number;
  power_mode: string;
  uptime_seconds: number;
}

function formatUptime(secs: number): string {
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = secs % 60;
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m ${s}s`;
  return `${s}s`;
}

export default function HealthPanel() {
  const [open, setOpen] = useState(false);
  const [diag, setDiag] = useState<DiagnosticsSnapshot | null>(null);

  const refresh = useCallback(async () => {
    try {
      const d = await invoke<DiagnosticsSnapshot>("get_diagnostics");
      setDiag(d);
    } catch { /* ignore */ }
  }, []);

  useEffect(() => {
    if (!open) return;
    refresh();
    const id = setInterval(refresh, 5000);
    return () => clearInterval(id);
  }, [open, refresh]);

  return (
    <div style={{ borderTop: "1px solid var(--evoury-border)", background: "var(--evoury-surface)" }}>
      <button
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between px-4 py-2.5 text-xs transition-all duration-200"
        style={{ color: "var(--evoury-text-dim)" }}
        onMouseEnter={(e) => { e.currentTarget.style.color = "var(--evoury-text)"; }}
        onMouseLeave={(e) => { e.currentTarget.style.color = "var(--evoury-text-dim)"; }}
      >
        <div className="flex items-center gap-2">
          <i className={`fas fa-chevron-${open ? "down" : "right"} text-[8px]`}
            style={{ color: "var(--evoury-text-dim)" }} />
          <i className="fas fa-stethoscope text-[10px]" style={{ color: "var(--evoury-accent)" }} />
          <span className="font-semibold uppercase tracking-wider text-[10px]">Diagnostics</span>
        </div>
        {diag && (
          <span className="text-[10px] font-mono" style={{ color: modeColor(diag.power_mode) }}>
            <i className={`fas fa-${diag.power_mode === "Active" ? "bolt" : diag.power_mode === "Idle" ? "pause" : "moon"} mr-1`} />
            {diag.power_mode}
          </span>
        )}
      </button>

      {open && diag && (
        <div className="px-4 pb-3 space-y-1.5 animate-slide-up">
          {/* Power Mode */}
          <Section title="Power Mode">
            <Row label="Mode" value={diag.power_mode} color={modeColor(diag.power_mode)} />
            <Row label="Uptime" value={formatUptime(diag.uptime_seconds)} />
          </Section>

          {/* Workers */}
          <Section title="Workers">
            <Row label="Total" value={String(diag.worker_count)} />
            <Row label="Active" value={String(diag.active_workers)} color={diag.active_workers > 0 ? "var(--evoury-accent)" : undefined} />
            <Row label="Pending Jobs" value={String(diag.pending_jobs)} />
            <Row label="Running Jobs" value={String(diag.running_jobs)} />
          </Section>

          {/* Library */}
          <Section title="Library">
            <Row label="Assets" value={String(diag.asset_count)} />
            <Row label="Cache Entries" value={String(diag.cache_entries)} />
            <Row label="Commands" value={String(diag.registered_commands)} />
          </Section>

          {/* Actions */}
          <div className="flex gap-2 mt-2">
            <button
              onClick={refresh}
              className="flex-1 px-3 py-1.5 text-xs rounded-lg transition-all duration-200"
              style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}
              onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
              onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
              <i className="fas fa-sync-alt mr-1.5 text-[10px]" />
              Refresh
            </button>
            <button
              onClick={async () => { await invoke("touch_activity"); refresh(); }}
              className="flex-1 px-3 py-1.5 text-xs rounded-lg transition-all duration-200"
              style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}
              onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
              onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
              <i className="fas fa-bolt mr-1.5 text-[10px]" />
              Wake
            </button>
          </div>
        </div>
      )}

      {open && !diag && (
        <div className="px-4 pb-3">
          <p className="text-[10px] text-center py-2" style={{ color: "var(--evoury-text-dim)" }}>
            Loading diagnostics...
          </p>
        </div>
      )}
    </div>
  );
}

function modeColor(mode: string): string {
  switch (mode) {
    case "Active": return "var(--evoury-success)";
    case "Idle": return "var(--evoury-warning)";
    case "Sleeping": return "var(--evoury-accent)";
    default: return "var(--evoury-text-dim)";
  }
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="pt-1.5">
      <h4 className="text-[9px] font-semibold uppercase tracking-wider mb-1" style={{ color: "var(--evoury-text-dim)" }}>
        {title}
      </h4>
      {children}
    </div>
  );
}

function Row({ label, value, color }: { label: string; value: string; color?: string }) {
  return (
    <div className="flex justify-between items-center">
      <span className="text-[10px]" style={{ color: "var(--evoury-text-dim)" }}>{label}</span>
      <span className="text-[10px] font-mono" style={{ color: color || "var(--evoury-text)" }}>{value}</span>
    </div>
  );
}
