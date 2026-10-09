import { describe, it, expect, vi, beforeEach } from "vitest";
import type { Preferences } from "../types/preferences";

const { invokeMock } = vi.hoisted(() => ({ invokeMock: vi.fn() }));
vi.mock("@tauri-apps/api/tauri", () => ({ invoke: invokeMock }));

import { usePreferencesStore } from "./preferences-store";

const DEFAULTS = {
  loaded: false,
  theme: "Dark" as const,
  density: "Normal" as const,
  default_view: { grid_size: 240, remember_last_collection: true, show_details: false, view_mode: "grid" as const },
  performance: { max_preview_memory: 512, enable_background_indexing: true, reduce_motion: false },
  interaction: { keyboard_navigation: true, double_click_to_open: true, show_tooltips: true },
  default_workspace: null,
};

beforeEach(() => {
  invokeMock.mockReset();
  invokeMock.mockResolvedValue(undefined);
  usePreferencesStore.setState(DEFAULTS);
});

function lastSaved(): Preferences {
  const calls = invokeMock.mock.calls.filter((c) => c[0] === "set_preferences");
  expect(calls.length).toBeGreaterThan(0);
  return calls[calls.length - 1][1].prefs as Preferences;
}

describe("preferences store", () => {
  it("saves the complete payload when density changes", () => {
    usePreferencesStore.getState().setDensity("Comfortable");
    const prefs = lastSaved();
    expect(prefs.density).toBe("Comfortable");
    expect(prefs.default_view).toBeDefined();
    expect(prefs.performance).toBeDefined();
    expect(prefs.interaction).toBeDefined();
  });

  it("saves layout and card size together", () => {
    usePreferencesStore.getState().setView({ view_mode: "comfortable", grid_size: 320 });
    const prefs = lastSaved();
    expect(prefs.default_view.view_mode).toBe("comfortable");
    expect(prefs.default_view.grid_size).toBe(320);
  });

  it("restores saved settings from the backend", async () => {
    const stored: Preferences = {
      theme: "Light",
      density: "Comfortable",
      default_workspace: null,
      default_view: { grid_size: 300, remember_last_collection: false, show_details: true, view_mode: "compact" },
      performance: { max_preview_memory: 256, enable_background_indexing: false, reduce_motion: true },
      interaction: { keyboard_navigation: false, double_click_to_open: false, show_tooltips: false },
    };
    invokeMock.mockResolvedValueOnce(stored);

    await usePreferencesStore.getState().load();
    const state = usePreferencesStore.getState();
    expect(state.loaded).toBe(true);
    expect(state.theme).toBe("Light");
    expect(state.density).toBe("Comfortable");
    expect(state.default_view.view_mode).toBe("compact");
    expect(state.default_view.grid_size).toBe(300);
    expect(state.performance.reduce_motion).toBe(true);
  });
});
