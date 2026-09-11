import { useState, useRef, useEffect, useCallback } from "react";
import type { Collection } from "../../types";
import { useWorkspaceStore, useTodoNotesStore } from "../../stores";

interface Props {
  collections?: Collection[];
  libraries?: string[];
}

const MIN_WIDTH = 200;
const MAX_WIDTH = 400;

export default function Sidebar({ collections = [], libraries = [] }: Props) {
  const [width, setWidth] = useState(240);
  const dragging = useRef(false);
  const startX = useRef(0);
  const startW = useRef(0);

  const { currentCollection, setCollection, currentCategory, setCategory } = useWorkspaceStore();
  const {
    todoItems, addTodo, toggleTodo, removeTodo, updateTodoText,
    addSubtask, toggleSubtask, removeSubtask,
  } = useTodoNotesStore();
  const [newTodoText, setNewTodoText] = useState("");
  const [newSubtaskText, setNewSubtaskText] = useState<string | null>(null);
  const [editingTodoId, setEditingTodoId] = useState<string | null>(null);
  const [editTodoText, setEditTodoText] = useState("");

  const onMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    dragging.current = true;
    startX.current = e.clientX;
    startW.current = width;
  }, [width]);

  useEffect(() => {
    function onMove(e: MouseEvent) {
      if (!dragging.current) return;
      const diff = e.clientX - startX.current;
      setWidth(Math.min(MAX_WIDTH, Math.max(MIN_WIDTH, startW.current + diff)));
    }
    function onUp() { dragging.current = false; }
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
    return () => { window.removeEventListener("mousemove", onMove); window.removeEventListener("mouseup", onUp); };
  }, []);

  return (
    <aside className="flex flex-col overflow-hidden animate-fade-in relative select-none"
      style={{ width, minWidth: MIN_WIDTH, maxWidth: MAX_WIDTH, background: "var(--evoury-surface)", borderRight: "1px solid var(--evoury-border)" }}>

      <div className="flex-1 overflow-y-auto p-2 space-y-0.5">
        {/* Libraries */}
        <div style={{ padding: "7px 12px", fontSize: 10, fontWeight: 600, textTransform: "uppercase", letterSpacing: "0.05em", color: "var(--evoury-text-dim)" }}>
          <i className="fas fa-book mr-2" />Libraries
        </div>
        {libraries.length === 0 && (
          <div className="flex flex-col items-center justify-center py-6 px-4">
            <p className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>No libraries yet</p>
          </div>
        )}
        {libraries.map((lib, i) => (
          <button key={i}
            className="w-full text-left px-3 py-1.5 rounded-lg text-xs flex items-center gap-2.5 transition-all"
            style={{ color: "var(--evoury-text-dim)", border: "none", cursor: "pointer" }}
            onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
            onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
            <i className="fas fa-book" style={{ fontSize: 12, width: 16 }} />
            <span className="truncate">{lib}</span>
          </button>
        ))}

        <div style={{ height: 1, background: "var(--evoury-border)", margin: "8px 12px" }} />

        {/* Collections */}
        <div style={{ padding: "7px 12px", fontSize: 10, fontWeight: 600, textTransform: "uppercase", letterSpacing: "0.05em", color: "var(--evoury-text-dim)" }}>
          <i className="fas fa-folder mr-2" />Collections
        </div>
        {collections.length === 0 && (
          <div className="flex flex-col items-center justify-center py-6 px-4">
            <p className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>No collections yet</p>
            <p className="text-[10px] mt-1" style={{ color: "var(--evoury-border-light)" }}>Scan a folder to get started</p>
          </div>
        )}
        {collections.map((col) => (
          <div key={col.id}>
            <button onClick={() => setCollection(col.id)}
              className="w-full text-left px-3 py-1.5 rounded-lg text-xs flex items-center gap-2.5 transition-all"
              style={{
                border: "none", cursor: "pointer",
                background: currentCollection === col.id ? "var(--evoury-accent-glow)" : "transparent",
                color: currentCollection === col.id ? "var(--evoury-text)" : "var(--evoury-text-dim)",
              }}
              onMouseEnter={(e) => { if (currentCollection !== col.id) { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}}
              onMouseLeave={(e) => { if (currentCollection !== col.id) { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}}>
              <span>{col.icon_path ? "\uD83D\uDDBC" : "\uD83D\uDCC1"}</span>
              <span className="truncate text-xs font-medium">{col.name}</span>
            </button>
            {currentCollection === col.id && (col.categories?.length ?? 0) > 0 && (
              <div className="ml-6 mt-0.5 space-y-0.5 border-l-2 pl-2 animate-slide-up"
                style={{ borderColor: "var(--evoury-accent-glow)" }}>
                {col.categories?.map((cat) => (
                  <button key={cat.id} onClick={() => setCategory(cat.id)}
                    className="w-full text-left px-2.5 py-1 rounded-md text-[11px] flex items-center gap-2 transition-all"
                    style={{
                      border: "none", cursor: "pointer",
                      background: currentCategory === cat.id ? "var(--evoury-elevated)" : "transparent",
                      color: currentCategory === cat.id ? "var(--evoury-text)" : "var(--evoury-text-dim)",
                    }}
                    onMouseEnter={(e) => { if (currentCategory !== cat.id) { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}}
                    onMouseLeave={(e) => { if (currentCategory !== cat.id) { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}}>
                    <i className="fas fa-tag text-[8px]" />
                    {cat.name}
                  </button>
                ))}
              </div>
            )}
          </div>
        ))}

        <div style={{ height: 1, background: "var(--evoury-border)", margin: "8px 12px" }} />

        {/* To-Do List */}
        <div style={{ padding: "7px 12px", fontSize: 10, fontWeight: 600, textTransform: "uppercase", letterSpacing: "0.05em", color: "var(--evoury-text-dim)" }}>
          <i className="fas fa-check-circle mr-2" />To-Do
          <span className="ml-2 text-[9px]" style={{ color: "var(--evoury-text-dim)" }}>({todoItems.filter((t) => !t.checked).length})</span>
        </div>

        <div className="px-2 space-y-0.5">
          {todoItems.map((item) => (
            <div key={item.id} className="space-y-0.5">
              <div className="flex items-center gap-1.5 py-1 px-1.5 rounded-md group"
                style={{ background: item.checked ? "rgba(34,197,94,0.05)" : "transparent" }}>
                <input type="checkbox" checked={item.checked} onChange={() => toggleTodo(item.id)}
                  style={{ accentColor: "var(--evoury-accent)", width: 11, height: 11 }} />
                {editingTodoId === item.id ? (
                  <input type="text" value={editTodoText}
                    onChange={(e) => setEditTodoText(e.target.value)}
                    onKeyDown={(e) => { if (e.key === "Enter" && editTodoText.trim()) { updateTodoText(item.id, editTodoText.trim()); setEditingTodoId(null); } if (e.key === "Escape") setEditingTodoId(null); }}
                    onBlur={() => { if (editTodoText.trim()) updateTodoText(item.id, editTodoText.trim()); setEditingTodoId(null); }}
                    autoFocus
                    className="flex-1 text-[10px] bg-transparent outline-none rounded px-1"
                    style={{ color: "var(--evoury-text)", border: "1px solid var(--evoury-accent)" }} />
                ) : (
                  <span className="flex-1 text-[10px] truncate block cursor-text"
                    style={{ color: item.checked ? "var(--evoury-text-dim)" : "var(--evoury-text)", textDecoration: item.checked ? "line-through" : "none" }}
                    onDoubleClick={() => { setEditingTodoId(item.id); setEditTodoText(item.text); }}>
                    {item.text}
                  </span>
                )}
                <button onClick={() => setNewSubtaskText(newSubtaskText === item.id ? null : item.id)}
                  className="text-[8px] px-1 rounded opacity-0 group-hover:opacity-100" style={{ color: "var(--evoury-text-dim)" }}>
                  <i className="fas fa-plus" />
                </button>
                <button onClick={() => removeTodo(item.id)}
                  className="text-[8px] px-1 rounded opacity-0 group-hover:opacity-100" style={{ color: "var(--evoury-error)" }}>
                  <i className="fas fa-times" />
                </button>
              </div>

              {/* Subtasks */}
              {item.subtasks.length > 0 && (
                <div className="ml-5 space-y-0.5">
                  {item.subtasks.map((sub) => (
                    <div key={sub.id} className="flex items-center gap-1.5 py-0.5 group">
                      <input type="checkbox" checked={sub.checked} onChange={() => toggleSubtask(item.id, sub.id)}
                        style={{ accentColor: "var(--evoury-accent)", width: 10, height: 10 }} />
                      <span className="text-[9px] flex-1" style={{ color: sub.checked ? "var(--evoury-text-dim)" : "var(--evoury-text)", textDecoration: sub.checked ? "line-through" : "none" }}>
                        {sub.text}
                      </span>
                      <button onClick={() => removeSubtask(item.id, sub.id)}
                        className="text-[7px] opacity-0 group-hover:opacity-100" style={{ color: "var(--evoury-error)" }}>
                        <i className="fas fa-times" />
                      </button>
                    </div>
                  ))}
                </div>
              )}

              {newSubtaskText === item.id && (
                <div className="ml-5 flex gap-1">
                  <input type="text" placeholder="Subtask..."
                    onKeyDown={(e) => {
                      if (e.key === "Enter" && (e.target as HTMLInputElement).value.trim()) {
                        addSubtask(item.id, (e.target as HTMLInputElement).value.trim());
                        (e.target as HTMLInputElement).value = "";
                        setNewSubtaskText(null);
                      }
                      if (e.key === "Escape") setNewSubtaskText(null);
                    }}
                    className="flex-1 text-[9px] rounded px-1.5 py-0.5 outline-none"
                    style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px solid var(--evoury-border)" }}
                    autoFocus />
                </div>
              )}
            </div>
          ))}

          {/* New todo input */}
          <div className="flex gap-1 pt-1">
            <input type="text" value={newTodoText} onChange={(e) => setNewTodoText(e.target.value)}
              onKeyDown={(e) => { if (e.key === "Enter" && newTodoText.trim()) { addTodo(newTodoText.trim()); setNewTodoText(""); } }}
              placeholder="Add to-do..."
              className="flex-1 text-[10px] rounded-md px-2 py-1 outline-none"
              style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px dashed var(--evoury-border)" }} />
            <button onClick={() => { if (newTodoText.trim()) { addTodo(newTodoText.trim()); setNewTodoText(""); } }}
              className="px-2 py-1 rounded-md text-[10px]" style={{ background: "var(--evoury-accent)", color: "white" }}>
              <i className="fas fa-plus" />
            </button>
          </div>
        </div>
      </div>

      {/* Resize Handle */}
      <div onMouseDown={onMouseDown}
        className="absolute top-0 right-0 w-1.5 h-full cursor-col-resize z-10 transition-all"
        style={{ background: "transparent" }}
        onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-accent-glow)"; }}
        onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; }} />
    </aside>
  );
}
