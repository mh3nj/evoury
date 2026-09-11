import { useVirtualizer } from "@tanstack/react-virtual";
import { useRef } from "react";

export function useVirtualGrid(itemCount: number, estimateSize: number = 260) {
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: itemCount,
    getScrollElement: () => parentRef.current,
    estimateSize: () => estimateSize,
    overscan: 5,
  });

  return { parentRef, virtualizer };
}
