import { describe, expect, it } from "vitest";
import { uuidFromString } from "./stable-id";

describe("uuidFromString", () => {
  it("is deterministic", () => {
    expect(uuidFromString("D:/Library/Fonts")).toBe(
      uuidFromString("D:/Library/Fonts"),
    );
  });

  it("differs for different inputs", () => {
    const ids = new Set(
      ["a", "b", "a/b", "A", "", "D:/x", "D:/y", "folder 1", "folder 2"].map(
        uuidFromString,
      ),
    );
    expect(ids.size).toBe(9);
  });

  it("looks like a UUID", () => {
    expect(uuidFromString("anything")).toMatch(
      /^[0-9a-f]{8}-[0-9a-f]{4}-5[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/,
    );
  });
});
