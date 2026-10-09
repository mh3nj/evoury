import { useEffect } from "react";
import Home from "./pages/Home";
import { useThemeStore, usePreferencesStore, useGalleryStore } from "./stores";

function App() {
  const loadTheme = useThemeStore((s) => s.load);
  const loadPrefs = usePreferencesStore((s) => s.load);
  const theme = usePreferencesStore((s) => s.theme);
  const gridSize = usePreferencesStore((s) => s.default_view.grid_size);
  const viewMode = usePreferencesStore((s) => s.default_view.view_mode);
  const reduceMotion = usePreferencesStore((s) => s.performance.reduce_motion);

  useEffect(() => {
    loadTheme();
    loadPrefs();
  }, [loadTheme, loadPrefs]);

  // Keep the gallery layout in sync with the saved view preferences
  useEffect(() => {
    if (!viewMode) return;
    const gallery = useGalleryStore.getState();
    if (gallery.gridSize !== gridSize || gallery.viewMode !== viewMode) {
      gallery.applyView(gridSize, viewMode);
    }
  }, [gridSize, viewMode]);

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
