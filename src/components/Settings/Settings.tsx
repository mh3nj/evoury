import { useEffect, useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useCommandStore, usePreferencesStore, useThemeStore } from "../../stores";

export default function Settings() {
  const { settingsOpen, closeSettings } = useCommandStore();
  const { loaded, load, theme, setTheme, default_view, setView, performance, setPerformance, interaction, setInteraction } = usePreferencesStore();
  const { setMode, setDensity, config } = useThemeStore();
  const [closeToTray, setCloseToTray] = useState(() => localStorage.getItem("evoury_close_to_tray") === "true");
  const [startOnBoot, setStartOnBoot] = useState(() => localStorage.getItem("evoury_start_on_boot") === "true");
  const [trayOnStart, setTrayOnStart] = useState(() => localStorage.getItem("evoury_tray_on_start") === "true");

  useEffect(() => {
    if (settingsOpen && !loaded) load();
  }, [settingsOpen, loaded, load]);

  useEffect(() => {
    if (!settingsOpen) return;
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") closeSettings();
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [settingsOpen, closeSettings]);

  const handleThemeChange = useCallback(async (t: "Dark" | "Light" | "System") => {
    await setTheme(t);
    await setMode(t);
  }, [setTheme, setMode]);

  const handleDensityChange = useCallback(async (d: "Compact" | "Normal" | "Comfortable") => {
    await setDensity(d);
  }, [setDensity]);

  if (!settingsOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center"
      style={{ background: "rgba(0, 0, 0, 0.6)", backdropFilter: "blur(8px)" }}
      onClick={closeSettings}>
      <div className="w-full max-w-xl max-h-[80vh] overflow-y-auto animate-scale-in"
        style={{
          background: "var(--evoury-surface)",
          border: "1px solid var(--evoury-border-light)",
          borderRadius: "16px",
          boxShadow: "0 24px 80px rgba(0,0,0,0.5)",
        }}
        onClick={(e) => e.stopPropagation()}>

        <div className="flex items-center justify-between px-5 py-3.5"
          style={{ borderBottom: "1px solid var(--evoury-border)" }}>
          <h2 className="text-sm font-semibold" style={{ color: "var(--evoury-text)" }}>
            <i className="fas fa-cog mr-2" style={{ color: "var(--evoury-accent)" }} />
            Settings
          </h2>
          <button onClick={closeSettings}
            className="text-[10px] px-2 py-1 rounded-md"
            style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)" }}>
            Esc
          </button>
        </div>

        <div className="p-5 space-y-6">
          <Section title="General">
            <div className="space-y-3">
              <ToggleRow label="Close to tray (sleep mode)" checked={closeToTray}
                onChange={(v) => { setCloseToTray(v); localStorage.setItem("evoury_close_to_tray", String(v)); invoke("set_close_to_tray", { enabled: v }).catch(() => {}); }} />
              <ToggleRow label="Start on system startup" checked={startOnBoot}
                onChange={(v) => { setStartOnBoot(v); localStorage.setItem("evoury_start_on_boot", String(v)); invoke("set_autostart", { enabled: v }).catch(() => {}); }} />
              <ToggleRow label="Go to tray on startup" checked={trayOnStart}
                onChange={(v) => { setTrayOnStart(v); localStorage.setItem("evoury_tray_on_start", String(v)); invoke("set_tray_on_start", { enabled: v }).catch(() => {}); }} />
            </div>
          </Section>

          <Section title="Theme">
            <div className="space-y-3">
              <div className="flex gap-2">
                {(["Dark", "Light", "System"] as const).map((t) => (
                  <button key={t} onClick={() => handleThemeChange(t)}
                    className="flex-1 py-2 rounded-lg text-xs font-medium transition-all"
                    style={{
                      background: theme === t ? "var(--evoury-accent-glow)" : "var(--evoury-elevated)",
                      color: theme === t ? "var(--evoury-accent)" : "var(--evoury-text-dim)",
                      border: theme === t ? "1px solid rgba(99, 102, 241, 0.2)" : "1px solid var(--evoury-border)",
                    }}>
                    <i className={`fas fa-${t === "Dark" ? "moon" : t === "Light" ? "sun" : "desktop"} mr-1.5`} />
                    {t}
                  </button>
                ))}
              </div>
              <div className="flex items-center justify-between">
                <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>Density</span>
                <div className="flex gap-1">
                  {(["Compact", "Normal", "Comfortable"] as const).map((d) => (
                    <button key={d} onClick={() => handleDensityChange(d)}
                      className="px-3 py-1 rounded-md text-[10px] transition-all"
                      style={{
                        background: config.density === d ? "var(--evoury-accent-glow)" : "var(--evoury-elevated)",
                        color: config.density === d ? "var(--evoury-accent)" : "var(--evoury-text-dim)",
                        border: config.density === d ? "1px solid rgba(99, 102, 241, 0.2)" : "1px solid var(--evoury-border)",
                      }}>
                      {d}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </Section>

          <Section title="View">
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>Default Grid Size</span>
                <div className="flex items-center gap-2">
                  <input type="range" min={120} max={400} value={default_view.grid_size}
                    onChange={(e) => setView({ grid_size: Number(e.target.value) })} className="w-24" />
                  <span className="text-xs w-8 text-right" style={{ color: "var(--evoury-text)" }}>{default_view.grid_size}</span>
                </div>
              </div>
              <ToggleRow label="Remember Last Collection" checked={default_view.remember_last_collection}
                onChange={(v) => setView({ remember_last_collection: v })} />
              <ToggleRow label="Show Details in Grid" checked={default_view.show_details}
                onChange={(v) => setView({ show_details: v })} />
            </div>
          </Section>

          <Section title="Performance">
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>Preview Memory (MB)</span>
                <span className="text-xs" style={{ color: "var(--evoury-text)" }}>{performance.max_preview_memory}</span>
              </div>
              <input type="range" min={128} max={2048} step={128} value={performance.max_preview_memory}
                onChange={(e) => setPerformance({ max_preview_memory: Number(e.target.value) })} className="w-full" />
              <ToggleRow label="Background Indexing" checked={performance.enable_background_indexing}
                onChange={(v) => setPerformance({ enable_background_indexing: v })} />
              <ToggleRow label="Reduce Motion" checked={performance.reduce_motion}
                onChange={(v) => setPerformance({ reduce_motion: v })} />
            </div>
          </Section>

          <Section title="Interaction">
            <div className="space-y-3">
              <ToggleRow label="Keyboard Navigation" checked={interaction.keyboard_navigation}
                onChange={(v) => setInteraction({ keyboard_navigation: v })} />
              <ToggleRow label="Double-click to Open" checked={interaction.double_click_to_open}
                onChange={(v) => setInteraction({ double_click_to_open: v })} />
              <ToggleRow label="Show Tooltips" checked={interaction.show_tooltips}
                onChange={(v) => setInteraction({ show_tooltips: v })} />
            </div>
          </Section>

          <div className="pt-2 text-[10px] text-center" style={{ color: "var(--evoury-border-light)" }}>
            Changes save automatically
          </div>
        </div>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div>
      <h3 className="text-[10px] font-semibold uppercase tracking-wider mb-3" style={{ color: "var(--evoury-text-dim)" }}>
        {title}
      </h3>
      {children}
    </div>
  );
}

function ToggleRow({ label, checked, onChange }: { label: string; checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>{label}</span>
      <button onClick={() => onChange(!checked)}
        className="w-9 h-5 rounded-full transition-all relative"
        style={{
          background: checked ? "var(--evoury-accent)" : "var(--evoury-elevated)",
          border: "1px solid var(--evoury-border)",
        }}>
        <div className="w-3.5 h-3.5 rounded-full absolute top-0.5 transition-all"
          style={{ left: checked ? "18px" : "2px", background: "white" }} />
      </button>
    </div>
  );
}
