import { useEffect, type ReactNode } from "react";

interface ModalProps {
  title: string;
  icon: string;
  onClose: () => void;
  children: ReactNode;
  width?: number;
}

/** A plain overlay in the app's colours, closed with Escape or by clicking outside. */
export function Modal({
  title,
  icon,
  onClose,
  children,
  width = 640,
}: ModalProps) {
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-6"
      style={{ background: "rgba(0,0,0,0.55)" }}
      onMouseDown={(e) => e.target === e.currentTarget && onClose()}
    >
      <div
        role="dialog"
        aria-label={title}
        className="flex flex-col rounded-2xl overflow-hidden"
        style={{
          width,
          maxWidth: "100%",
          maxHeight: "85vh",
          background: "var(--evoury-surface, var(--evoury-bg))",
          border: "1px solid var(--evoury-border)",
          color: "var(--evoury-text)",
        }}
      >
        <div
          className="flex items-center gap-3 px-5 py-3"
          style={{ borderBottom: "1px solid var(--evoury-border)" }}
        >
          <i
            className={`fas ${icon}`}
            style={{ color: "var(--evoury-accent)" }}
          />
          <h2 className="text-sm font-medium flex-1">{title}</h2>
          <button
            onClick={onClose}
            aria-label="Close"
            className="w-7 h-7 rounded-md text-xs"
            style={{ color: "var(--evoury-text-dim)" }}
          >
            <i className="fas fa-times" />
          </button>
        </div>
        <div className="overflow-auto p-5 flex-1">{children}</div>
      </div>
    </div>
  );
}
