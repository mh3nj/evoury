import { useState, useCallback, useEffect, useMemo, useRef } from "react";
import type {
  Asset,
  Collection,
  Category,
  LibraryRoot,
  ScanSummary,
} from "../types";
import {
  Sidebar,
  GalleryToolbar,
  GalleryGrid,
  AssetViewer,
  CommandPalette,
  StatusBar,
  Presentation,
  Settings,
  ThemeStudio,
  CompareMode,
  NotificationCenter,
  Logo,
  RightSidebar,
} from "../components";
import {
  useCommandStore,
  useScanStore,
  useWorkspaceStore,
  useSearchStore,
  useGalleryStore,
  useUndoStore,
  usePreferencesStore,
} from "../stores";
import { useAtlasEvents } from "../hooks";
import { uuidFromString } from "../utils/stable-id";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { getSavedLibraryPath } from "../stores/persistence";
import { DuplicatesDialog, LibrariesDialog } from "../components/Libraries";

export default function Home() {
  const [assets, setAssets] = useState<Asset[]>([]);
  const [collections, setCollections] = useState<Collection[]>([]);
  const [presentationMode, setPresentationMode] = useState(false);
  const [status, setStatus] = useState("Ready");

  const { openSettings } = useCommandStore();
  const scanState = useScanStore();
  const backgroundIndexing = usePreferencesStore(
    (s) => s.performance.enable_background_indexing,
  );
  useWorkspaceStore();

  const [roots, setRoots] = useState<LibraryRoot[]>([]);
  const [librariesOpen, setLibrariesOpen] = useState(false);
  const [duplicatesOpen, setDuplicatesOpen] = useState(false);

  const onOpenExternal = useCallback(async (asset: Asset) => {
    try {
      await invoke("open_external_file", { path: asset.archive_path });
    } catch (e) {
      console.error("open external failed", e);
    }
  }, []);

  const onOpenPath = useCallback(async (asset: Asset) => {
    try {
      await invoke("open_file_location", { path: asset.archive_path });
    } catch (e) {
      console.error("open path failed", e);
    }
  }, []);

  // Ctrl+= / Ctrl+- for grid zoom
  useEffect(() => {
    function handler(e: KeyboardEvent) {
      if (e.ctrlKey || e.metaKey) {
        if (e.key === "=" || e.key === "+") {
          e.preventDefault();
          const gs = useGalleryStore.getState();
          gs.setGridSize(Math.min(400, gs.gridSize + 20));
        }
        if (e.key === "-") {
          e.preventDefault();
          const gs = useGalleryStore.getState();
          gs.setGridSize(Math.max(80, gs.gridSize - 20));
        }
        if (e.key === "0") {
          e.preventDefault();
          useGalleryStore.getState().setGridSize(200);
        }
      }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);

  useEffect(() => {
    function handler(e: KeyboardEvent) {
      if ((e.ctrlKey || e.metaKey) && e.key === ",") {
        e.preventDefault();
        openSettings();
      }
      if ((e.ctrlKey || e.metaKey) && e.key === "z" && !e.shiftKey) {
        e.preventDefault();
        useUndoStore.getState().undo();
      }
      if ((e.ctrlKey || e.metaKey) && e.key === "z" && e.shiftKey) {
        e.preventDefault();
        useUndoStore.getState().redo();
      }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [openSettings]);

  const extractCollections = useCallback(
    (allAssets: Asset[], rootPath: string) => {
      const path = rootPath.replace(/\\/g, "/");
      const rootName = path.split("/").pop() || "Root";
      const folders = new Map<string, { name: string; assets: Asset[] }>();

      for (const asset of allAssets) {
        const dir = asset.archive_path.replace(/\\/g, "/");
        const parts = dir.split("/");
        const folderKey =
          parts.length > 1 ? parts.slice(0, -1).join("/") : "root";
        if (!folders.has(folderKey)) {
          const folderName =
            folderKey === "root"
              ? rootName
              : folderKey.split("/").pop() || rootName;
          folders.set(folderKey, { name: folderName, assets: [] });
        }
        folders.get(folderKey)!.assets.push(asset);
      }

      const collectionsList: Collection[] = [];
      const categoriesMap = new Map<string, Category[]>();

      for (const [folderKey] of folders) {
        const parts = folderKey.split("/");
        if (parts.length <= 1 && folderKey === "root") {
          categoriesMap.set("root", []);
          continue;
        }
        const collectionName = parts[parts.length - 1] || rootName;
        const parentKey = parts.slice(0, -1).join("/") || "root";
        const cat: Category = {
          id: uuidFromString(`category:${folderKey}`),
          name: collectionName,
          path: folderKey,
        };
        if (!categoriesMap.has(parentKey)) {
          categoriesMap.set(parentKey, []);
        }
        categoriesMap.get(parentKey)!.push(cat);
      }

      const rootCat = categoriesMap.get("root");
      if (rootCat && rootCat.length > 0) {
        collectionsList.push({
          id: uuidFromString(`collection:${path}`),
          name: rootName,
          path: path,
          icon_path: null,
          accent_color: null,
          categories: rootCat,
        });
      }

      for (const [folderKey, cats] of categoriesMap) {
        if (folderKey === "root") continue;
        const name = folderKey.split("/").pop() || rootName;
        collectionsList.push({
          id: uuidFromString(`collection:${folderKey}`),
          name,
          path: folderKey,
          icon_path: null,
          accent_color: null,
          categories: cats,
        });
      }

      setCollections(collectionsList);
    },
    [],
  );

  /** Reload libraries, assets and the folder tree from the backend. */
  const refreshLibrary = useCallback(async () => {
    const [list, all] = await Promise.all([
      invoke<LibraryRoot[]>("list_library_roots"),
      invoke<Asset[]>("get_assets"),
    ]);
    setRoots(list);
    setAssets(all);
    extractCollections(all, list[0]?.path ?? "");
    return { list, all };
  }, [extractCollections]);

  /** Follow every library folder (or stop, when background indexing is off). */
  const syncWatcher = useCallback(() => {
    if (usePreferencesStore.getState().performance.enable_background_indexing) {
      invoke("watch_libraries").catch(() => {});
    } else {
      invoke("stop_watching").catch(() => {});
    }
  }, []);

  /** Scan every library with progress and tell the user how it went. */
  const scanLibraries = useCallback(
    async (mode: "full" | "startup" | "incremental") => {
      const scan = useScanStore.getState();
      try {
        setStatus("Scanning...");
        scan.startScan(crypto.randomUUID());
        const summary = await invoke<ScanSummary>("scan_roots", { mode });
        await refreshLibrary();
        setStatus(
          summary.errors.length > 0 ? `Ready · ${summary.errors[0]}` : "Ready",
        );
        scan.finishScan();
      } catch (err) {
        setStatus("Scan failed");
        scan.finishScan();
        scan.setError(String(err));
        console.error(err);
      }
    },
    [refreshLibrary],
  );

  /** "Scan Library": add a folder (or rescan everything when libraries exist). */
  const handleScan = useCallback(async () => {
    const scan = useScanStore.getState();
    try {
      if (roots.length > 0) {
        await scanLibraries("full");
        return;
      }
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Library Folder",
      });
      if (!selected || typeof selected !== "string") return;

      setStatus("Scanning...");
      scan.startScan(crypto.randomUUID());
      await invoke("add_library_root", { path: selected });
      await refreshLibrary();
      syncWatcher();
      setStatus("Ready");
      scan.finishScan();
    } catch (err) {
      setStatus("Scan failed");
      scan.finishScan();
      scan.setError(String(err));
      console.error(err);
    }
  }, [roots.length, scanLibraries, refreshLibrary, syncWatcher]);

  const rescanTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const rescanning = useRef(false);
  const rescanAgain = useRef(false);
  const rescanRef = useRef<() => void>(() => {});

  const scheduleRescan = useCallback(() => {
    if (rescanTimer.current) clearTimeout(rescanTimer.current);
    rescanTimer.current = setTimeout(() => rescanRef.current(), 1500);
  }, []);

  rescanRef.current = async () => {
    if (rescanning.current) {
      rescanAgain.current = true;
      return;
    }
    rescanning.current = true;
    try {
      // Incremental: only folders that changed are listed again, so this is cheap
      await invoke<ScanSummary>("scan_roots", { mode: "incremental" });
      await refreshLibrary();
    } catch (err) {
      console.error("Rescan after a library change failed", err);
    } finally {
      rescanning.current = false;
      if (rescanAgain.current) {
        rescanAgain.current = false;
        scheduleRescan();
      }
    }
  };

  useEffect(
    () => () => {
      if (rescanTimer.current) clearTimeout(rescanTimer.current);
    },
    [],
  );

  useAtlasEvents(
    useCallback(
      (event: any) => {
        if (!event) return;

        if (event.ScanProgress) {
          const p = event.ScanProgress;
          useScanStore
            .getState()
            .updateProgress(
              p.total_files,
              p.scanned_files,
              p.archives_found,
              p.previews_found,
              p.current_file,
            );
        }

        if (event.WatcherEvent) {
          // The watcher only reports that something changed; the library itself is
          // updated by a scan. Wait for the burst to finish (an archive plus its
          // preview and thumbnail arrive together), then rescan once.
          scheduleRescan();
        }
      },
      [scheduleRescan],
    ),
  );

  // Apply tray & autostart settings saved in localStorage to Rust backend
  useEffect(() => {
    const closeToTray = localStorage.getItem("evoury_close_to_tray") === "true";
    const startOnBoot = localStorage.getItem("evoury_start_on_boot") === "true";
    const trayOnStart = localStorage.getItem("evoury_tray_on_start") === "true";
    invoke("set_close_to_tray", { enabled: closeToTray }).catch(() => {});
    invoke("set_tray_on_start", { enabled: trayOnStart }).catch(() => {});
    if (startOnBoot) {
      invoke("set_autostart", { enabled: true }).catch(() => {});
    }
    if (trayOnStart) {
      setTimeout(() => {
        invoke("hide_main_window").catch(() => {});
      }, 600);
    }
  }, []);

  // Start-up: the library is shown straight from the saved index (no scan), then a quick
  // incremental scan in the background catches anything that changed while Evoury was closed.
  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        setStatus("Restoring...");
        let list = await invoke<LibraryRoot[]>("list_library_roots");
        const legacyPath = getSavedLibraryPath();
        if (list.length === 0 && legacyPath) {
          // First launch after upgrading: the one saved folder becomes the first library.
          await invoke("add_library_root", {
            path: legacyPath,
            scan: false,
          }).catch(() => {});
        }
        const loaded = await refreshLibrary();
        list = loaded.list;
        if (cancelled) return;
        setStatus("Ready");
        if (list.length === 0) return;

        const background =
          usePreferencesStore.getState().performance.enable_background_indexing;
        if (loaded.all.length === 0) {
          // Nothing indexed yet (first launch after upgrading): one full scan is needed.
          await scanLibraries("full");
        } else if (background) {
          await scanLibraries("startup");
        }
        if (!cancelled) syncWatcher();
      } catch {
        if (!cancelled) setStatus("Ready");
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [refreshLibrary, scanLibraries, syncWatcher]);

  // The folder watcher follows the background indexing preference as it changes
  useEffect(() => {
    if (roots.length === 0) return;
    if (backgroundIndexing) {
      invoke("watch_libraries").catch(() => {});
    } else {
      invoke("stop_watching").catch(() => {});
    }
  }, [backgroundIndexing, roots.length]);

  const searchQuery = useSearchStore((s) => s.query);

  // Extract CamelCase tags from a filename into individual lowercase tags
  function extractTags(name: string): string[] {
    const base = name.replace(/\.[^/.]+$/, ""); // remove extension
    const parts = base.split(/(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])/);
    return parts.map((p) => p.toLowerCase()).filter((p) => p.length > 1);
  }

  const filteredAssets = useMemo(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return assets;

    // Parse query into text tokens and filter conditions
    const tokens = q.split(/\s+/);
    const textTokens: string[] = [];
    const conds: { field: string; value: string }[] = [];

    for (const token of tokens) {
      if (token.includes(":")) {
        const [field, ...rest] = token.split(":");
        conds.push({
          field: field.toLowerCase(),
          value: rest.join(":").toLowerCase(),
        });
      } else {
        const t = token.toLowerCase();
        if (t) textTokens.push(t);
      }
    }

    return assets.filter((a) => {
      const name = a.name.toLowerCase();
      const type = a.archive_type.toLowerCase();
      const mime = (a.mime_type || "").toLowerCase();
      const tags = extractTags(a.name);

      // Text tokens: must match at least one field OR a tag
      if (textTokens.length > 0) {
        const matchesAny = textTokens.some(
          (t) =>
            name.includes(t) ||
            type.includes(t) ||
            mime.includes(t) ||
            tags.includes(t),
        );
        if (!matchesAny) return false;
      }

      // Filter conditions: all must pass (AND)
      for (const c of conds) {
        if (c.field === "type" || c.field === "format") {
          if (!type.includes(c.value) && !mime.includes(c.value)) return false;
        }
        if (c.field === "name") {
          if (!name.includes(c.value)) return false;
        }
      }

      return true;
    });
  }, [assets, searchQuery]);

  return (
    <div
      className="h-screen flex flex-col"
      style={{ background: "var(--evoury-bg)" }}
    >
      <header
        className="flex items-center justify-between px-5 py-3"
        style={{ borderBottom: "1px solid var(--evoury-border)" }}
      >
        <div className="flex items-center gap-4">
          <Logo size={24} showText />
          <div
            className="w-px h-6"
            style={{ background: "var(--evoury-border-light)" }}
          />
          <button
            onClick={handleScan}
            disabled={scanState.scanning}
            className="premium-btn px-4 py-1.5 text-xs disabled:opacity-50"
          >
            <i
              className={`fas fa-${scanState.scanning ? "spinner fa-pulse" : "sync-alt"} mr-1.5 text-[10px]`}
            />
            {scanState.scanning
              ? "Scanning..."
              : roots.length > 0
                ? "Rescan"
                : "Scan Library"}
          </button>
          <button
            onClick={() => setLibrariesOpen(true)}
            className="px-3 py-1.5 text-xs rounded-lg transition-all duration-200"
            style={{
              background: "var(--evoury-elevated)",
              color: "var(--evoury-text-dim)",
              border: "1px solid var(--evoury-border-light)",
            }}
            title="Manage library folders"
          >
            <i className="fas fa-layer-group mr-1.5 text-[10px]" />
            Libraries{roots.length > 1 ? ` (${roots.length})` : ""}
          </button>
          <button
            onClick={() => setDuplicatesOpen(true)}
            disabled={assets.length === 0}
            className="px-3 py-1.5 text-xs rounded-lg transition-all duration-200 disabled:opacity-40"
            style={{
              background: "var(--evoury-elevated)",
              color: "var(--evoury-text-dim)",
              border: "1px solid var(--evoury-border-light)",
            }}
            title="Find identical archives and files repeated across packages"
          >
            <i className="fas fa-clone mr-1.5 text-[10px]" />
            Duplicates
          </button>
          <button
            onClick={() => setPresentationMode(true)}
            className="px-3 py-1.5 text-xs rounded-lg transition-all duration-200"
            style={{
              background: "var(--evoury-elevated)",
              color: "var(--evoury-text-dim)",
              border: "1px solid var(--evoury-border-light)",
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderColor = "var(--evoury-accent)";
              e.currentTarget.style.color = "var(--evoury-text)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderColor = "var(--evoury-border-light)";
              e.currentTarget.style.color = "var(--evoury-text-dim)";
            }}
          >
            <i className="fas fa-tv mr-1.5 text-[10px]" />
            Present
          </button>
        </div>
        <div className="flex items-center gap-2">
          <NotificationCenter />
          <button
            onClick={openSettings}
            className="w-7 h-7 rounded-lg flex items-center justify-center text-xs transition-all duration-200"
            style={{ color: "var(--evoury-text-dim)" }}
            onMouseEnter={(e) => {
              e.currentTarget.style.color = "var(--evoury-text)";
              e.currentTarget.style.background = "var(--evoury-elevated)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.color = "var(--evoury-text-dim)";
              e.currentTarget.style.background = "transparent";
            }}
            title="Settings (Ctrl+,)"
          >
            <i className="fas fa-cog" />
          </button>
          <span
            className="px-2 py-1 text-[10px] rounded-md font-mono"
            style={{
              background: "var(--evoury-elevated)",
              color: "var(--evoury-text-dim)",
              border: "1px solid var(--evoury-border)",
            }}
          >
            Ctrl+K
          </span>
        </div>
      </header>

      <div className="flex flex-1 overflow-hidden">
        <Sidebar collections={collections} />

        <div
          className="flex flex-col flex-1 overflow-hidden"
          style={{ borderLeft: "1px solid var(--evoury-border)" }}
        >
          <GalleryToolbar />
          <GalleryGrid
            assets={filteredAssets}
            onOpenExternal={onOpenExternal}
            onOpenPath={onOpenPath}
          />
        </div>

        <RightSidebar />
      </div>

      <StatusBar assetCount={assets.length} status={status} />

      <AssetViewer assets={assets} />
      <CompareMode assets={assets} />
      <CommandPalette />
      <Settings />
      <ThemeStudio />

      {presentationMode && (
        <Presentation
          assets={assets}
          onClose={() => setPresentationMode(false)}
        />
      )}

      {librariesOpen && (
        <LibrariesDialog
          roots={roots}
          onClose={() => setLibrariesOpen(false)}
          onChanged={async () => {
            await refreshLibrary();
            syncWatcher();
          }}
        />
      )}

      {duplicatesOpen && (
        <DuplicatesDialog
          assets={assets}
          onClose={() => setDuplicatesOpen(false)}
        />
      )}
    </div>
  );
}
