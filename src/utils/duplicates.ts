import type { AnalyzeProgress } from "../types";

/**
 * Whether reading archive contents is finished.
 *
 * `remaining` includes archives that need 7-Zip when it isn't installed; those stay
 * pending forever, so they must not keep the loop running. It also stops when a call made
 * no progress at all (nothing left that can be done).
 */
export function analysisFinished(p: AnalyzeProgress): boolean {
  return p.remaining - p.needs_tool <= 0 || p.analyzed + p.failed === 0;
}
