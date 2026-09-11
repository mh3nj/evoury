import { useEffect } from "react";

export function useKeyboard(key: string, callback: () => void) {
  useEffect(() => {
    function handler(event: KeyboardEvent) {
      if (event.key === key) {
        callback();
      }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [key, callback]);
}
