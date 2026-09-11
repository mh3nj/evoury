import type { ReactNode } from "react";
import { useLayoutStore } from "../../stores";

interface Props {
  children: ReactNode;
}

export default function FocusMode({ children }: Props) {
  const { focusMode, toggleFocusMode } = useLayoutStore();

  if (!focusMode) return <>{children}</>;

  return (
    <div
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 40,
        background: "var(--evoury-bg)",
        overflow: "auto",
      }}
    >
      {/* Exit button */}
      <button
        onClick={toggleFocusMode}
        style={{
          position: "fixed",
          top: 16,
          right: 16,
          zIndex: 50,
          width: 36,
          height: 36,
          borderRadius: 10,
          border: "1px solid var(--evoury-border)",
          background: "var(--evoury-surface)",
          color: "var(--evoury-text-dim)",
          cursor: "pointer",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          fontSize: 14,
          transition: "all 0.15s ease",
        }}
        onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
        onMouseLeave={(e) => { e.currentTarget.style.background = "var(--evoury-surface)"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}
        title="Exit Focus Mode (Ctrl+Shift+F)"
      >
        <i className="fas fa-compress" />
      </button>
      {children}
    </div>
  );
}
