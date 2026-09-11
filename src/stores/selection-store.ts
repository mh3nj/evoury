import { create } from "zustand";

type SelectionMode = "replace" | "toggle" | "add" | "remove" | "range" | "invert";

interface SelectionState {
  selectedIds: Set<string>;
  lastSelected: string | null;
  anchor: string | null;
  mode: SelectionMode;
  select: (id: string, orderedIds?: string[]) => void;
  toggle: (id: string) => void;
  selectRange: (id: string, orderedIds: string[]) => void;
  selectAll: (ids: string[]) => void;
  invert: (allIds: string[]) => void;
  clear: () => void;
  isSelected: (id: string) => boolean;
  count: () => number;
  first: () => string | null;
  setAnchor: (id: string) => void;
}

export const useSelectionStore = create<SelectionState>((set, get) => ({
  selectedIds: new Set<string>(),
  lastSelected: null,
  anchor: null,
  mode: "replace",

  select: (id, orderedIds) => {
    const { anchor, selectedIds } = get();
    // If shift is held and we have an anchor, do range select
    if (orderedIds && anchor) {
      const anchorPos = orderedIds.indexOf(anchor);
      const currentPos = orderedIds.indexOf(id);
      if (anchorPos !== -1 && currentPos !== -1) {
        const newSet = new Set(selectedIds);
        const start = Math.min(anchorPos, currentPos);
        const end = Math.max(anchorPos, currentPos);
        for (let i = start; i <= end; i++) {
          newSet.add(orderedIds[i]);
        }
        set({ selectedIds: newSet, lastSelected: id });
        return;
      }
    }
    set({ selectedIds: new Set([id]), lastSelected: id, anchor: id });
  },

  toggle: (id) => {
    const newSet = new Set(get().selectedIds);
    if (newSet.has(id)) newSet.delete(id);
    else newSet.add(id);
    set({ selectedIds: newSet, lastSelected: id });
  },

  selectRange: (id, orderedIds) => {
    const { lastSelected, selectedIds } = get();
    if (!lastSelected) {
      set({ selectedIds: new Set([id]), lastSelected: id, anchor: id });
      return;
    }
    const newSet = new Set(selectedIds);
    const start = orderedIds.indexOf(lastSelected);
    const end = orderedIds.indexOf(id);
    if (start !== -1 && end !== -1) {
      const from = Math.min(start, end);
      const to = Math.max(start, end);
      for (let i = from; i <= to; i++) {
        newSet.add(orderedIds[i]);
      }
    }
    set({ selectedIds: newSet, lastSelected: id });
  },

  selectAll: (ids) => set({ selectedIds: new Set(ids) }),

  invert: (allIds) => {
    const { selectedIds } = get();
    const newSet = new Set(allIds);
    for (const id of selectedIds) newSet.delete(id);
    set({ selectedIds: newSet });
  },

  clear: () => set({ selectedIds: new Set(), lastSelected: null, anchor: null }),

  isSelected: (id) => get().selectedIds.has(id),

  count: () => get().selectedIds.size,

  first: () => get().selectedIds.values().next().value ?? null,

  setAnchor: (id) => set({ anchor: id }),
}));
