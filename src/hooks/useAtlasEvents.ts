import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export function useAtlasEvents(callback: (event: any) => void) {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;
    async function setup() {
      try {
        const stop = await listen("atlas-event", (event) => {
          callback(event.payload);
        });
        if (cancelled) stop();
        else unlisten = stop;
      } catch (e) {
        // Only happens when the UI runs outside the desktop shell
        console.error("Failed to subscribe to atlas events", e);
      }
    }
    setup();
    return () => {
      cancelled = true;
      if (unlisten) unlisten();
    };
  }, [callback]);
}
