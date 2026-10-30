export interface ContentEntry {
  path: string;
  size: number;
  is_dir: boolean;
  category?: string | null;
}

export interface ArchiveContents {
  source: "sidecar" | "7-zip";
  entries: ContentEntry[];
  total_files: number;
  total_size: number;
  truncated: boolean;
  package_name?: string | null;
}

export interface TreeNode {
  name: string;
  /** Full path inside the archive ("" for the root). */
  path: string;
  is_dir: boolean;
  /** Files: its size. Folders: total size of everything below. */
  size: number;
  /** Files below this folder (1 for a file). */
  fileCount: number;
  category?: string | null;
  children: TreeNode[];
}

const collator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: "base",
});

/** Turn the flat listing into a folder tree with rolled-up sizes. */
export function buildTree(entries: ContentEntry[]): TreeNode {
  const root: TreeNode = {
    name: "",
    path: "",
    is_dir: true,
    size: 0,
    fileCount: 0,
    children: [],
  };
  const folders = new Map<string, TreeNode>([["", root]]);

  const folder = (path: string): TreeNode => {
    const existing = folders.get(path);
    if (existing) return existing;
    const cut = path.lastIndexOf("/");
    const parent = folder(cut === -1 ? "" : path.slice(0, cut));
    const node: TreeNode = {
      name: path.slice(cut + 1),
      path,
      is_dir: true,
      size: 0,
      fileCount: 0,
      children: [],
    };
    parent.children.push(node);
    folders.set(path, node);
    return node;
  };

  for (const entry of entries) {
    const path = entry.path.replace(/\\/g, "/").replace(/^\/+|\/+$/g, "");
    if (!path) continue;
    if (entry.is_dir) {
      folder(path);
      continue;
    }
    const cut = path.lastIndexOf("/");
    const parent = folder(cut === -1 ? "" : path.slice(0, cut));
    parent.children.push({
      name: path.slice(cut + 1),
      path,
      is_dir: false,
      size: entry.size,
      fileCount: 1,
      category: entry.category,
      children: [],
    });
  }

  const finish = (node: TreeNode) => {
    node.children.sort(
      (a, b) =>
        Number(b.is_dir) - Number(a.is_dir) || collator.compare(a.name, b.name),
    );
    node.size = 0;
    node.fileCount = 0;
    for (const child of node.children) {
      if (child.is_dir) finish(child);
      node.size += child.size;
      node.fileCount += child.fileCount;
    }
  };
  finish(root);
  return root;
}

/** Entries whose path contains `query` (case-insensitive). */
export function filterEntries(
  entries: ContentEntry[],
  query: string,
): ContentEntry[] {
  const q = query.trim().toLowerCase();
  if (!q) return entries;
  return entries.filter((e) => e.path.toLowerCase().includes(q));
}

export function formatBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes < 0) return "";
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let i = 0;
  while (value >= 1024 && i < units.length - 1) {
    value /= 1024;
    i++;
  }
  return `${value >= 100 ? value.toFixed(0) : value.toFixed(1)} ${units[i]}`;
}

/** A readable explanation of the error strings the backend returns. */
export function describeContentsError(message: string): string {
  return (
    message.replace(/^Error:\s*/, "").trim() || "Could not read the archive."
  );
}
