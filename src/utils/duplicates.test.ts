import { describe, expect, it } from "vitest";
import { analysisFinished } from "./duplicates";

const progress = (
  analyzed: number,
  failed: number,
  needs_tool: number,
  remaining: number,
) => ({
  analyzed,
  failed,
  needs_tool,
  remaining,
});

describe("analysisFinished", () => {
  it("keeps going while archives can still be read", () => {
    expect(analysisFinished(progress(40, 0, 0, 100))).toBe(false);
  });

  it("is done when nothing is left", () => {
    expect(analysisFinished(progress(5, 0, 0, 0))).toBe(true);
  });

  it("does not wait for archives that need a tool that is not installed", () => {
    expect(analysisFinished(progress(10, 0, 30, 30))).toBe(true);
  });

  it("stops when a call achieved nothing", () => {
    expect(analysisFinished(progress(0, 0, 0, 12))).toBe(true);
  });
});
