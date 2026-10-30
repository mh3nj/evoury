import { describe, it, expect } from "vitest";
import {
  BUILTIN_THEMES,
  blankTheme,
  hexToRgba,
  mapVsCodeTheme,
  mergeThemes,
  parseThemeContent,
  resolveTheme,
  slugify,
  stripAlpha,
} from "./palette";
import type { StudioTheme } from "../types/theme";

const VSCODE_THEME = {
  name: "Sample Night",
  type: "dark",
  colors: {
    "editor.background": "#10102080",
    "sideBar.background": "#181830",
    "editorWidget.background": "#202040",
    "editorGroupHeader.tabsBorder": "#303050",
    "editor.foreground": "#e8e8f8",
    descriptionForeground: "#9090b8",
    focusBorder: "#ff00ff",
    "charts.blue": "#00aaff",
    "charts.green": "#33cc66",
    "charts.yellow": "#ffcc00",
    "charts.red": "#ee4444",
  },
};

function paletteTheme(overrides: Partial<StudioTheme> = {}): StudioTheme {
  return {
    ...blankTheme("My Theme"),
    ...overrides,
  };
}

describe("stripAlpha / hexToRgba / slugify", () => {
  it("drops the alpha byte from 8-digit hex", () => {
    expect(stripAlpha("#11223344")).toBe("#112233");
    expect(stripAlpha("#112233")).toBe("#112233");
    expect(stripAlpha("not-a-color")).toBe("not-a-color");
  });

  it("converts hex to rgba", () => {
    expect(hexToRgba("#6366f1", 0.2)).toBe("rgba(99, 102, 241, 0.2)");
    expect(hexToRgba("#00aaff80", 0.15)).toBe("rgba(0, 170, 255, 0.15)");
    expect(hexToRgba("bad", 0.5)).toBe("bad");
  });

  it("slugifies names", () => {
    expect(slugify("My Cool Theme")).toBe("my-cool-theme");
    expect(slugify("  Anar Dark!! ")).toBe("anar-dark");
    expect(slugify("!!!")).toBe("theme");
  });
});

describe("mapVsCodeTheme", () => {
  it("maps VS Code colors onto palette roles", () => {
    const theme = mapVsCodeTheme(VSCODE_THEME, "sample-night");
    expect(theme).not.toBeNull();
    if (!theme) return;
    expect(theme.id).toBe("sample-night");
    expect(theme.name).toBe("Sample Night");
    expect(theme.mode).toBe("dark");
    expect(theme.colors.bg).toBe("#101020");
    expect(theme.colors.surface).toBe("#181830");
    expect(theme.colors.elevated).toBe("#202040");
    expect(theme.colors.border).toBe("#303050");
    expect(theme.colors.text).toBe("#e8e8f8");
    expect(theme.colors.text_dim).toBe("#9090b8");
    expect(theme.colors.accent).toBe("#ff00ff");
    expect(theme.colors.accent_glow).toBe("rgba(255, 0, 255, 0.2)");
    expect(theme.colors.accent2).toBe("#00aaff");
    expect(theme.colors.accent2_glow).toBe("rgba(0, 170, 255, 0.15)");
    expect(theme.colors.success).toBe("#33cc66");
    expect(theme.colors.warning).toBe("#ffcc00");
    expect(theme.colors.error).toBe("#ee4444");
  });

  it("detects light themes", () => {
    const theme = mapVsCodeTheme(
      { ...VSCODE_THEME, type: "light" },
      "sample-day",
    );
    expect(theme?.mode).toBe("light");
  });

  it("rejects JSON without usable colors", () => {
    expect(mapVsCodeTheme(null)).toBeNull();
    expect(mapVsCodeTheme({})).toBeNull();
    expect(mapVsCodeTheme({ colors: {} })).toBeNull();
    expect(
      mapVsCodeTheme({ colors: { "editor.background": "#101020" } }),
    ).toBeNull();
  });
});

describe("builtin themes", () => {
  it("bundles all 34 mh3nj themes", () => {
    expect(BUILTIN_THEMES).toHaveLength(34);
    for (const theme of BUILTIN_THEMES) {
      expect(theme.id).toBeTruthy();
      expect(theme.name).toBeTruthy();
      expect(theme.colors.bg).toMatch(/^#[0-9a-fA-F]{6}$/);
      expect(theme.colors.text).toMatch(/^#[0-9a-fA-F]{6}$/);
    }
  });

  it("has unique ids and includes both modes", () => {
    const ids = new Set(BUILTIN_THEMES.map((t) => t.id));
    expect(ids.size).toBe(BUILTIN_THEMES.length);
    expect(BUILTIN_THEMES.some((t) => t.mode === "dark")).toBe(true);
    expect(BUILTIN_THEMES.some((t) => t.mode === "light")).toBe(true);
  });
});

describe("parseThemeContent", () => {
  it("accepts VS Code theme JSON", () => {
    const theme = parseThemeContent(JSON.stringify(VSCODE_THEME));
    expect(theme).not.toBeNull();
    expect(theme?.id).toBe("sample-night");
    expect(theme?.colors.bg).toBe("#101020");
  });

  it("accepts Evoury palette JSON and keeps its id and mode", () => {
    const original = paletteTheme({ id: "custom-one", mode: "light" });
    const theme = parseThemeContent(JSON.stringify(original));
    expect(theme).toEqual(original);
  });

  it("fills missing palette keys with fallbacks", () => {
    const theme = parseThemeContent(
      JSON.stringify({
        id: "half",
        name: "Half",
        mode: "dark",
        colors: { bg: "#000000", text: "#ffffff" },
      }),
    );
    expect(theme).not.toBeNull();
    expect(theme?.colors.bg).toBe("#000000");
    expect(theme?.colors.text).toBe("#ffffff");
    expect(theme?.colors.accent).toBeTruthy();
  });

  it("rejects invalid content", () => {
    expect(parseThemeContent("{not json")).toBeNull();
    expect(parseThemeContent("[]")).toBeNull();
    expect(parseThemeContent('{"foo": 1}')).toBeNull();
  });
});

describe("mergeThemes / resolveTheme", () => {
  it("lets user themes override builtins by id", () => {
    const original = BUILTIN_THEMES[0];
    const override: StudioTheme = {
      ...original,
      name: "Overridden",
      colors: { ...original.colors, bg: "#000000" },
    };
    const merged = mergeThemes([override]);
    expect(merged.get(original.id)?.name).toBe("Overridden");
    expect(resolveTheme(original.id, [override])?.colors.bg).toBe("#000000");
    expect(resolveTheme(null, [override])).toBeUndefined();
    expect(resolveTheme("missing", [])).toBeUndefined();
  });
});
