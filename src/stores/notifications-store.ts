import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { AppNotification } from "../types";

interface NotificationsState {
  notifications: AppNotification[];
  unreadCount: number;
  loading: boolean;
  load: () => Promise<void>;
  markRead: (id: string) => Promise<void>;
  markAllRead: () => Promise<void>;
  dismiss: (id: string) => Promise<void>;
  clearAll: () => Promise<void>;
}

export const useNotificationsStore = create<NotificationsState>((set) => ({
  notifications: [],
  unreadCount: 0,
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const [notifications, unreadCount] = await Promise.all([
        invoke<AppNotification[]>("get_notifications"),
        invoke<number>("get_unread_count"),
      ]);
      set({ notifications, unreadCount, loading: false });
    } catch { set({ loading: false }); }
  },
  markRead: async (id) => {
    await invoke("mark_notification_read", { notificationId: id });
    set((s) => ({
      notifications: s.notifications.map((n) => n.id === id ? { ...n, read: true } : n),
      unreadCount: Math.max(0, s.unreadCount - 1),
    }));
  },
  markAllRead: async () => {
    await invoke("mark_all_notifications_read");
    set((s) => ({
      notifications: s.notifications.map((n) => ({ ...n, read: true })),
      unreadCount: 0,
    }));
  },
  dismiss: async (id) => {
    await invoke("dismiss_notification", { notificationId: id });
    set((s) => ({ notifications: s.notifications.filter((n) => n.id !== id) }));
  },
  clearAll: async () => {
    await invoke("clear_notifications");
    set({ notifications: [], unreadCount: 0 });
  },
}));
