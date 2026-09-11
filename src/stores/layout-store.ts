import { create } from "zustand";

export interface PanelState {
  id: string;
  visible: boolean;
  width: number;
  collapsed: boolean;
  pinned: boolean;
}

interface LayoutState {
  sidebarOpen: boolean;
  inspectorOpen: boolean;
  focusMode: boolean;
  sidebarWidth: number;
  inspectorWidth: number;
  panels: PanelState[];
  toggleSidebar: () => void;
  toggleInspector: () => void;
  toggleFocusMode: () => void;
  setSidebarWidth: (w: number) => void;
  setInspectorWidth: (w: number) => void;
  setPanelVisibility: (id: string, visible: boolean) => void;
  reset: () => void;
}

export const useLayoutStore = create<LayoutState>((set) => ({
  sidebarOpen: true,
  inspectorOpen: true,
  focusMode: false,
  sidebarWidth: 256,
  inspectorWidth: 288,
  panels: [
    { id: "sidebar", visible: true, width: 256, collapsed: false, pinned: true },
    { id: "inspector", visible: true, width: 288, collapsed: false, pinned: true },
    { id: "gallery", visible: true, width: 0, collapsed: false, pinned: true },
    { id: "basket", visible: false, width: 256, collapsed: true, pinned: false },
    { id: "health", visible: false, width: 200, collapsed: true, pinned: false },
  ],
  toggleSidebar: () => set((s) => ({ sidebarOpen: !s.sidebarOpen })),
  toggleInspector: () => set((s) => ({ inspectorOpen: !s.inspectorOpen })),
  toggleFocusMode: () => set((s) => ({ focusMode: !s.focusMode })),
  setSidebarWidth: (w) => set({ sidebarWidth: w }),
  setInspectorWidth: (w) => set({ inspectorWidth: w }),
  setPanelVisibility: (id, visible) =>
    set((s) => ({
      panels: s.panels.map((p) => (p.id === id ? { ...p, visible } : p)),
    })),
  reset: () =>
    set({
      sidebarOpen: true,
      inspectorOpen: true,
      focusMode: false,
      sidebarWidth: 256,
      inspectorWidth: 288,
    }),
}));
