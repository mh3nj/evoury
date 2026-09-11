import { useEffect } from "react";
import Home from "./pages/Home";
import { useThemeStore, usePreferencesStore } from "./stores";

function App() {
  const loadTheme = useThemeStore((s) => s.load);
  const loadPrefs = usePreferencesStore((s) => s.load);
  const theme = usePreferencesStore((s) => s.theme);

  useEffect(() => { loadTheme(); loadPrefs(); }, [loadTheme, loadPrefs]);

  // Listen for system theme changes
  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    function onChange() {
      if (theme === "System") {
        const root = document.documentElement;
        root.classList.toggle("theme-light", !mq.matches);
      }
    }
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, [theme]);

  return <Home />;
}

export default App;
