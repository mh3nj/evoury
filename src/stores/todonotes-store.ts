import { create } from "zustand";

export interface TodoItem {
  id: string;
  text: string;
  checked: boolean;
  subtasks: TodoItem[];
}

export interface NoteTab {
  id: string;
  name: string;
  content: string;
}

interface TodoNotesState {
  todoItems: TodoItem[];
  noteTabs: NoteTab[];
  activeNoteTab: string | null;
  addTodo: (text: string) => void;
  toggleTodo: (id: string) => void;
  removeTodo: (id: string) => void;
  updateTodoText: (id: string, text: string) => void;
  addSubtask: (parentId: string, text: string) => void;
  toggleSubtask: (parentId: string, id: string) => void;
  removeSubtask: (parentId: string, id: string) => void;
  addNoteTab: (name: string) => void;
  renameNoteTab: (id: string, name: string) => void;
  removeNoteTab: (id: string) => void;
  setActiveNoteTab: (id: string | null) => void;
  setNoteContent: (id: string, content: string) => void;
  reorderTodos: (items: TodoItem[]) => void;
  reorderNoteTabs: (tabs: NoteTab[]) => void;
}

function persistTodo(items: TodoItem[]) {
  localStorage.setItem("evoury_todos", JSON.stringify(items));
}
function loadTodo(): TodoItem[] {
  try { const r = localStorage.getItem("evoury_todos"); return r ? JSON.parse(r) : []; } catch { return []; }
}
function persistNotes(tabs: NoteTab[]) {
  localStorage.setItem("evoury_notes", JSON.stringify(tabs));
}
function loadNotes(): NoteTab[] {
  try { const r = localStorage.getItem("evoury_notes"); return r ? JSON.parse(r) : []; } catch { return []; }
}

export const useTodoNotesStore = create<TodoNotesState>((set, get) => ({
  todoItems: loadTodo(),
  noteTabs: loadNotes(),
  activeNoteTab: null,

  addTodo: (text) => {
    const items = [...get().todoItems, { id: crypto.randomUUID(), text, checked: false, subtasks: [] }];
    set({ todoItems: items });
    persistTodo(items);
  },
  toggleTodo: (id) => {
    const items = get().todoItems.map((t) => t.id === id ? { ...t, checked: !t.checked } : t);
    set({ todoItems: items });
    persistTodo(items);
  },
  removeTodo: (id) => {
    const items = get().todoItems.filter((t) => t.id !== id);
    set({ todoItems: items });
    persistTodo(items);
  },
  updateTodoText: (id, text) => {
    const items = get().todoItems.map((t) => t.id === id ? { ...t, text } : t);
    set({ todoItems: items });
    persistTodo(items);
  },
  addSubtask: (parentId, text) => {
    const items = get().todoItems.map((t) =>
      t.id === parentId ? { ...t, subtasks: [...t.subtasks, { id: crypto.randomUUID(), text, checked: false, subtasks: [] }] } : t
    );
    set({ todoItems: items });
    persistTodo(items);
  },
  toggleSubtask: (parentId, id) => {
    const items = get().todoItems.map((t) =>
      t.id === parentId ? { ...t, subtasks: t.subtasks.map((s) => s.id === id ? { ...s, checked: !s.checked } : s) } : t
    );
    set({ todoItems: items });
    persistTodo(items);
  },
  removeSubtask: (parentId, id) => {
    const items = get().todoItems.map((t) =>
      t.id === parentId ? { ...t, subtasks: t.subtasks.filter((s) => s.id !== id) } : t
    );
    set({ todoItems: items });
    persistTodo(items);
  },
  addNoteTab: (name) => {
    const tab = { id: crypto.randomUUID(), name, content: "" };
    const tabs = [...get().noteTabs, tab];
    set({ noteTabs: tabs, activeNoteTab: tab.id });
    persistNotes(tabs);
  },
  renameNoteTab: (id, name) => {
    const tabs = get().noteTabs.map((t) => t.id === id ? { ...t, name } : t);
    set({ noteTabs: tabs });
    persistNotes(tabs);
  },
  removeNoteTab: (id) => {
    const tabs = get().noteTabs.filter((t) => t.id !== id);
    set({ noteTabs: tabs, activeNoteTab: get().activeNoteTab === id ? (tabs.length > 0 ? tabs[0].id : null) : get().activeNoteTab });
    persistNotes(tabs);
  },
  setActiveNoteTab: (id) => set({ activeNoteTab: id }),
  setNoteContent: (id, content) => {
    const tabs = get().noteTabs.map((t) => t.id === id ? { ...t, content } : t);
    set({ noteTabs: tabs });
    persistNotes(tabs);
  },
  reorderTodos: (items) => { set({ todoItems: items }); persistTodo(items); },
  reorderNoteTabs: (tabs) => { set({ noteTabs: tabs }); persistNotes(tabs); },
}));
