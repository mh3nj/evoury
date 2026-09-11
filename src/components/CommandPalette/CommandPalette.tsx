import { useState, useEffect, useRef } from "react";
import { useCommandStore } from "../../stores";

interface Command {
  id: string;
  name: string;
  description: string;
  category: string;
  action?: () => void;
}

export default function CommandPalette() {
  const { open, closePalette, openSettings } = useCommandStore();
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const defaultCommands: Command[] = [
    { id: "open_settings", name: "Open Settings", description: "Configure preferences", category: "System", action: () => { closePalette(); openSettings(); } },
    { id: "open_asset", name: "Open Asset", description: "Open selected asset", category: "Navigation" },
    { id: "refresh_library", name: "Refresh Library", description: "Rescan all folders", category: "Library" },
    { id: "toggle_inspector", name: "Toggle Inspector", description: "Show or hide the inspector panel", category: "View" },
    { id: "toggle_sidebar", name: "Toggle Sidebar", description: "Show or hide the sidebar", category: "View" },
    { id: "run_health_check", name: "Run Health Check", description: "Verify library integrity", category: "System" },
    { id: "export_backup", name: "Export Backup", description: "Create a full backup", category: "System" },
  ];

  const filtered = query
    ? defaultCommands.filter(
        (c) =>
          c.name.toLowerCase().includes(query.toLowerCase()) ||
          c.description.toLowerCase().includes(query.toLowerCase())
      )
    : defaultCommands;

  useEffect(() => {
    if (open) {
      setQuery("");
      setSelectedIndex(0);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [open]);

  useEffect(() => {
    if (!open) return;
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") closePalette();
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelectedIndex((i) => Math.min(i + 1, filtered.length - 1));
      }
      if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelectedIndex((i) => Math.max(i - 1, 0));
      }
      if (e.key === "Enter" && filtered[selectedIndex]) {
        closePalette();
        filtered[selectedIndex].action?.();
      }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [open, filtered, selectedIndex, closePalette, openSettings]);

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-[15vh]"
      style={{ background: "rgba(0, 0, 0, 0.6)", backdropFilter: "blur(8px)" }}
      onClick={closePalette}>
      <div className="w-full max-w-lg animate-scale-in"
        style={{
          background: "var(--evoury-surface)",
          border: "1px solid var(--evoury-border-light)",
          borderRadius: "16px",
          boxShadow: "0 24px 80px rgba(0,0,0,0.5)",
          overflow: "hidden",
        }}
        onClick={(e) => e.stopPropagation()}>
        <div className="flex items-center px-4" style={{ borderBottom: "1px solid var(--evoury-border)" }}>
          <i className="fas fa-search text-xs" style={{ color: "var(--evoury-text-dim)" }} />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setSelectedIndex(0);
            }}
            placeholder="Search commands..."
            className="flex-1 py-3.5 px-3 bg-transparent outline-none text-sm"
            style={{ color: "var(--evoury-text)", caretColor: "var(--evoury-accent)" }}
          />
          <button
            onClick={closePalette}
            className="text-[10px] px-2 py-1 rounded-md transition-all"
            style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)" }}
            onMouseEnter={(e) => { e.currentTarget.style.color = "var(--evoury-text)"; }}
            onMouseLeave={(e) => { e.currentTarget.style.color = "var(--evoury-text-dim)"; }}
          >
            Esc
          </button>
        </div>
        <div className="max-h-72 overflow-y-auto p-2 space-y-0.5">
          {filtered.length === 0 && (
            <p className="text-xs text-center py-6" style={{ color: "var(--evoury-text-dim)" }}>
              No commands found
            </p>
          )}
          {filtered.map((cmd, i) => (
            <button
              key={cmd.id}
              onClick={() => { closePalette(); cmd.action?.(); }}
              className="w-full text-left px-3 py-2.5 rounded-xl text-sm transition-all duration-150"
              style={{
                background: i === selectedIndex ? "var(--evoury-accent-glow)" : "transparent",
                color: i === selectedIndex ? "var(--evoury-text)" : "var(--evoury-text-dim)",
                border: i === selectedIndex ? "1px solid rgba(99, 102, 241, 0.2)" : "1px solid transparent",
              }}
              onMouseEnter={(e) => {
                if (i !== selectedIndex) {
                  e.currentTarget.style.background = "var(--evoury-elevated)";
                  e.currentTarget.style.color = "var(--evoury-text)";
                }
              }}
              onMouseLeave={(e) => {
                if (i !== selectedIndex) {
                  e.currentTarget.style.background = "transparent";
                  e.currentTarget.style.color = "var(--evoury-text-dim)";
                }
              }}
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <i className="fas fa-chevron-right text-[8px]" style={{ color: "var(--evoury-accent)" }} />
                  <span className="font-medium">{cmd.name}</span>
                </div>
                <span className="text-[9px] px-1.5 py-0.5 rounded-md uppercase tracking-wider"
                  style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)" }}>
                  {cmd.category}
                </span>
              </div>
              <p className="text-[11px] mt-0.5 ml-4" style={{ color: "var(--evoury-text-dim)" }}>
                {cmd.description}
              </p>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
