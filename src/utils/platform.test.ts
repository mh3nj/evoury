import { describe, it, expect, vi, afterEach } from "vitest";
import { detectOs, revealLabel } from "./platform";

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("detectOs", () => {
  it("detects Windows user agents", () => {
    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)" });
    expect(detectOs()).toBe("windows");
  });

  it("detects macOS user agents", () => {
    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)" });
    expect(detectOs()).toBe("macos");
  });

  it("detects Linux user agents", () => {
    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 (X11; Linux x86_64)" });
    expect(detectOs()).toBe("linux");
  });
});

describe("revealLabel", () => {
  it("names the file manager for the current platform", () => {
    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 (Windows NT 10.0)" });
    expect(revealLabel()).toBe("Show in Explorer");

    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)" });
    expect(revealLabel()).toBe("Reveal in Finder");

    vi.stubGlobal("navigator", { userAgent: "Mozilla/5.0 (X11; Linux x86_64)" });
    expect(revealLabel()).toBe("Show in Folder");
  });
});
