import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  buildTree,
  describeContentsError,
  filterEntries,
  formatBytes,
  type ArchiveContents,
  type TreeNode,
} from "../../utils/archive-tree";

interface Props {
  assetId?: string;
  assetName: string;
}

const CATEGORY_LABEL: Record<string, string> = {
  transformable: "layered",
  preservable: "kept as-is",
  container: "archive",
  license: "license",
};

/**
 * What is inside the archive, without extracting anything. Kaelovun's sidecar
 * answers instantly (RAR included); otherwise the backend lists it with 7-Zip.
 */
export default function ArchiveContentsViewer({ assetId, assetName }: Props) {
  const [contents, setContents] = useState<ArchiveContents | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [query, setQuery] = useState("");
  const [open, setOpen] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!assetId) return;
    let cancelled = false;
    setLoading(true);
    setError(null);
    setContents(null);
    setQuery("");
    invoke<ArchiveContents>("get_archive_contents", { assetId })
      .then((c) => {
        if (cancelled) return;
        setContents(c);
        // First level open, the rest on demand
        setOpen(
          new Set(
            c.entries
              .filter((e) => e.is_dir && !e.path.includes("/"))
              .map((e) => e.path),
          ),
        );
      })
      .catch((e) => !cancelled && setError(describeContentsError(String(e))))
      .finally(() => !cancelled && setLoading(false));
    return () => {
      cancelled = true;
    };
  }, [assetId]);

  const searching = query.trim().length > 0;
  const tree = useMemo(
    () => (contents ? buildTree(filterEntries(contents.entries, query)) : null),
    [contents, query],
  );

  const toggle = (path: string) =>
    setOpen((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });

  const rows: { node: TreeNode; depth: number }[] = [];
  const walk = (node: TreeNode, depth: number) => {
    for (const child of node.children) {
      rows.push({ node: child, depth });
      if (child.is_dir && (searching || open.has(child.path)))
        walk(child, depth + 1);
    }
  };
  if (tree) walk(tree, 0);

  return (
    <div className="flex flex-col w-full max-w-2xl h-full max-h-[80vh] gap-3">
      <div className="flex items-center gap-3">
        <i
          className="fas fa-file-archive text-lg"
          style={{ color: "var(--evoury-accent)" }}
        />
        <div className="min-w-0">
          <div
            className="text-sm font-medium truncate"
            style={{ color: "var(--evoury-text)" }}
          >
            {contents?.package_name || assetName}
          </div>
          <div className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>
            {loading && "Reading archive..."}
            {contents &&
              `${contents.total_files.toLocaleString()} files · ${formatBytes(contents.total_size)} · from ${
                contents.source === "sidecar" ? "Kaelovun's manifest" : "7-Zip"
              }`}
          </div>
        </div>
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Filter files..."
          disabled={!contents}
          className="ml-auto text-xs rounded-lg px-3 py-1.5 outline-none"
          style={{
            background: "var(--evoury-elevated)",
            border: "1px solid var(--evoury-border)",
            color: "var(--evoury-text)",
            width: 180,
          }}
        />
      </div>

      {error && (
        <div
          role="alert"
          className="rounded-xl p-4 text-sm"
          style={{
            background: "var(--evoury-elevated)",
            color: "var(--evoury-text-dim)",
          }}
        >
          <i className="fas fa-circle-info mr-2" />
          {error}
        </div>
      )}

      {tree && (
        <div
          className="flex-1 overflow-auto rounded-xl p-2 font-mono text-xs"
          style={{
            background: "var(--evoury-elevated)",
            color: "var(--evoury-text-dim)",
          }}
        >
          {rows.length === 0 && (
            <div className="p-3">
              {searching ? "No file matches." : "The archive is empty."}
            </div>
          )}
          {rows.map(({ node, depth }) => (
            <div
              key={node.path}
              className="flex items-center gap-2 py-0.5 pr-2 rounded"
              style={{
                paddingLeft: depth * 16 + 6,
                cursor: node.is_dir ? "pointer" : "default",
              }}
              onClick={() => node.is_dir && toggle(node.path)}
              title={node.path}
            >
              <i
                className={`fas ${
                  node.is_dir
                    ? searching || open.has(node.path)
                      ? "fa-folder-open"
                      : "fa-folder"
                    : "fa-file"
                } w-4 text-center`}
                style={{
                  color: node.is_dir ? "var(--evoury-accent)" : undefined,
                }}
              />
              <span
                className="truncate"
                style={{ color: "var(--evoury-text)" }}
              >
                {node.name}
              </span>
              {node.category && CATEGORY_LABEL[node.category] && (
                <span
                  className="text-[10px] px-1.5 rounded"
                  style={{
                    background: "var(--evoury-bg)",
                    color: "var(--evoury-accent)",
                  }}
                >
                  {CATEGORY_LABEL[node.category]}
                </span>
              )}
              <span className="ml-auto flex-shrink-0">
                {node.is_dir
                  ? `${node.fileCount.toLocaleString()} files · `
                  : ""}
                {formatBytes(node.size)}
              </span>
            </div>
          ))}
          {contents?.truncated && (
            <div className="p-2" style={{ color: "var(--evoury-accent)" }}>
              Only the first {contents.entries.length.toLocaleString()} entries
              are shown; the totals above cover the whole archive.
            </div>
          )}
        </div>
      )}
    </div>
  );
}
