import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export function useAtlasEvents(callback: (event: any) => void) {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    async function setup() {
      unlisten = await listen("atlas-event", (event) => {
        callback(event.payload);
      });
    }
    setup();
    return () => {
      if (unlisten) unlisten();
    };
  }, [callback]);
}
