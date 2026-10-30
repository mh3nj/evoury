import { describe, it, expect, vi, beforeEach } from "vitest";

const { invokeMock, cssVars } = vi.hoisted(() => {
  const cssVars = new Map<string, string>();
  const root = {
    style: {
      setProperty: vi.fn((key: string, value: string) => {
        cssVars.set(key, value);
      }),
      removeProperty: vi.fn((key: string) => {
        cssVars.delete(key);
      }),
      getPropertyValue: (key: string) => cssVars.get(key) ?? "",
      fontFamily: "",
      fontSize: "",
    },
    classList: { toggle: vi.fn(), add: vi.fn(), remove: vi.fn() },
  };
  (globalThis as any).document = { documentElement: root };
  (globalThis as any).window = {
    matchMedia: () => ({ matches: false }),
  };
  return { invokeMock: vi.fn(), cssVars };
});

vi.mock("@tauri-apps/api/tauri", () => ({ invoke: invokeMock }));

import { useThemeStudioStore } from "./theme-studio-store";
import { useThemeStore } from "./theme-store";
import { usePreferencesStore } from "./preferences-store";
import { blankTheme } from "../themes/palette";
import type { StudioTheme } from "../types/theme";

let storedThemes: StudioTheme[] = [];
let importContent: string | null = null;

beforeEach(() => {
  storedThemes = [];
  importContent = null;
  cssVars.clear();
  invokeMock.mockReset();
  invokeMock.mockImplementation(
    async (cmd: string, args?: Record<string, any>) => {
      if (cmd === "get_themes") return [...storedThemes];
      if (cmd === "save_theme") {
        const theme = args?.theme as StudioTheme;
        storedThemes = [
          ...storedThemes.filter((t) => t.id !== theme.id),
          theme,
        ];
        return null;
      }
      if (cmd === "delete_theme") {
        storedThemes = storedThemes.filter((t) => t.id !== args?.id);
        return null;
      }
      if (cmd === "import_theme") return importContent;
      return null;
    },
  );

  usePreferencesStore.setState({
    loaded: true,
    theme: "Dark",
    color_theme: null,
    density: "Normal",
    default_workspace: null,
  });
  useThemeStore.setState({
    config: {
      ...useThemeStore.getState().config,
      mode: "Dark",
      color_theme: null,
    },
  });
  useThemeStudioStore.setState({ loaded: false, userThemes: [], error: null });
});

function theme(
  name: string,
  overrides: Partial<StudioTheme> = {},
): StudioTheme {
  return { ...blankTheme(name), ...overrides };
}

function savedPrefs(): Record<string, any>[] {
  return invokeMock.mock.calls
    .filter((c) => c[0] === "set_preferences")
    .map((c) => c[1].prefs as Record<string, any>);
}

describe("theme studio store", () => {
  it("loads user themes from the backend", async () => {
    storedThemes = [theme("Custom One", { id: "custom-one" })];
    await useThemeStudioStore.getState().load();
    const state = useThemeStudioStore.getState();
    expect(state.loaded).toBe(true);
    expect(state.userThemes.map((t) => t.id)).toEqual(["custom-one"]);
    expect(state.error).toBeNull();
  });

  it("saves a theme with a slugified id and reloads", async () => {
    const draft = theme("My Cool Theme", { id: "" });
    const saved = await useThemeStudioStore.getState().save(draft);
    expect(saved.id).toBe("my-cool-theme");
    const saveCall = invokeMock.mock.calls.find((c) => c[0] === "save_theme");
    expect(saveCall?.[1].theme.id).toBe("my-cool-theme");
    expect(useThemeStudioStore.getState().userThemes).toHaveLength(1);
  });

  it("save and apply persists the active color theme", async () => {
    await useThemeStudioStore
      .getState()
      .saveAndApply(theme("Sunset Glow", { id: "sunset-glow", mode: "light" }));

    const store = useThemeStore.getState();
    expect(store.config.color_theme).toBe("sunset-glow");
    expect(store.config.mode).toBe("Light");
    const prefs = savedPrefs();
    const last = prefs[prefs.length - 1];
    expect(last?.color_theme).toBe("sunset-glow");
    expect(last?.theme).toBe("Light");
  });

  it("apply paints the palette onto the document", async () => {
    const t = theme("Painted", {
      id: "painted",
      colors: { ...blankTheme("x").colors, bg: "#123456", text: "#abcdef" },
    });
    await useThemeStudioStore.getState().apply(t);
    expect(cssVars.get("--evoury-bg")).toBe("#123456");
    expect(cssVars.get("--evoury-text")).toBe("#abcdef");
  });

  it("removing the active theme clears it from preferences", async () => {
    storedThemes = [theme("Doomed", { id: "doomed" })];
    await useThemeStudioStore.getState().load();
    await useThemeStudioStore.getState().apply(storedThemes[0]);
    expect(useThemeStore.getState().config.color_theme).toBe("doomed");

    await useThemeStudioStore.getState().remove("doomed");

    expect(invokeMock.mock.calls).toContainEqual([
      "delete_theme",
      { id: "doomed" },
    ]);
    expect(useThemeStore.getState().config.color_theme).toBeNull();
    const prefs = savedPrefs();
    expect(prefs[prefs.length - 1]?.color_theme).toBeNull();
    expect(useThemeStudioStore.getState().userThemes).toHaveLength(0);
  });

  it("imports a VS Code theme and uniquifies colliding ids", async () => {
    importContent = JSON.stringify({
      name: "Anar Dark",
      type: "dark",
      colors: {
        "editor.background": "#101010",
        "editor.foreground": "#f0f0f0",
        focusBorder: "#ff8800",
      },
    });

    const imported = await useThemeStudioStore.getState().importTheme();
    expect(imported?.id).toBe("anar-dark-2");
    expect(imported?.colors.bg).toBe("#101010");
    expect(useThemeStudioStore.getState().error).toBeNull();
    expect(useThemeStudioStore.getState().userThemes.map((t) => t.id)).toEqual([
      "anar-dark-2",
    ]);
  });

  it("returns null and reports an error for unreadable files", async () => {
    importContent = "definitely not json";
    const imported = await useThemeStudioStore.getState().importTheme();
    expect(imported).toBeNull();
    expect(useThemeStudioStore.getState().error).toMatch(/not a theme/i);
    expect(invokeMock.mock.calls.some((c) => c[0] === "save_theme")).toBe(
      false,
    );
  });

  it("preview paints the draft and stopPreview restores the active theme", async () => {
    const preview = theme("Preview Me", {
      id: "preview-me",
      colors: { ...blankTheme("x").colors, bg: "#0a0b0c" },
    });
    useThemeStudioStore.getState().preview(preview);
    expect(cssVars.get("--evoury-bg")).toBe("#0a0b0c");

    useThemeStudioStore.getState().stopPreview();
    expect(cssVars.has("--evoury-bg")).toBe(false);
  });

  it("exports through the backend", async () => {
    const t = theme("Exportable", { id: "exportable" });
    await useThemeStudioStore.getState().exportTheme(t);
    expect(invokeMock.mock.calls).toContainEqual([
      "export_theme",
      { theme: t },
    ]);
  });
});
