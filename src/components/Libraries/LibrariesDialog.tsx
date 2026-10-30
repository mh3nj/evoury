import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import type { LibraryRoot, ScanSummary } from "../../types";
import { Modal } from "./Modal";

interface Props {
  roots: LibraryRoot[];
  onClose: () => void;
  /** Called after anything changed, so the app can reload assets and collections. */
  onChanged: () => Promise<void>;
}

const button = {
  background: "var(--evoury-elevated)",
  color: "var(--evoury-text-dim)",
  border: "1px solid var(--evoury-border-light)",
} as const;

function when(iso: string | null): string {
  if (!iso) return "never";
  const d = new Date(iso);
  return Number.isNaN(d.getTime()) ? "never" : d.toLocaleString();
}

/**
 * Library folders. Evoury can show several at once; removing one only forgets it:
 * files on disk are never touched.
 */
export default function LibrariesDialog({ roots, onClose, onChanged }: Props) {
  const [busy, setBusy] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const [editing, setEditing] = useState<string | null>(null);
  const [draft, setDraft] = useState("");
  const [confirmRemove, setConfirmRemove] = useState<string | null>(null);

  const run = async (key: string, work: () => Promise<string | void>) => {
    setBusy(key);
    setMessage(null);
    try {
      const note = await work();
      if (note) setMessage(note);
      await onChanged();
    } catch (e) {
      setMessage(String(e).replace(/^Error:\s*/, ""));
    } finally {
      setBusy(null);
    }
  };

  const addFolder = async () => {
    const picked = await open({
      directory: true,
      multiple: false,
      title: "Add a library folder",
    });
    if (!picked || typeof picked !== "string") return;
    await run("add", async () => {
      await invoke("add_library_root", { path: picked });
    });
  };

  const rescan = (root: LibraryRoot) =>
    run(root.id, async () => {
      const s = await invoke<ScanSummary>("scan_roots", {
        mode: "verify",
        rootId: root.id,
      });
      const parts = [`${s.added} new`, `${s.removed} removed`];
      return `"${root.name}": ${parts.join(", ")}.${s.errors.length ? " " + s.errors[0] : ""}`;
    });

  const saveName = (root: LibraryRoot) =>
    run(root.id, async () => {
      await invoke("rename_library_root", { id: root.id, name: draft });
      setEditing(null);
    });

  const remove = (root: LibraryRoot) =>
    run(root.id, async () => {
      const n = await invoke<number>("remove_library_root", { id: root.id });
      setConfirmRemove(null);
      return `Removed "${root.name}" (${n} assets). Your files were not touched.`;
    });

  return (
    <Modal title="Libraries" icon="fa-layer-group" onClose={onClose}>
      <p className="text-xs mb-4" style={{ color: "var(--evoury-text-dim)" }}>
        Evoury shows everything in these folders together. A folder that is
        offline (an unplugged drive, say) keeps its assets until it comes back.
      </p>

      <div className="flex flex-col gap-2">
        {roots.length === 0 && (
          <div
            className="text-xs py-6 text-center"
            style={{ color: "var(--evoury-text-dim)" }}
          >
            No library folder yet. Add the folder Kaelovun writes to.
          </div>
        )}
        {roots.map((root) => (
          <div
            key={root.id}
            className="rounded-xl p-3 flex flex-col gap-2"
            style={{
              background: "var(--evoury-elevated)",
              border: "1px solid var(--evoury-border)",
              opacity: busy === root.id ? 0.6 : 1,
            }}
          >
            <div className="flex items-center gap-2">
              <i
                className={`fas ${root.available ? "fa-hard-drive" : "fa-plug-circle-xmark"}`}
                style={{
                  color: root.available ? "var(--evoury-accent)" : "#d9822b",
                }}
              />
              {editing === root.id ? (
                <input
                  autoFocus
                  value={draft}
                  onChange={(e) => setDraft(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === "Enter") void saveName(root);
                    if (e.key === "Escape") setEditing(null);
                  }}
                  className="text-sm rounded px-2 py-0.5 flex-1 outline-none"
                  style={{
                    background: "var(--evoury-bg)",
                    border: "1px solid var(--evoury-border)",
                    color: "var(--evoury-text)",
                  }}
                />
              ) : (
                <span className="text-sm font-medium flex-1 truncate">
                  {root.name}
                </span>
              )}
              <span
                className="text-xs"
                style={{ color: "var(--evoury-text-dim)" }}
              >
                {root.asset_count.toLocaleString()} assets
              </span>
            </div>
            <div
              className="text-[11px] truncate"
              title={root.path}
              style={{ color: "var(--evoury-text-dim)" }}
            >
              {root.path}
              {!root.available && " · offline: its assets are kept"}
              {" · scanned "}
              {when(root.last_scanned_at)}
            </div>
            <div className="flex gap-2 flex-wrap">
              {editing === root.id ? (
                <>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md"
                    style={button}
                    onClick={() => saveName(root)}
                  >
                    Save name
                  </button>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md"
                    style={button}
                    onClick={() => setEditing(null)}
                  >
                    Cancel
                  </button>
                </>
              ) : confirmRemove === root.id ? (
                <>
                  <span
                    className="text-xs self-center"
                    style={{ color: "var(--evoury-text-dim)" }}
                  >
                    Forget this library? Files stay where they are.
                  </span>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md"
                    style={button}
                    onClick={() => remove(root)}
                  >
                    Yes, remove
                  </button>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md"
                    style={button}
                    onClick={() => setConfirmRemove(null)}
                  >
                    Keep
                  </button>
                </>
              ) : (
                <>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md disabled:opacity-40"
                    style={button}
                    disabled={!root.available || busy !== null}
                    onClick={() => rescan(root)}
                    title="Check every file again, not just folders that changed"
                  >
                    <i className="fas fa-sync-alt mr-1.5 text-[10px]" />
                    {busy === root.id ? "Scanning..." : "Rescan"}
                  </button>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md"
                    style={button}
                    onClick={() => {
                      setDraft(root.name);
                      setEditing(root.id);
                    }}
                  >
                    Rename
                  </button>
                  <button
                    className="px-2.5 py-1 text-xs rounded-md"
                    style={button}
                    onClick={() => setConfirmRemove(root.id)}
                  >
                    Remove
                  </button>
                </>
              )}
            </div>
          </div>
        ))}
      </div>

      <div className="flex items-center gap-3 mt-4">
        <button
          onClick={addFolder}
          disabled={busy !== null}
          className="premium-btn px-4 py-1.5 text-xs disabled:opacity-50"
        >
          <i
            className={`fas ${busy === "add" ? "fa-spinner fa-pulse" : "fa-plus"} mr-1.5 text-[10px]`}
          />
          {busy === "add" ? "Adding and scanning..." : "Add library folder"}
        </button>
        {message && (
          <span
            role="status"
            className="text-xs"
            style={{ color: "var(--evoury-text-dim)" }}
          >
            {message}
          </span>
        )}
      </div>
    </Modal>
  );
}
