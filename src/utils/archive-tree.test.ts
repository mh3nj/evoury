import { describe, expect, it } from "vitest";
import {
  buildTree,
  filterEntries,
  formatBytes,
  type ContentEntry,
} from "./archive-tree";

const file = (path: string, size: number, category?: string): ContentEntry => ({
  path,
  size,
  is_dir: false,
  category,
});

describe("buildTree", () => {
  it("nests folders, rolls up sizes and counts files", () => {
    const tree = buildTree([
      file("SOURCE/logo.psd", 1000, "transformable"),
      file("SOURCE/deep/er/a.txt", 5),
      file("LICENSE/license.pdf", 20),
      file("readme.txt", 3),
    ]);
    expect(tree.fileCount).toBe(4);
    expect(tree.size).toBe(1028);
    const source = tree.children.find((c) => c.name === "SOURCE")!;
    expect(source.size).toBe(1005);
    expect(source.fileCount).toBe(2);
    expect(
      source.children.find((c) => c.name === "deep")!.children[0].name,
    ).toBe("er");
  });

  it("lists folders before files and sorts naturally", () => {
    const tree = buildTree([
      file("b.txt", 1),
      file("a/file10.txt", 1),
      file("a/file2.txt", 1),
      file("z/x.txt", 1),
    ]);
    expect(tree.children.map((c) => c.name)).toEqual(["a", "z", "b.txt"]);
    expect(tree.children[0].children.map((c) => c.name)).toEqual([
      "file2.txt",
      "file10.txt",
    ]);
  });

  it("keeps empty folders and creates implicit parents", () => {
    const tree = buildTree([
      { path: "empty", size: 0, is_dir: true },
      file("only/child/f.bin", 7),
    ]);
    expect(tree.children.map((c) => c.name).sort()).toEqual(["empty", "only"]);
    expect(tree.children.find((c) => c.name === "empty")!.fileCount).toBe(0);
  });

  it("tolerates backslashes, stray slashes and blank paths", () => {
    const tree = buildTree([
      file("dir\\f.txt", 1),
      file("/lead/x.txt", 1),
      file("", 9),
      file("/", 9),
    ]);
    expect(tree.fileCount).toBe(2);
    expect(tree.children.map((c) => c.name).sort()).toEqual(["dir", "lead"]);
  });

  it("keeps the category of files", () => {
    const tree = buildTree([file("a.psd", 1, "transformable")]);
    expect(tree.children[0].category).toBe("transformable");
  });

  it("handles unicode names", () => {
    const tree = buildTree([file("پرونده/فایل.psd", 1)]);
    expect(tree.children[0].name).toBe("پرونده");
  });
});

describe("filterEntries", () => {
  it("is case-insensitive and passes everything through for an empty query", () => {
    const entries = [file("A/Logo.PSD", 1), file("b/readme.txt", 1)];
    expect(filterEntries(entries, "logo")).toHaveLength(1);
    expect(filterEntries(entries, "  ")).toHaveLength(2);
  });
});

describe("formatBytes", () => {
  it("formats", () => {
    expect(formatBytes(0)).toBe("0 B");
    expect(formatBytes(1536)).toBe("1.5 KB");
    expect(formatBytes(5 * 1024 * 1024)).toBe("5.0 MB");
    expect(formatBytes(-1)).toBe("");
  });
});
