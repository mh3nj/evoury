import { useNavHistoryStore } from "../../stores";

export default function Breadcrumbs() {
  const { breadcrumbs, navigateBreadcrumb } = useNavHistoryStore();

  if (breadcrumbs.length <= 1) return null;

  return (
    <nav
      style={{
        display: "flex",
        alignItems: "center",
        gap: 4,
        padding: "6px 16px",
        borderBottom: "1px solid var(--evoury-border)",
        background: "var(--evoury-surface)",
        overflowX: "auto",
        flexShrink: 0,
      }}
    >
      {breadcrumbs.map((crumb, i) => (
        <div key={i} style={{ display: "flex", alignItems: "center", gap: 4 }}>
          {i > 0 && (
            <i className="fas fa-chevron-right" style={{ fontSize: 8, color: "var(--evoury-text-dim)", opacity: 0.5 }} />
          )}
          <button
            onClick={() => navigateBreadcrumb(i)}
            style={{
              display: "flex",
              alignItems: "center",
              gap: 6,
              padding: "4px 10px",
              borderRadius: 6,
              border: "none",
              background: i === breadcrumbs.length - 1 ? "var(--evoury-elevated)" : "transparent",
              color: i === breadcrumbs.length - 1 ? "var(--evoury-text)" : "var(--evoury-text-dim)",
              fontSize: 12,
              fontWeight: i === breadcrumbs.length - 1 ? 600 : 400,
              cursor: "pointer",
              whiteSpace: "nowrap",
              transition: "all 0.15s ease",
            }}
            onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; }}
            onMouseLeave={(e) => {
              if (i !== breadcrumbs.length - 1) e.currentTarget.style.background = "transparent";
            }}
          >
            {crumb.icon && <i className={`fas fa-${crumb.icon}`} style={{ fontSize: 10 }} />}
            {crumb.label}
          </button>
        </div>
      ))}
    </nav>
  );
}
