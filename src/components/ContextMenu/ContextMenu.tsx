import { useEffect, useRef } from "react";

export interface ContextMenuItem {
  id: string;
  label: string;
  icon?: string;
  shortcut?: string;
  danger?: boolean;
  disabled?: boolean;
  divider?: boolean;
  children?: ContextMenuItem[];
  action: () => void;
}

interface Props {
  x: number;
  y: number;
  items: ContextMenuItem[];
  onClose: () => void;
}

export default function ContextMenu({ x, y, items, onClose }: Props) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        onClose();
      }
    };
    const keyHandler = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("mousedown", handler);
    document.addEventListener("keydown", keyHandler);
    return () => {
      document.removeEventListener("mousedown", handler);
      document.removeEventListener("keydown", keyHandler);
    };
  }, [onClose]);

  const menuStyle: React.CSSProperties = {
    position: "fixed",
    left: x,
    top: y,
    zIndex: 1000,
    minWidth: 200,
    background: "var(--evoury-surface)",
    border: "1px solid var(--evoury-border)",
    borderRadius: 12,
    padding: "4px",
    boxShadow: "0 16px 48px rgba(0,0,0,0.4)",
    backdropFilter: "blur(20px)",
  };

  const renderItem = (item: ContextMenuItem, i: number) => {
    if (item.divider) {
      return (
        <div key={i} style={{ height: 1, background: "var(--evoury-border)", margin: "4px 8px" }} />
      );
    }
    return (
      <button
        key={item.id}
        disabled={item.disabled}
        onClick={() => {
          item.action();
          onClose();
        }}
        style={{
          width: "100%",
          display: "flex",
          alignItems: "center",
          gap: 10,
          padding: "8px 12px",
          borderRadius: 8,
          border: "none",
          background: "transparent",
          color: item.danger ? "#ef4444" : "var(--evoury-text)",
          fontSize: 13,
          cursor: item.disabled ? "not-allowed" : "pointer",
          opacity: item.disabled ? 0.4 : 1,
        }}
        onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; }}
        onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; }}
      >
        {item.icon && <i className={`fas fa-${item.icon}`} style={{ width: 16, textAlign: "center", fontSize: 12, color: "var(--evoury-text-dim)" }} />}
        <span style={{ flex: 1 }}>{item.label}</span>
        {item.shortcut && <span style={{ fontSize: 10, color: "var(--evoury-text-dim)", marginLeft: 16 }}>{item.shortcut}</span>}
      </button>
    );
  };

  return (
    <div ref={ref} style={menuStyle} className="animate-fade-in">
      {items.map((item, i) => renderItem(item, i))}
    </div>
  );
}
