import { describe, expect, it, vi } from "vitest";

vi.mock("@tauri-apps/api/tauri", () => ({
  convertFileSrc: (p: string) => `asset://${p}`,
}));

import { previewSrc, thumbSrc } from "./preview-url";

describe("preview urls", () => {
  it("goes through the asset protocol, never a raw path", () => {
    expect(previewSrc({ path: "C:\\lib\\a.avif" })).toBe(
      "asset://C:\\lib\\a.avif",
    );
  });

  it("prefers the thumbnail for grids and falls back to the full preview", () => {
    expect(thumbSrc({ path: "a.avif", thumb: "a.thumb.avif" })).toBe(
      "asset://a.thumb.avif",
    );
    expect(thumbSrc({ path: "a.avif" })).toBe("asset://a.avif");
    expect(thumbSrc({ path: "a.avif", thumb: null })).toBe("asset://a.avif");
  });

  it("handles assets without a preview", () => {
    expect(previewSrc(null)).toBeNull();
    expect(thumbSrc(undefined)).toBeNull();
  });
});
