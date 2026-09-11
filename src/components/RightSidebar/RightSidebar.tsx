import { useState, useRef } from "react";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { invoke } from "@tauri-apps/api/tauri";
import { useBasketStore, useTodoNotesStore } from "../../stores";
import type { BasketAsset } from "../../stores/basket-store";

export default function RightSidebar() {
  const {
    baskets, activeBasketId, createBasket, renameBasket, setActiveBasket,
    removeAsset, setTempName, clearBasket, deleteBasket,
  } = useBasketStore();
  const {
    noteTabs, activeNoteTab,
    addNoteTab, renameNoteTab, removeNoteTab, setActiveNoteTab, setNoteContent,
  } = useTodoNotesStore();

  const activeNote = noteTabs.find((t) => t.id === activeNoteTab);

  const [newBasketName, setNewBasketName] = useState("");
  const [editingBasketId, setEditingBasketId] = useState<string | null>(null);
  const [editBasketName, setEditBasketName] = useState("");
  const [editingAssetId, setEditingAssetId] = useState<string | null>(null);
  const [editTempName, setEditTempName] = useState("");
  const [newNoteName, setNewNoteName] = useState("");
  const [editingNoteId, setEditingNoteId] = useState<string | null>(null);
  const [editNoteName, setEditNoteName] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [collapsedSections, setCollapsedSections] = useState<Record<string, boolean>>({});
  const noteView = useState<"preview" | "code">("preview");

  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const basketOpen = !collapsedSections["baskets"];
  const notesOpen = !collapsedSections["notes"];

  function toggleSection(s: string) {
    setCollapsedSections((p) => ({ ...p, [s]: !p[s] }));
  }

  function handleCreateBasket() {
    if (!newBasketName.trim()) return;
    createBasket(newBasketName.trim());
    setNewBasketName("");
  }

  function handleRenameBasket(basketId: string) {
    if (editBasketName.trim()) renameBasket(basketId, editBasketName.trim());
    setEditingBasketId(null);
  }

  function handleOpenExternal(archivePath: string) {
    invoke("open_external_file", { path: archivePath }).catch(() => {});
  }

  function handleOpenPath(archivePath: string) {
    invoke("open_file_location", { path: archivePath }).catch(() => {});
  }

  function wrapSelection(prefix: string, suffix = "") {
    const ta = textareaRef.current;
    if (!ta) return;
    const start = ta.selectionStart;
    const end = ta.selectionEnd;
    const text = ta.value;
    const selected = text.substring(start, end);
    const before = text.substring(0, start);
    const after = text.substring(end);
    const replacement = `${prefix}${selected}${suffix}`;
    const content = before + replacement + after;
    if (activeNote) setNoteContent(activeNote.id, content);
    setTimeout(() => {
      ta.focus();
      ta.selectionStart = start + prefix.length;
      ta.selectionEnd = start + prefix.length + selected.length;
    }, 0);
  }

  function insertHeading(level: number) {
    const prefix = "#".repeat(level) + " ";
    wrapSelection(prefix);
  }

  function insertLine() {
    wrapSelection("", "");
    const ta = textareaRef.current;
    if (!ta) return;
    const start = ta.selectionStart;
    const text = ta.value;
    const content = text.substring(0, start) + "\n---\n" + text.substring(start);
    if (activeNote) setNoteContent(activeNote.id, content);
  }

  // Search baskets
  const searchedAssets = searchQuery.trim()
    ? baskets.flatMap((b) =>
        b.assets.filter((a) =>
          a.tempName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          a.name.toLowerCase().includes(searchQuery.toLowerCase())
        ).map((a) => ({ ...a, basketName: b.name, basketId: b.id }))
      )
    : [];

  function formatNoteContent(content: string): string {
    return content
      .replace(/^###### (.*$)/gm, "<h6>$1</h6>")
      .replace(/^##### (.*$)/gm, "<h5>$1</h5>")
      .replace(/^#### (.*$)/gm, "<h4>$1</h4>")
      .replace(/^### (.*$)/gm, "<h3>$1</h3>")
      .replace(/^## (.*$)/gm, "<h2>$1</h2>")
      .replace(/^# (.*$)/gm, "<h1>$1</h1>")
      .replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>")
      .replace(/\*(.*?)\*/g, "<em>$1</em>")
      .replace(/~~(.*?)~~/g, "<s>$1</s>")
      .replace(/```(\w*)\n([\s\S]*?)```/g, "<pre><code>$2</code></pre>")
      .replace(/`([^`]+)`/g, "<code>$1</code>")
      .replace(/^- (.*$)/gm, "<li>$1</li>")
      .replace(/(<li>.*<\/li>\n?)+/g, "<ul>$&</ul>")
      .replace(/\n/g, "<br/>");
  }

  return (
    <aside className="w-80 overflow-y-auto flex flex-col"
      style={{ background: "var(--evoury-surface)", borderLeft: "1px solid var(--evoury-border)" }}>

      {/* ===== BASKETS SECTION ===== */}
      <div>
        <button onClick={() => toggleSection("baskets")}
          className="w-full flex items-center justify-between px-4 py-2.5 text-xs transition-all"
          style={{ color: "var(--evoury-text-dim)", borderBottom: "1px solid var(--evoury-border)" }}
          onMouseEnter={(e) => { e.currentTarget.style.color = "var(--evoury-text)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
          <div className="flex items-center gap-2">
            <i className={`fas fa-chevron-${basketOpen ? "down" : "right"} text-[8px] transition-transform`} />
            <span className="font-semibold uppercase tracking-wider text-[10px]">Baskets</span>
          </div>
          <span className="text-[10px] px-1.5 py-0.5 rounded-md" style={{ background: "var(--evoury-elevated)" }}>
            {baskets.reduce((sum, b) => sum + b.assets.length, 0)}
          </span>
        </button>

        {basketOpen && (
          <div className="px-3 pb-3 space-y-2 animate-slide-up" style={{ borderBottom: "1px solid var(--evoury-border)" }}>
            {/* Search within baskets */}
            <div className="relative mt-1">
              <i className="fas fa-search absolute left-2.5 top-1/2 -translate-y-1/2 text-[8px]" style={{ color: "var(--evoury-text-dim)" }} />
              <input type="text" value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search baskets..."
                className="w-full text-[10px] rounded-md pl-6 pr-2 py-1 outline-none"
                style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px solid var(--evoury-border)" }} />
            </div>

            {searchQuery.trim() && searchedAssets.length > 0 && (
              <div className="max-h-32 overflow-y-auto space-y-1 p-1 rounded-md" style={{ background: "var(--evoury-elevated)" }}>
                {searchedAssets.map((a, i) => (
                  <div key={`${a.basketId}-${a.assetId}-${i}`}
                    className="flex items-center gap-2 px-2 py-1 rounded text-[10px]"
                    style={{ color: "var(--evoury-text-dim)" }}>
                    <i className="fas fa-file text-[8px]" />
                    <span className="truncate flex-1">{a.tempName}</span>
                    <span className="text-[8px]" style={{ color: "var(--evoury-border-light)" }}>{a.basketName}</span>
                  </div>
                ))}
              </div>
            )}

            {baskets.length === 0 && (
              <div className="flex flex-col items-center py-4">
                <p className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>No baskets yet</p>
              </div>
            )}

            {baskets.map((basket) => (
              <div key={basket.id} className="rounded-lg transition-all"
                style={{ background: basket.id === activeBasketId ? "rgba(99,102,241,0.05)" : "transparent" }}>
                {/* Basket header — click the row to select, click the name text to edit */}
                <div className="flex items-center gap-1 px-2 py-1.5">
                  {editingBasketId === basket.id ? (
                    <input type="text" value={editBasketName}
                      onChange={(e) => setEditBasketName(e.target.value)}
                      onKeyDown={(e) => { if (e.key === "Enter") handleRenameBasket(basket.id); if (e.key === "Escape") setEditingBasketId(null); }}
                      onBlur={() => handleRenameBasket(basket.id)}
                      autoFocus
                      className="flex-1 text-[10px] rounded px-1.5 py-0.5 outline-none"
                      style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px solid var(--evoury-accent)" }}
                      onClick={(e) => e.stopPropagation()} />
                  ) : (
                    <button onClick={() => { setActiveBasket(basket.id); }}
                      className="flex-1 text-left text-[11px] font-medium truncate flex items-center gap-1.5"
                      style={{ color: basket.id === activeBasketId ? "var(--evoury-text)" : "var(--evoury-text-dim)" }}>
                      <i className={`fas fa-${basket.assets.length > 0 ? "basket-shopping" : "shopping-basket"} text-[10px]`} style={{ color: "var(--evoury-accent)" }} />
                      {basket.name}
                      <span className="text-[9px] ml-auto" style={{ color: "var(--evoury-text-dim)" }}>({basket.assets.length})</span>
                    </button>
                  )}
                  <button onClick={() => { setEditingBasketId(basket.id); setEditBasketName(basket.name); }}
                    className="text-[9px] px-1 rounded" style={{ color: "var(--evoury-text-dim)" }} title="Rename basket">
                    <i className="fas fa-pen" />
                  </button>
                  <button onClick={() => deleteBasket(basket.id)}
                    className="text-[9px] px-1 rounded" style={{ color: "var(--evoury-error)" }} title="Delete basket">
                    <i className="fas fa-trash" />
                  </button>
                </div>

                {/* Assets in basket */}
                {activeBasketId === basket.id && basket.assets.length > 0 && (
                  <div className="ml-2 space-y-1 max-h-60 overflow-y-auto">
                    {basket.assets.map((asset) => (
                      <BasketAssetRow key={asset.assetId}
                        asset={asset}
                        baskets={baskets}
                        currentBasketId={basket.id}
                        editingAssetId={editingAssetId}
                        editTempName={editTempName}
                        onEditStart={(id, name) => { setEditingAssetId(id); setEditTempName(name); }}
                        onEditChange={setEditTempName}
                        onEditSave={() => {
                          if (editingAssetId && editTempName.trim()) {
                            setTempName(basket.id, editingAssetId, editTempName.trim());
                          }
                          setEditingAssetId(null);
                        }}
                        onEditCancel={() => setEditingAssetId(null)}
                        onRemove={() => removeAsset(basket.id, asset.assetId)}
                        onOpenExternal={() => handleOpenExternal(asset.archivePath)}
                        onOpenPath={() => handleOpenPath(asset.archivePath)}
                        onMoveTo={(targetBasketId) => {
                          if (targetBasketId !== basket.id) {
                            const bs = useBasketStore.getState();
                            bs.moveAsset(basket.id, targetBasketId, asset.assetId);
                          }
                        }}
                      />
                    ))}
                    {basket.assets.length > 0 && (
                      <button onClick={() => clearBasket(basket.id)}
                        className="w-full text-[9px] py-1 rounded-md mt-1 transition-all"
                        style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}>
                        Clear basket
                      </button>
                    )}
                  </div>
                )}
              </div>
            ))}

            {/* New basket input */}
            <div className="flex gap-1 pt-1">
              <input type="text" value={newBasketName} onChange={(e) => setNewBasketName(e.target.value)}
                onKeyDown={(e) => { if (e.key === "Enter") handleCreateBasket(); }}
                placeholder="New basket name..."
                className="flex-1 text-[10px] rounded-md px-2 py-1 outline-none"
                style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px dashed var(--evoury-border)" }} />
              <button onClick={handleCreateBasket}
                className="px-2 py-1 rounded-md text-[10px]" style={{ background: "var(--evoury-accent)", color: "white" }}>
                <i className="fas fa-plus" />
              </button>
            </div>
          </div>
        )}
      </div>

      {/* ===== NOTES SECTION ===== */}
      <div className="flex-1 flex flex-col">
        <button onClick={() => toggleSection("notes")}
          className="w-full flex items-center justify-between px-4 py-2.5 text-xs transition-all"
          style={{ color: "var(--evoury-text-dim)", borderBottom: "1px solid var(--evoury-border)" }}
          onMouseEnter={(e) => { e.currentTarget.style.color = "var(--evoury-text)"; }}
          onMouseLeave={(e) => { e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
          <div className="flex items-center gap-2">
            <i className={`fas fa-chevron-${notesOpen ? "down" : "right"} text-[8px]`} />
            <span className="font-semibold uppercase tracking-wider text-[10px]">Notes</span>
          </div>
        </button>

        {notesOpen && (
          <div className="flex-1 flex flex-col animate-slide-up">
            {/* Note tabs */}
            <div className="flex items-center gap-1 px-3 py-1.5 flex-wrap" style={{ borderBottom: "1px solid var(--evoury-border)" }}>
              {noteTabs.map((tab) => (
                <div key={tab.id} className="flex items-center gap-1 group">
                  {editingNoteId === tab.id ? (
                    <input type="text" value={editNoteName}
                      onChange={(e) => setEditNoteName(e.target.value)}
                      onKeyDown={(e) => { if (e.key === "Enter" && editNoteName.trim()) { renameNoteTab(tab.id, editNoteName.trim()); setEditingNoteId(null); } if (e.key === "Escape") setEditingNoteId(null); }}
                      onBlur={() => { if (editNoteName.trim()) renameNoteTab(tab.id, editNoteName.trim()); setEditingNoteId(null); }}
                      autoFocus
                      className="text-[9px] rounded px-1 py-0.5 outline-none w-20"
                      style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text)", border: "1px solid var(--evoury-accent)" }} />
                  ) : (
                    <button onClick={() => setActiveNoteTab(tab.id)}
                      className="text-[9px] px-1.5 py-0.5 rounded-md transition-all truncate max-w-[80px]"
                      style={{
                        background: tab.id === activeNoteTab ? "var(--evoury-accent-glow)" : "var(--evoury-elevated)",
                        color: tab.id === activeNoteTab ? "var(--evoury-accent)" : "var(--evoury-text-dim)",
                      }}>
                      {tab.name}
                    </button>
                  )}
                  <button onClick={() => { setEditingNoteId(tab.id); setEditNoteName(tab.name); }}
                    className="text-[7px] opacity-0 group-hover:opacity-100" style={{ color: "var(--evoury-text-dim)" }}>
                    <i className="fas fa-pen" />
                  </button>
                  <button onClick={() => removeNoteTab(tab.id)}
                    className="text-[7px] opacity-0 group-hover:opacity-100" style={{ color: "var(--evoury-error)" }}>
                    <i className="fas fa-times" />
                  </button>
                </div>
              ))}
              <input type="text" value={newNoteName} onChange={(e) => setNewNoteName(e.target.value)}
                onKeyDown={(e) => { if (e.key === "Enter" && newNoteName.trim()) { addNoteTab(newNoteName.trim()); setNewNoteName(""); } }}
                placeholder="+"
                className="text-[9px] rounded px-1.5 py-0.5 outline-none w-8"
                style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px dashed var(--evoury-border)" }} />
            </div>

            {/* Note toolbar */}
            {activeNote && (
              <div className="flex items-center gap-0.5 px-2 py-1 flex-wrap" style={{ borderBottom: "1px solid var(--evoury-border)", background: "var(--evoury-bg)" }}>
                <ToolbarBtn icon="fa-bold" title="Bold (Ctrl+B)" onClick={() => wrapSelection("**", "**")} />
                <ToolbarBtn icon="fa-italic" title="Italic (Ctrl+I)" onClick={() => wrapSelection("*", "*")} />
                <ToolbarBtn icon="fa-strikethrough" title="Strikethrough" onClick={() => wrapSelection("~~", "~~")} />
                <div className="w-px h-4 mx-1" style={{ background: "var(--evoury-border)" }} />
                <ToolbarBtn icon="fa-heading" title="H1" onClick={() => insertHeading(1)} />
                <ToolbarBtn icon="fa-heading" sub="2" title="H2" onClick={() => insertHeading(2)} />
                <ToolbarBtn icon="fa-heading" sub="3" title="H3" onClick={() => insertHeading(3)} />
                <div className="w-px h-4 mx-1" style={{ background: "var(--evoury-border)" }} />
                <ToolbarBtn icon="fa-align-left" title="Left" onClick={() => wrapSelection("<div style=\"text-align:left\">", "</div>")} />
                <ToolbarBtn icon="fa-align-center" title="Center" onClick={() => wrapSelection("<div style=\"text-align:center\">", "</div>")} />
                <ToolbarBtn icon="fa-align-right" title="Right" onClick={() => wrapSelection("<div style=\"text-align:right\">", "</div>")} />
                <ToolbarBtn icon="fa-align-justify" title="Justify" onClick={() => wrapSelection("<div style=\"text-align:justify\">", "</div>")} />
                <div className="w-px h-4 mx-1" style={{ background: "var(--evoury-border)" }} />
                <ToolbarBtn icon="fa-language" title="LTR" onClick={() => wrapSelection("<div dir=\"ltr\">", "</div>")} />
                <ToolbarBtn icon="fa-language" sub="RTL" title="RTL" onClick={() => wrapSelection("<div dir=\"rtl\">", "</div>")} />
                <div className="w-px h-4 mx-1" style={{ background: "var(--evoury-border)" }} />
                <ToolbarBtn icon="fa-minus" title="Horizontal line" onClick={insertLine} />
                <div className="w-px h-4 mx-1" style={{ background: "var(--evoury-border)" }} />
                <button onClick={() => {
                  const [view, setView] = noteView;
                  setView(view === "preview" ? "code" : "preview");
                }}
                  className="px-2 py-0.5 rounded text-[9px] font-medium transition-all"
                  style={{
                    background: noteView[0] === "preview" ? "var(--evoury-accent-glow)" : "var(--evoury-elevated)",
                    color: noteView[0] === "preview" ? "var(--evoury-accent)" : "var(--evoury-text-dim)",
                  }}>
                  {noteView[0] === "preview" ? "Code" : "Preview"}
                </button>
              </div>
            )}

            {/* Note content */}
            {activeNote ? (
              <div className="flex-1 flex flex-col overflow-y-auto">
                {noteView[0] === "code" ? (
                  <textarea ref={textareaRef}
                    value={activeNote.content}
                    onChange={(e) => setNoteContent(activeNote.id, e.target.value)}
                    onKeyDown={(e) => {
                      if ((e.ctrlKey || e.metaKey) && e.key === "b") { e.preventDefault(); wrapSelection("**", "**"); }
                      if ((e.ctrlKey || e.metaKey) && e.key === "i") { e.preventDefault(); wrapSelection("*", "*"); }
                    }}
                    placeholder="Write your notes in Markdown..."
                    className="flex-1 w-full resize-none outline-none text-xs p-3 leading-relaxed"
                    style={{ background: "var(--evoury-bg)", color: "var(--evoury-text)", fontFamily: "var(--evoury-font-family)", minHeight: 120 }} />
                ) : (
                  <div className="flex-1 w-full p-3 text-xs leading-relaxed overflow-y-auto"
                    style={{ background: "var(--evoury-bg)", color: "var(--evoury-text)", fontFamily: "var(--evoury-font-family)", overflowWrap: "break-word" }}
                    dangerouslySetInnerHTML={{ __html: formatNoteContent(activeNote.content) }} />
                )}
              </div>
            ) : (
              <div className="flex-1 flex items-center justify-center p-4">
                <p className="text-[10px]" style={{ color: "var(--evoury-text-dim)" }}>
                  {noteTabs.length === 0 ? "Create a note tab above" : "Select a note tab"}
                </p>
              </div>
            )}
          </div>
        )}
      </div>
    </aside>
  );
}

// ---- Sub-components ----

function BasketAssetRow({
  asset, baskets, currentBasketId, editingAssetId, editTempName,
  onEditStart, onEditChange, onEditSave, onEditCancel, onRemove, onOpenExternal, onOpenPath, onMoveTo,
}: {
  asset: BasketAsset; baskets: { id: string; name: string }[]; currentBasketId: string;
  editingAssetId: string | null; editTempName: string;
  onEditStart: (id: string, name: string) => void;
  onEditChange: (v: string) => void; onEditSave: () => void; onEditCancel: () => void;
  onRemove: () => void; onOpenExternal: () => void; onOpenPath: () => void;
  onMoveTo: (targetBasketId: string) => void;
}) {
  const previewUrl = asset.previewPath ? convertFileSrc(asset.previewPath) : null;
  const isEditing = editingAssetId === asset.assetId;
  const [showMove, setShowMove] = useState(false);

  return (
    <div className="flex items-center gap-1.5 px-2 py-1 rounded-md transition-all group relative"
      style={{ background: "var(--evoury-elevated)" }}
      onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-accent-glow)"; }}
      onMouseLeave={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; }}>
      {/* Thumbnail */}
      <div className="w-8 h-8 rounded-md overflow-hidden flex-shrink-0" style={{ background: "var(--evoury-bg)" }}>
        {previewUrl ? (
          <img src={previewUrl} alt="" className="w-full h-full object-cover"
            onError={(e) => { (e.target as HTMLImageElement).style.display = "none"; }} />
        ) : (
          <div className="w-full h-full flex items-center justify-center">
            <i className="fas fa-file text-[8px]" style={{ color: "var(--evoury-text-dim)" }} />
          </div>
        )}
      </div>

      {/* Name */}
      <div className="flex-1 min-w-0">
        {isEditing ? (
          <input type="text" value={editTempName}
            onChange={(e) => onEditChange(e.target.value)}
            onKeyDown={(e) => { if (e.key === "Enter") onEditSave(); if (e.key === "Escape") onEditCancel(); }}
            onBlur={onEditSave} autoFocus
            className="w-full text-[9px] rounded px-1 py-0.5 outline-none"
            style={{ background: "var(--evoury-bg)", color: "var(--evoury-text)", border: "1px solid var(--evoury-accent)" }} />
        ) : (
          <span className="text-[9px] truncate block" style={{ color: "var(--evoury-text)" }}
            onDoubleClick={() => onEditStart(asset.assetId, asset.tempName)}>
            {asset.tempName}
          </span>
        )}
      </div>

      {/* Action buttons */}
      <div className="flex gap-0.5 opacity-0 group-hover:opacity-100 transition-all">
        <button onClick={(e) => { e.stopPropagation(); onEditStart(asset.assetId, asset.tempName); }}
          className="w-4 h-4 flex items-center justify-center rounded text-[7px]" style={{ color: "var(--evoury-text-dim)" }} title="Rename">
          <i className="fas fa-pen" />
        </button>
        <button onClick={(e) => { e.stopPropagation(); onOpenExternal(); }}
          className="w-4 h-4 flex items-center justify-center rounded text-[7px]" style={{ color: "var(--evoury-text-dim)" }} title="Open">
          <i className="fas fa-external-link-alt" />
        </button>
        <button onClick={(e) => { e.stopPropagation(); onOpenPath(); }}
          className="w-4 h-4 flex items-center justify-center rounded text-[7px]" style={{ color: "var(--evoury-text-dim)" }} title="Show in explorer">
          <i className="fas fa-folder-open" />
        </button>
        {/* Move to basket */}
        <div className="relative">
          <button onClick={(e) => { e.stopPropagation(); setShowMove(!showMove); }}
            className="w-4 h-4 flex items-center justify-center rounded text-[7px]" style={{ color: "var(--evoury-text-dim)" }} title="Move to basket">
            <i className="fas fa-arrow-right" />
          </button>
          {showMove && (
            <div className="absolute right-0 top-full mt-0.5 z-50 animate-slide-up"
              style={{ background: "var(--evoury-surface)", border: "1px solid var(--evoury-border)", borderRadius: 6, minWidth: 130, boxShadow: "0 4px 16px rgba(0,0,0,0.3)" }}>
              <div className="py-0.5">
                {baskets.filter((b) => b.id !== currentBasketId).map((b) => (
                  <button key={b.id} onClick={(e) => { e.stopPropagation(); onMoveTo(b.id); setShowMove(false); }}
                    className="w-full text-left px-2.5 py-1 text-[9px] transition-all flex items-center gap-1.5"
                    style={{ color: "var(--evoury-text-dim)" }}
                    onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-accent-glow)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
                    onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
                    <i className="fas fa-shopping-basket text-[7px]" style={{ color: "var(--evoury-accent)" }} />
                    {b.name}
                  </button>
                ))}
                {baskets.filter((b) => b.id !== currentBasketId).length === 0 && (
                  <div className="px-2.5 py-1 text-[8px]" style={{ color: "var(--evoury-border-light)" }}>No other baskets</div>
                )}
              </div>
            </div>
          )}
        </div>
        <button onClick={(e) => { e.stopPropagation(); onRemove(); }}
          className="w-4 h-4 flex items-center justify-center rounded text-[7px]" style={{ color: "var(--evoury-error)" }} title="Remove from basket">
          <i className="fas fa-times" />
        </button>
      </div>
    </div>
  );
}

function ToolbarBtn({ icon, sub, title, onClick }: { icon: string; sub?: string; title?: string; onClick: () => void }) {
  return (
    <button onClick={onClick} title={title}
      className="w-6 h-6 flex items-center justify-center rounded text-[10px] transition-all"
      style={{ color: "var(--evoury-text-dim)" }}
      onMouseEnter={(e) => { e.currentTarget.style.background = "var(--evoury-elevated)"; e.currentTarget.style.color = "var(--evoury-text)"; }}
      onMouseLeave={(e) => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "var(--evoury-text-dim)"; }}>
      <i className={`fas ${icon}`} />
      {sub && <span className="text-[6px] ml-0.5">{sub}</span>}
    </button>
  );
}
