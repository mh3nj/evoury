import { describe, it, expect, vi, beforeEach } from "vitest";
import type { Preferences } from "../types/preferences";

const { invokeMock } = vi.hoisted(() => ({ invokeMock: vi.fn() }));
vi.mock("@tauri-apps/api/tauri", () => ({ invoke: invokeMock }));
vi.mock("./persistence", () => ({
  loadState: () => null,
  saveState: () => {},
}));

import { useGalleryStore } from "./gallery-store";

beforeEach(() => {
  invokeMock.mockReset();
  invokeMock.mockResolvedValue(undefined);
});

function savedPrefs(): Preferences[] {
  return invokeMock.mock.calls
    .filter((c) => c[0] === "set_preferences")
    .map((c) => c[1].prefs as Preferences);
}

describe("gallery store", () => {
  it("saves the layout chosen in settings or the toolbar", () => {
    useGalleryStore.getState().setViewMode("comfortable");
    const saved = savedPrefs();
    expect(saved.length).toBeGreaterThan(0);
    expect(saved[saved.length - 1].default_view.view_mode).toBe("comfortable");
  });

  it("saves the card size", () => {
    useGalleryStore.getState().setGridSize(320);
    const saved = savedPrefs();
    expect(saved[saved.length - 1].default_view.grid_size).toBe(320);
  });

  it("applies a restored layout without writing it back", () => {
    useGalleryStore.getState().applyView(320, "compact");
    expect(useGalleryStore.getState().gridSize).toBe(320);
    expect(useGalleryStore.getState().viewMode).toBe("compact");
    expect(savedPrefs().length).toBe(0);
  });
});
