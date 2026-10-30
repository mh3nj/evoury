import { convertFileSrc } from "@tauri-apps/api/tauri";

export interface PreviewRef {
  path: string;
  /** Small grid thumbnail (Kaelovun's `Name.thumb.avif`), when one exists. */
  thumb?: string | null;
}

/**
 * URL the webview can actually load for a file on disk.
 *
 * A raw path such as `C:\\Library\\Pack.avif` in an `<img src>` is blocked by the
 * webview; files must go through the asset protocol. Every place that shows a
 * preview should use these two helpers instead of touching `preview.path`.
 */
export function previewSrc(preview?: PreviewRef | null): string | null {
  return preview?.path ? convertFileSrc(preview.path) : null;
}

/** The small thumbnail when there is one (fast grids), otherwise the full preview. */
export function thumbSrc(preview?: PreviewRef | null): string | null {
  if (!preview) return null;
  const path = preview.thumb || preview.path;
  return path ? convertFileSrc(path) : null;
}
