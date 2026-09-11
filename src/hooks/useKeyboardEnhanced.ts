import { useEffect } from "react";
import { useKeyboardStore, useLayoutStore, useSelectionStore, useNavHistoryStore, useCommandStore } from "../stores";

export function useKeyboardEnhanced() {
  const findMatch = useKeyboardStore((s) => s.findMatch);
  const enabled = useKeyboardStore((s) => s.enabled);
  const toggleSidebar = useLayoutStore((s) => s.toggleSidebar);
  const toggleInspector = useLayoutStore((s) => s.toggleInspector);
  const toggleFocusMode = useLayoutStore((s) => s.toggleFocusMode);
  const clear = useSelectionStore((s) => s.clear);
  const goBack = useNavHistoryStore((s) => s.goBack);
  const goForward = useNavHistoryStore((s) => s.goForward);
  const openPalette = useCommandStore((s) => s.openPalette);
  const openSettings = useCommandStore((s) => s.openSettings);

  useEffect(() => {
    if (!enabled) return;

    const handler = (e: KeyboardEvent) => {
      // Don't handle if inside an input
      const tag = (e.target as HTMLElement)?.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;

      const ctrl = e.ctrlKey || e.metaKey;
      const shift = e.shiftKey;
      const alt = e.altKey;

      const match = findMatch(e.key, ctrl, shift, alt, e.metaKey, "global");
      if (!match) return;

      e.preventDefault();

      switch (match.command) {
        case "command-palette.open":
          openPalette();
          break;
        case "navigation.back":
          goBack();
          break;
        case "navigation.forward":
          goForward();
          break;
        case "layout.toggle-sidebar":
          toggleSidebar();
          break;
        case "layout.toggle-inspector":
          toggleInspector();
          break;
        case "layout.focus-mode":
          toggleFocusMode();
          break;
        case "selection.clear":
          clear();
          break;
        case "selection.select-all":
          // selectAll would need access to current visible IDs
          break;
        case "selection.invert":
          // invert would need access to current visible IDs
          break;
        case "settings.open":
          openSettings();
          break;
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [enabled, findMatch, toggleSidebar, toggleInspector, toggleFocusMode, clear, goBack, goForward, openPalette, openSettings]);
}
