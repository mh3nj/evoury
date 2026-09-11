import { useNavHistoryStore } from "../../stores";

export default function WorkspaceTabs() {
  const { tabs, activeTabId, activateTab, closeTab } = useNavHistoryStore();

  if (tabs.length <= 1) return null;

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 2,
        padding: "4px 8px 0",
        borderBottom: "1px solid var(--evoury-border)",
        background: "var(--evoury-surface)",
        overflowX: "auto",
        flexShrink: 0,
      }}
    >
      {tabs.map((tab) => (
        <div
          key={tab.id}
          onClick={() => activateTab(tab.id)}
          style={{
            display: "flex",
            alignItems: "center",
            gap: 8,
            padding: "6px 12px",
            borderTopLeftRadius: 8,
            borderTopRightRadius: 8,
            border: "1px solid",
            borderColor: activeTabId === tab.id ? "var(--evoury-border)" : "transparent",
            borderBottom: activeTabId === tab.id ? "1px solid var(--evoury-surface)" : "none",
            background: activeTabId === tab.id ? "var(--evoury-surface)" : "transparent",
            color: activeTabId === tab.id ? "var(--evoury-text)" : "var(--evoury-text-dim)",
            fontSize: 12,
            cursor: "pointer",
            whiteSpace: "nowrap",
            marginBottom: -1,
            transition: "all 0.15s ease",
            userSelect: "none",
          }}
          onMouseEnter={(e) => {
            if (activeTabId !== tab.id) e.currentTarget.style.background = "var(--evoury-elevated)";
          }}
          onMouseLeave={(e) => {
            if (activeTabId !== tab.id) e.currentTarget.style.background = "transparent";
          }}
        >
          {tab.pinned && <i className="fas fa-thumbtack" style={{ fontSize: 9, transform: "rotate(45deg)" }} />}
          <span>{tab.title}</span>
          {!tab.pinned && (
            <button
              onClick={(e) => { e.stopPropagation(); closeTab(tab.id); }}
              style={{
                width: 16,
                height: 16,
                borderRadius: 4,
                border: "none",
                background: "transparent",
                color: "var(--evoury-text-dim)",
                cursor: "pointer",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                fontSize: 9,
                opacity: 0,
                transition: "opacity 0.15s ease",
              }}
              onMouseEnter={(e) => { e.currentTarget.style.opacity = "1"; e.currentTarget.style.background = "var(--evoury-elevated)"; }}
              onMouseLeave={(e) => { e.currentTarget.style.opacity = "0"; e.currentTarget.style.background = "transparent"; }}
              title="Close tab"
            >
              <i className="fas fa-times" />
            </button>
          )}
        </div>
      ))}
    </div>
  );
}
