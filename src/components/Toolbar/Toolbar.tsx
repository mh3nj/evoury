import { useCommandStore } from "../../stores";

export default function Toolbar() {
  const openPalette = useCommandStore((s) => s.openPalette);

  return (
    <header className="flex items-center justify-between px-5 py-3"
      style={{ background: "var(--evoury-surface)", borderBottom: "1px solid var(--evoury-border)" }}>
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-3">
          <div className="w-7 h-7 rounded-lg flex items-center justify-center text-white text-xs font-bold"
            style={{ background: "linear-gradient(135deg, var(--evoury-accent), var(--evoury-accent2))" }}>
            E
          </div>
          <div>
            <h1 className="text-sm font-semibold tracking-tight" style={{ color: "var(--evoury-text)" }}>
              Evoury
            </h1>
            <p className="text-[10px] leading-none mt-0.5" style={{ color: "var(--evoury-text-dim)" }}>
              Enter the Flow
            </p>
          </div>
        </div>
        <div className="w-px h-6" style={{ background: "var(--evoury-border-light)" }} />
        <button
          onClick={() => openPalette()}
          className="text-xs transition-all duration-200 flex items-center gap-2 px-2 py-1 rounded-md"
          style={{ color: "var(--evoury-text-dim)" }}
          onMouseEnter={(e) => { e.currentTarget.style.color = "var(--evoury-text)"; e.currentTarget.style.background = "var(--evoury-elevated)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.color = "var(--evoury-text-dim)"; e.currentTarget.style.background = "transparent"; }}
        >
          <i className="fas fa-terminal text-[10px]" />
          Commands
        </button>
      </div>
      <div className="flex items-center gap-2">
        <button className="premium-btn px-3 py-1.5 text-xs flex items-center gap-1.5">
          <i className="fas fa-sync-alt text-[10px]" />
          Scan
        </button>
        <button className="px-3 py-1.5 text-xs rounded-lg transition-all duration-200 flex items-center gap-1.5"
          style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border-light)" }}
          onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
          <i className="fas fa-cog text-[10px]" />
          Settings
        </button>
      </div>
    </header>
  );
}
