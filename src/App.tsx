import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Home from "./pages/Home";
import {
  useThemeStore,
  usePreferencesStore,
  useGalleryStore,
  useWorkspaceStore,
} from "./stores";

function App() {
  const loadTheme = useThemeStore((s) => s.load);
  const loadPrefs = usePreferencesStore((s) => s.load);
  const prefsLoaded = usePreferencesStore((s) => s.loaded);
  const theme = usePreferencesStore((s) => s.theme);
  const gridSize = usePreferencesStore((s) => s.default_view.grid_size);
  const viewMode = usePreferencesStore((s) => s.default_view.view_mode);
  const reduceMotion = usePreferencesStore((s) => s.performance.reduce_motion);
  const rememberCollection = usePreferencesStore(
    (s) => s.default_view.remember_last_collection,
  );
  const previewMemory = usePreferencesStore(
    (s) => s.performance.max_preview_memory,
  );

  useEffect(() => {
    loadTheme();
    loadPrefs();
  }, [loadTheme, loadPrefs]);

  // Keep the gallery layout in sync with the saved view preferences. This waits
  // for the saved values, otherwise the defaults would win over the last session.
  useEffect(() => {
    if (!prefsLoaded || !viewMode) return;
    const gallery = useGalleryStore.getState();
    if (gallery.gridSize !== gridSize || gallery.viewMode !== viewMode) {
      gallery.applyView(gridSize, viewMode);
    }
  }, [prefsLoaded, gridSize, viewMode]);

  // "Remember last collection" decides whether the saved collection survives a restart
  useEffect(() => {
    if (!prefsLoaded || rememberCollection) return;
    const workspace = useWorkspaceStore.getState();
    if (workspace.currentCollection || workspace.currentCategory) {
      workspace.setCollection(null);
      workspace.setCategory(null);
    }
  }, [prefsLoaded, rememberCollection]);

  // Preview memory budget is enforced by the Rust cache
  useEffect(() => {
    if (!prefsLoaded) return;
    invoke("set_preview_memory_budget", { megabytes: previewMemory }).catch(
      (e) => console.error("Failed to set preview memory budget", e),
    );
  }, [prefsLoaded, previewMemory]);

  // Follow the OS theme while "System" is selected
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    function onChange() {
      if (theme !== "System") return;
      const root = document.documentElement;
      root.classList.toggle("theme-light", !mq.matches);
    }
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, [theme]);

  useEffect(() => {
    document.documentElement.classList.toggle("reduced-motion", !!reduceMotion);
  }, [reduceMotion]);

  return <Home />;
}

export default App;
