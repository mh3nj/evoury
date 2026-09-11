import { useRef } from "react";
import { useGalleryStore, useCommandStore, useSearchStore } from "../../stores";

interface Props {
  onScan?: () => void;
}

export default function GalleryToolbar({ onScan }: Props) {
  const { gridSize, setGridSize, viewMode, setViewMode } = useGalleryStore();
  const openPalette = useCommandStore((s) => s.openPalette);
  const { query, setQuery, setFilters, filters } = useSearchStore();
  const inputRef = useRef<HTMLInputElement>(null);

  function handleChange(q: string) {
    setQuery(q);
    const tokens = q.split(/\s+/);
    const fs: Array<{ field: string; value: string; label: string }> = [];
    for (const token of tokens) {
      if (token.includes(":")) {
        const [field, ...rest] = token.split(":");
        const value = rest.join(":");
        const labels: Record<string, string> = {
          type: "Type", tag: "Tag", tags: "Tag",
          rating: "Rating", rate: "Rating",
          favorite: "Favorite", fav: "Favorite",
          collection: "Collection", col: "Collection",
          category: "Category", cat: "Category",
          sort: "Sort",
        };
        fs.push({ field: field.toLowerCase(), value, label: labels[field.toLowerCase()] || field });
      }
    }
    setFilters(fs);
  }

  function removeFilter(index: number) {
    const tokens = query.split(/\s+/);
    let removed = 0;
    const filtered = tokens.filter((t) => {
      if (t.includes(":")) {
        if (removed === index) { removed++; return false; }
        removed++;
      }
      return true;
    });
    handleChange(filtered.join(" "));
  }

  return (
    <div className="flex flex-col" style={{ background: "var(--evoury-surface)" }}>
      <div className="flex items-center justify-between px-4 py-2.5"
        style={{ borderBottom: "1px solid var(--evoury-border)" }}>
        <div className="flex items-center gap-3 flex-1">
          <button
            onClick={() => openPalette()}
            className="px-3 py-1.5 text-xs rounded-lg transition-all duration-200 flex items-center gap-2"
            style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}
            onMouseEnter={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border-light)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
            onMouseLeave={(e) => { e.currentTarget.style.borderColor = "var(--evoury-border)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}
          >
            <i className="fas fa-terminal text-[10px]" />
            Commands
            <span className="text-[8px] px-1.5 py-0.5 rounded" style={{ background: "var(--evoury-bg)", color: "var(--evoury-text-dim)" }}>Ctrl+K</span>
          </button>
          <div className="w-px h-5" style={{ background: "var(--evoury-border)" }} />
          {(["compact", "grid", "comfortable"] as const).map((mode) => (
            <button
              key={mode}
              onClick={() => setViewMode(mode)}
              className="px-2.5 py-1 text-[10px] rounded-md transition-all duration-200 uppercase tracking-wider font-medium"
              style={{
                background: viewMode === mode ? "var(--evoury-accent-glow)" : "transparent",
                color: viewMode === mode ? "var(--evoury-accent)" : "var(--evoury-text-dim)",
                border: viewMode === mode ? "1px solid rgba(99, 102, 241, 0.2)" : "1px solid transparent",
              }}
              onMouseEnter={(e) => { if (viewMode !== mode) { e.currentTarget.style.color = "var(--evoury-text)"; }}}
              onMouseLeave={(e) => { if (viewMode !== mode) { e.currentTarget.style.color = "var(--evoury-text-dim)"; }}}
            >
              {mode}
            </button>
          ))}
          <div className="flex-1 max-w-md">
            <div className="relative">
              <i className="fas fa-search absolute left-3 top-1/2 -translate-y-1/2 text-[10px]"
                style={{ color: "var(--evoury-text-dim)" }} />
              <input
                ref={inputRef}
                type="text"
                value={query}
                onChange={(e) => handleChange(e.target.value)}
                placeholder='Search assets... (type:zip rating:5)'
                className="w-full text-xs rounded-lg pl-8 pr-3 py-1.5 outline-none transition-all duration-200"
                style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: query ? "1px solid var(--evoury-accent)" : "1px solid var(--evoury-border)" }}
              />
              {query && (
                <button
                  onClick={() => { setQuery(""); setFilters([]); }}
                  className="absolute right-2 top-1/2 -translate-y-1/2 text-[8px]"
                  style={{ color: "var(--evoury-text-dim)" }}
                >
                  <i className="fas fa-times" />
                </button>
              )}
            </div>
          </div>
        </div>
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <i className="fas fa-sliders-h text-[10px]" style={{ color: "var(--evoury-text-dim)" }} />
            <input
              type="range"
              min={120}
              max={400}
              value={gridSize}
              onChange={(e) => setGridSize(Number(e.target.value))}
              className="w-20"
            />
          </div>
          {onScan && (
            <button
              onClick={onScan}
              className="premium-btn px-3 py-1.5 text-xs flex items-center gap-1.5"
            >
              <i className="fas fa-sync-alt text-[10px]" />
              Scan
            </button>
          )}
        </div>
      </div>
      {filters.length > 0 && (
        <div className="flex items-center gap-1.5 px-4 py-1.5 animate-fade-in"
          style={{ borderBottom: "1px solid var(--evoury-border)", background: "var(--evoury-bg)" }}>
          {filters.map((f, i) => (
            <span key={i} className="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px]"
              style={{ background: "var(--evoury-accent-glow)", color: "var(--evoury-accent)", border: "1px solid rgba(99, 102, 241, 0.2)" }}>
              <span className="font-medium">{f.label}:</span>
              <span>{f.value || "..."}</span>
              <button onClick={() => removeFilter(i)} className="ml-0.5 hover:opacity-70">
                <i className="fas fa-times" />
              </button>
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
