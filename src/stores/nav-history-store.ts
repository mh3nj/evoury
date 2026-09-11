import { create } from "zustand";

export interface NavLocation {
  title: string;
  path: string[];
  collectionId?: string;
  query?: string;
  assetId?: string;
}

export interface Breadcrumb {
  label: string;
  path: string;
  icon?: string;
}

export interface Tab {
  id: string;
  title: string;
  pinned: boolean;
  modified: boolean;
}

interface NavHistoryState {
  history: NavLocation[];
  index: number;
  breadcrumbs: Breadcrumb[];
  tabs: Tab[];
  activeTabId: string | null;
  pushHistory: (loc: NavLocation) => void;
  goBack: () => NavLocation | null;
  goForward: () => NavLocation | null;
  canGoBack: () => boolean;
  canGoForward: () => boolean;
  pushBreadcrumb: (crumb: Breadcrumb) => void;
  popBreadcrumb: () => void;
  navigateBreadcrumb: (index: number) => void;
  openTab: (title: string, loc: NavLocation) => string;
  closeTab: (id: string) => void;
  activateTab: (id: string) => void;
}

export const useNavHistoryStore = create<NavHistoryState>((set, get) => ({
  history: [{ title: "All Assets", path: ["All Assets"] }],
  index: 0,
  breadcrumbs: [{ label: "All Assets", path: "/", icon: "fa-home" }],
  tabs: [{ id: crypto.randomUUID(), title: "All Assets", pinned: true, modified: false }],
  activeTabId: null,

  pushHistory: (loc) => {
    const { history, index } = get();
    const newHistory = [...history.slice(0, index + 1), loc];
    set({ history: newHistory, index: newHistory.length - 1 });
  },

  goBack: () => {
    const { history, index } = get();
    if (index > 0) {
      const newIndex = index - 1;
      set({ index: newIndex });
      return history[newIndex];
    }
    return null;
  },

  goForward: () => {
    const { history, index } = get();
    if (index < history.length - 1) {
      const newIndex = index + 1;
      set({ index: newIndex });
      return history[newIndex];
    }
    return null;
  },

  canGoBack: () => get().index > 0,
  canGoForward: () => get().index < get().history.length - 1,

  pushBreadcrumb: (crumb) =>
    set((s) => ({ breadcrumbs: [...s.breadcrumbs, crumb] })),

  popBreadcrumb: () =>
    set((s) => ({
      breadcrumbs: s.breadcrumbs.length > 1 ? s.breadcrumbs.slice(0, -1) : s.breadcrumbs,
    })),

  navigateBreadcrumb: (index) =>
    set((s) => ({ breadcrumbs: s.breadcrumbs.slice(0, index + 1) })),

  openTab: (title, _loc) => {
    const { tabs } = get();
    const existing = tabs.find((t) => t.title === title);
    if (existing) {
      set({ activeTabId: existing.id });
      return existing.id;
    }
    const id = crypto.randomUUID();
    set((s) => ({
      tabs: [...s.tabs, { id, title, pinned: false, modified: false }],
      activeTabId: id,
    }));
    return id;
  },

  closeTab: (id) => {
    const { tabs, activeTabId } = get();
    const tab = tabs.find((t) => t.id === id);
    if (!tab || tab.pinned) return;
    const newTabs = tabs.filter((t) => t.id !== id);
    let newActive = activeTabId;
    if (activeTabId === id) {
      newActive = newTabs.length > 0 ? newTabs[newTabs.length - 1].id : null;
    }
    set({ tabs: newTabs, activeTabId: newActive });
  },

  activateTab: (id) => set({ activeTabId: id }),
}));
