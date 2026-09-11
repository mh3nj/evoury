import { useEffect, useState } from "react";
import { useNotificationsStore } from "../../stores";

export default function NotificationCenter() {
  const [open, setOpen] = useState(false);
  const { notifications, unreadCount, load, markRead, markAllRead, dismiss, clearAll } = useNotificationsStore();

  useEffect(() => { load(); }, [load]);

  const iconByLevel: Record<string, string> = {
    Info: "fa-info-circle", Success: "fa-check-circle", Warning: "fa-exclamation-triangle", Error: "fa-times-circle",
  };
  const colorByLevel: Record<string, string> = {
    Info: "var(--evoury-accent)", Success: "#22c55e", Warning: "#f59e0b", Error: "#ef4444",
  };

  return (
    <div className="relative">
      <button onClick={() => setOpen(!open)}
        className="relative w-7 h-7 rounded-lg flex items-center justify-center text-xs transition-all"
        style={{ color: "var(--evoury-text-dim)" }}
        onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
        onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
        <i className="fas fa-bell" />
        {unreadCount > 0 && (
          <span className="absolute -top-0.5 -right-0.5 w-3.5 h-3.5 rounded-full flex items-center justify-center text-[8px] font-bold text-white"
            style={{ background: "var(--evoury-accent)" }}>
            {unreadCount > 9 ? "9+" : unreadCount}
          </span>
        )}
      </button>
      {open && (
        <div className="absolute right-0 top-9 w-80 rounded-xl shadow-2xl overflow-hidden z-50 animate-fade-in"
          style={{ background: "var(--evoury-bg)", border: "1px solid var(--evoury-border)" }}>
          <div className="flex items-center justify-between px-3 py-2" style={{ borderBottom: "1px solid var(--evoury-border)" }}>
            <span className="text-xs font-medium" style={{ color: "var(--evoury-text)" }}>Notifications</span>
            <div className="flex items-center gap-2">
              {unreadCount > 0 && <button onClick={markAllRead} className="text-[10px]" style={{ color: "var(--evoury-accent)" }}>Mark all read</button>}
              <button onClick={clearAll} className="text-[10px]" style={{ color: "var(--evoury-text-dim)" }}>Clear</button>
            </div>
          </div>
          <div className="max-h-72 overflow-y-auto">
            {notifications.length === 0 ? (
              <div className="flex flex-col items-center py-6">
                <i className="fas fa-bell-slash text-lg" style={{ color: "var(--evoury-text-dim)" }} />
                <span className="text-xs mt-2" style={{ color: "var(--evoury-text-dim)" }}>No notifications</span>
              </div>
            ) : (
              notifications.map((n) => (
                <div key={n.id} className={`px-3 py-2 border-b text-xs transition-all ${n.read ? "" : ""}`}
                  style={{ borderColor: "var(--evoury-border)", opacity: n.read ? 0.6 : 1 }}
                  onClick={() => markRead(n.id)}>
                  <div className="flex items-start gap-2">
                    <i className={`fas ${iconByLevel[n.level] || "fa-bell"} mt-0.5 text-[10px]`}
                      style={{ color: colorByLevel[n.level] }} />
                    <div className="flex-1 min-w-0">
                      <div className="font-medium truncate" style={{ color: "var(--evoury-text)" }}>{n.title}</div>
                      <div className="truncate mt-0.5" style={{ color: "var(--evoury-text-dim)" }}>{n.message}</div>
                      <div className="flex items-center gap-2 mt-1">
                        <span className="text-[9px]" style={{ color: "var(--evoury-text-dim)" }}>{n.category}</span>
                        <span className="text-[9px]" style={{ color: "var(--evoury-text-dim)" }}>
                          {new Date(n.created_at).toLocaleTimeString()}
                        </span>
                      </div>
                    </div>
                    <button onClick={(e) => { e.stopPropagation(); dismiss(n.id); }}
                      className="w-4 h-4 flex items-center justify-center rounded"
                      style={{ color: "var(--evoury-text-dim)" }}>
                      <i className="fas fa-times text-[8px]" />
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}
