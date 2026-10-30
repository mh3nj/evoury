import { useEffect, useMemo, useState } from "react";
import { useCommandStore, useThemeStore } from "../../stores";
import { useThemeStudioStore } from "../../stores/theme-studio-store";
import {
  BUILTIN_THEMES,
  blankTheme,
  hexToRgba,
  mergeThemes,
} from "../../themes/palette";
import type { StudioTheme, ThemeColors } from "../../types/theme";
import { COLOR_KEYS_SOLID } from "../../types/theme";

const COLOR_LABELS: Record<keyof ThemeColors, string> = {
  bg: "Background",
  surface: "Surface",
  elevated: "Elevated",
  border: "Border",
  border_light: "Border light",
  text: "Text",
  text_dim: "Text dim",
  accent: "Accent",
  accent_glow: "Accent glow",
  accent2: "Accent 2",
  accent2_glow: "Accent 2 glow",
  success: "Success",
  warning: "Warning",
  error: "Error",
};

export default function ThemeStudio() {
  const { themeStudioOpen, closeThemeStudio } = useCommandStore();
  const { config } = useThemeStore();
  const {
    loaded,
    userThemes,
    error,
    load,
    apply,
    save,
    saveAndApply,
    remove,
    importTheme,
    exportTheme,
    preview,
    stopPreview,
  } = useThemeStudioStore();
  const [draft, setDraft] = useState<StudioTheme | null>(null);
  const [editingBaseId, setEditingBaseId] = useState<string | null>(null);
  const [previewId, setPreviewId] = useState<string | null>(null);

  useEffect(() => {
    if (themeStudioOpen && !loaded) load();
  }, [themeStudioOpen, loaded, load]);

  useEffect(() => {
    if (!themeStudioOpen) return;
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") {
        if (draft) setDraft(null);
        else closeThemeStudio();
      }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [themeStudioOpen, draft, closeThemeStudio]);

  const allThemes = useMemo(
    () => [...mergeThemes(userThemes).values()],
    [userThemes],
  );
  const isCustom = (id: string) => userThemes.some((t) => t.id === id);
  const isActive = (id: string) => config.color_theme === id;

  function close() {
    stopPreview();
    setPreviewId(null);
    setDraft(null);
    closeThemeStudio();
  }

  function nextNewId(base: string): string {
    const taken = new Set(allThemes.map((t) => t.id));
    if (!taken.has(base)) return base;
    let n = 2;
    while (taken.has(`${base}-${n}`)) n += 1;
    return `${base}-${n}`;
  }

  function openEditor(theme: StudioTheme | null) {
    if (theme) {
      setDraft({ ...theme, colors: { ...theme.colors } });
      setEditingBaseId(theme.id);
      return;
    }
    const fresh = blankTheme("New Theme");
    setDraft({ ...fresh, id: nextNewId(fresh.id) });
    setEditingBaseId(null);
  }

  function handlePreview(theme: StudioTheme) {
    if (previewId === theme.id) {
      stopPreview();
      setPreviewId(null);
    } else {
      preview(theme);
      setPreviewId(theme.id);
    }
  }

  if (!themeStudioOpen) return null;

  const draftTheme: StudioTheme = draft
    ? {
        ...draft,
        colors: {
          ...draft.colors,
          accent_glow: hexToRgba(draft.colors.accent, 0.2),
          accent2_glow: hexToRgba(draft.colors.accent2, 0.15),
        },
      }
    : blankTheme("New Theme");

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      style={{ background: "rgba(0, 0, 0, 0.6)", backdropFilter: "blur(8px)" }}
      onClick={close}
    >
      <div
        className="w-full max-w-5xl max-h-[88vh] overflow-y-auto animate-scale-in"
        style={{
          background: "var(--evoury-surface)",
          border: "1px solid var(--evoury-border-light)",
          borderRadius: "16px",
          boxShadow: "0 24px 80px rgba(0,0,0,0.5)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <div
          className="flex items-center justify-between px-5 py-3.5"
          style={{ borderBottom: "1px solid var(--evoury-border)" }}
        >
          <h2
            className="text-sm font-semibold"
            style={{ color: "var(--evoury-text)" }}
          >
            <i
              className="fas fa-palette mr-2"
              style={{ color: "var(--evoury-accent)" }}
            />
            Theme Studio
            <span
              className="ml-3 text-[10px] font-normal"
              style={{ color: "var(--evoury-text-dim)" }}
            >
              {allThemes.length} themes: apply, edit, save, import and export
            </span>
          </h2>
          <div className="flex gap-2">
            <button
              onClick={() => openEditor(null)}
              className="text-[10px] px-2.5 py-1.5 rounded-md transition-all"
              style={{
                background: "var(--evoury-accent)",
                color: "white",
              }}
            >
              <i className="fas fa-plus mr-1.5" />
              New theme
            </button>
            <button
              onClick={async () => {
                const t = await importTheme();
                if (t) openEditor(t);
              }}
              className="text-[10px] px-2.5 py-1.5 rounded-md transition-all"
              style={{
                background: "var(--evoury-elevated)",
                color: "var(--evoury-text)",
                border: "1px solid var(--evoury-border)",
              }}
            >
              <i className="fas fa-file-import mr-1.5" />
              Import
            </button>
            <button
              onClick={close}
              className="text-[10px] px-2 py-1 rounded-md"
              style={{
                background: "var(--evoury-elevated)",
                color: "var(--evoury-text-dim)",
              }}
            >
              Esc
            </button>
          </div>
        </div>

        {error && (
          <div
            className="mx-5 mt-3 text-xs px-3 py-2 rounded-md"
            style={{
              background: "var(--evoury-error)",
              color: "white",
            }}
          >
            {error}
          </div>
        )}

        {draft && (
          <EditorPane
            draft={draftTheme}
            isEditingBuiltin={
              editingBaseId !== null &&
              BUILTIN_THEMES.some((t) => t.id === editingBaseId)
            }
            onChange={(colors) => setDraft({ ...draftTheme, colors })}
            onName={(name) => setDraft({ ...draftTheme, name })}
            onMode={(mode) => setDraft({ ...draftTheme, mode })}
            onCancel={() => setDraft(null)}
            onSave={() => {
              void save(draftTheme).then(() => setDraft(null));
            }}
            onSaveAndApply={() => {
              void saveAndApply(draftTheme).then(() => setDraft(null));
            }}
          />
        )}

        <div className="p-5 pt-3 grid grid-cols-2 md:grid-cols-3 gap-3">
          {allThemes.map((theme) => (
            <div
              key={theme.id}
              className="rounded-xl p-3 transition-all"
              style={{
                background: "var(--evoury-elevated)",
                border: isActive(theme.id)
                  ? "1px solid var(--evoury-accent)"
                  : "1px solid var(--evoury-border)",
              }}
            >
              <div
                className="h-6 rounded-md mb-2 flex overflow-hidden"
                style={{ border: "1px solid var(--evoury-border)" }}
              >
                {[
                  theme.colors.bg,
                  theme.colors.surface,
                  theme.colors.elevated,
                  theme.colors.accent,
                  theme.colors.accent2,
                  theme.colors.text,
                ].map((c, i) => (
                  <div key={i} className="flex-1" style={{ background: c }} />
                ))}
              </div>
              <div className="flex items-center justify-between mb-1.5">
                <span
                  className="text-xs font-medium truncate"
                  style={{ color: "var(--evoury-text)" }}
                >
                  {theme.name}
                </span>
                <span
                  className="text-[8px] px-1.5 py-0.5 rounded-md uppercase tracking-wider ml-2"
                  style={{
                    background:
                      theme.mode === "dark"
                        ? "var(--evoury-accent-glow)"
                        : "rgba(255, 255, 255, 0.15)",
                    color: "var(--evoury-text-dim)",
                  }}
                >
                  {theme.mode}
                </span>
              </div>
              <div
                className="text-[9px] mb-2"
                style={{ color: "var(--evoury-text-dim)" }}
              >
                {isCustom(theme.id)
                  ? "custom" +
                    (BUILTIN_THEMES.some((t) => t.id === theme.id)
                      ? " (overrides built-in)"
                      : "")
                  : "built-in"}
                {isActive(theme.id) ? " · active" : ""}
              </div>
              <div className="flex flex-wrap gap-1.5">
                {!isActive(theme.id) && (
                  <button
                    onClick={() => void apply(theme)}
                    className="text-[9px] px-2 py-1 rounded-md"
                    style={{
                      background: "var(--evoury-accent)",
                      color: "white",
                    }}
                  >
                    Apply
                  </button>
                )}
                <button
                  onClick={() => handlePreview(theme)}
                  className="text-[9px] px-2 py-1 rounded-md"
                  style={{
                    background:
                      previewId === theme.id
                        ? "var(--evoury-accent-glow)"
                        : "var(--evoury-elevated)",
                    color:
                      previewId === theme.id
                        ? "var(--evoury-accent)"
                        : "var(--evoury-text-dim)",
                    border: "1px solid var(--evoury-border)",
                  }}
                >
                  {previewId === theme.id ? "Stop" : "Preview"}
                </button>
                <button
                  onClick={() => openEditor(theme)}
                  className="text-[9px] px-2 py-1 rounded-md"
                  style={{
                    background: "var(--evoury-elevated)",
                    color: "var(--evoury-text-dim)",
                    border: "1px solid var(--evoury-border)",
                  }}
                >
                  Edit
                </button>
                <button
                  onClick={() => void exportTheme(theme)}
                  className="text-[9px] px-2 py-1 rounded-md"
                  style={{
                    background: "var(--evoury-elevated)",
                    color: "var(--evoury-text-dim)",
                    border: "1px solid var(--evoury-border)",
                  }}
                >
                  Export
                </button>
                {isCustom(theme.id) && (
                  <button
                    onClick={() => void remove(theme.id)}
                    className="text-[9px] px-2 py-1 rounded-md"
                    style={{
                      background: "transparent",
                      color: "var(--evoury-error)",
                    }}
                  >
                    Delete
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

function EditorPane({
  draft,
  isEditingBuiltin,
  onChange,
  onName,
  onMode,
  onCancel,
  onSave,
  onSaveAndApply,
}: {
  draft: StudioTheme;
  isEditingBuiltin: boolean;
  onChange: (colors: ThemeColors) => void;
  onName: (name: string) => void;
  onMode: (mode: "dark" | "light") => void;
  onCancel: () => void;
  onSave: () => void;
  onSaveAndApply: () => void;
}) {
  return (
    <div
      className="mx-5 mt-4 rounded-xl p-4 flex gap-4 flex-wrap"
      style={{
        background: "var(--evoury-elevated)",
        border: "1px solid var(--evoury-border)",
      }}
    >
      <div className="flex-1 min-w-64">
        <div className="flex items-center gap-2 mb-3">
          <input
            value={draft.name}
            onChange={(e) => onName(e.target.value)}
            className="flex-1 px-2 py-1.5 rounded-md text-xs outline-none"
            style={{
              background: "var(--evoury-bg)",
              color: "var(--evoury-text)",
              border: "1px solid var(--evoury-border)",
            }}
            placeholder="Theme name"
          />
          <div className="flex gap-1">
            {(["dark", "light"] as const).map((m) => (
              <button
                key={m}
                onClick={() => onMode(m)}
                className="text-[9px] px-2 py-1 rounded-md"
                style={{
                  background:
                    draft.mode === m
                      ? "var(--evoury-accent)"
                      : "var(--evoury-elevated)",
                  color: draft.mode === m ? "white" : "var(--evoury-text-dim)",
                  border: "1px solid var(--evoury-border)",
                }}
              >
                {m}
              </button>
            ))}
          </div>
        </div>
        <div className="grid grid-cols-2 gap-x-4 gap-y-1.5">
          {COLOR_KEYS_SOLID.map((key) => (
            <label
              key={key}
              className="flex items-center gap-2 text-[10px]"
              style={{ color: "var(--evoury-text-dim)" }}
            >
              <input
                type="color"
                value={draft.colors[key]}
                onChange={(e) =>
                  onChange({ ...draft.colors, [key]: e.target.value })
                }
                className="w-7 h-6 rounded cursor-pointer"
                style={{
                  border: "1px solid var(--evoury-border)",
                  background: "transparent",
                }}
              />
              <span className="w-16 truncate">{COLOR_LABELS[key]}</span>
              <span
                className="text-[9px]"
                style={{ color: "var(--evoury-text)" }}
              >
                {draft.colors[key]}
              </span>
            </label>
          ))}
        </div>
        {isEditingBuiltin && (
          <p
            className="text-[9px] mt-2"
            style={{ color: "var(--evoury-text-dim)" }}
          >
            Saving creates a custom copy that overrides the built-in. Delete it
            to get the original back.
          </p>
        )}
        <div className="flex gap-2 mt-4">
          <button
            onClick={onSave}
            className="text-[10px] px-3 py-1.5 rounded-md"
            style={{
              background: "var(--evoury-accent)",
              color: "white",
            }}
          >
            Save
          </button>
          <button
            onClick={onSaveAndApply}
            className="text-[10px] px-3 py-1.5 rounded-md"
            style={{
              background: "var(--evoury-accent-glow)",
              color: "var(--evoury-accent)",
              border: "1px solid var(--evoury-accent)",
            }}
          >
            Save and Apply
          </button>
          <button
            onClick={onCancel}
            className="text-[10px] px-3 py-1.5 rounded-md"
            style={{
              background: "var(--evoury-elevated)",
              color: "var(--evoury-text-dim)",
              border: "1px solid var(--evoury-border)",
            }}
          >
            Cancel
          </button>
        </div>
      </div>
      <PreviewMock draft={draft} />
    </div>
  );
}

function PreviewMock({ draft }: { draft: StudioTheme }) {
  const vars = {
    "--evoury-bg": draft.colors.bg,
    "--evoury-surface": draft.colors.surface,
    "--evoury-elevated": draft.colors.elevated,
    "--evoury-border": draft.colors.border,
    "--evoury-border-light": draft.colors.border_light,
    "--evoury-text": draft.colors.text,
    "--evoury-text-dim": draft.colors.text_dim,
    "--evoury-accent": draft.colors.accent,
    "--evoury-accent-glow": draft.colors.accent_glow,
    "--evoury-accent2": draft.colors.accent2,
    "--evoury-accent2-glow": draft.colors.accent2_glow,
    "--evoury-success": draft.colors.success,
    "--evoury-warning": draft.colors.warning,
    "--evoury-error": draft.colors.error,
  } as React.CSSProperties;

  return (
    <div
      className="w-72 rounded-lg p-2.5"
      style={{ ...vars, background: "var(--evoury-bg)" }}
    >
      <p
        className="text-[9px] uppercase tracking-wider mb-2"
        style={{ color: "var(--evoury-text-dim)" }}
      >
        Live preview
      </p>
      <div
        className="rounded-md overflow-hidden"
        style={{
          background: "var(--evoury-bg)",
          border: "1px solid var(--evoury-border)",
        }}
      >
        <div
          className="flex items-center gap-1 px-2 py-1.5"
          style={{
            background: "var(--evoury-surface)",
            borderBottom: "1px solid var(--evoury-border)",
          }}
        >
          <div
            className="w-2 h-2 rounded-full"
            style={{ background: "var(--evoury-error)" }}
          />
          <div
            className="w-2 h-2 rounded-full"
            style={{ background: "var(--evoury-warning)" }}
          />
          <div
            className="w-2 h-2 rounded-full"
            style={{ background: "var(--evoury-success)" }}
          />
          <span
            className="ml-2 text-[8px]"
            style={{ color: "var(--evoury-text-dim)" }}
          >
            {draft.name}
          </span>
        </div>
        <div className="flex">
          <div
            className="w-14 p-1.5 space-y-1"
            style={{
              background: "var(--evoury-surface)",
              borderRight: "1px solid var(--evoury-border)",
            }}
          >
            {[0, 1, 2, 3].map((i) => (
              <div
                key={i}
                className="h-1.5 rounded"
                style={{
                  background:
                    i === 0 ? "var(--evoury-accent)" : "var(--evoury-elevated)",
                }}
              />
            ))}
          </div>
          <div className="flex-1 p-2">
            <div className="flex gap-1.5 mb-2">
              {[0, 1, 2].map((i) => (
                <div
                  key={i}
                  className="flex-1 h-10 rounded"
                  style={{
                    background:
                      i === 1
                        ? "var(--evoury-accent-glow)"
                        : "var(--evoury-elevated)",
                    border: "1px solid var(--evoury-border)",
                  }}
                />
              ))}
            </div>
            <p
              className="text-[8px] leading-snug"
              style={{ color: "var(--evoury-text)" }}
            >
              Pack 01.rar
            </p>
            <p
              className="text-[7px]"
              style={{ color: "var(--evoury-text-dim)" }}
            >
              24 assets · 1.2 GB
            </p>
            <div
              className="mt-1.5 h-3 w-12 rounded"
              style={{ background: "var(--evoury-accent)" }}
            />
          </div>
        </div>
      </div>
      <div className="flex gap-1 mt-2">
        <span
          className="w-3 h-3 rounded-full"
          style={{ background: "var(--evoury-success)" }}
        />
        <span
          className="w-3 h-3 rounded-full"
          style={{ background: "var(--evoury-warning)" }}
        />
        <span
          className="w-3 h-3 rounded-full"
          style={{ background: "var(--evoury-error)" }}
        />
        <span
          className="w-3 h-3 rounded-full"
          style={{ background: "var(--evoury-accent)" }}
        />
        <span
          className="w-3 h-3 rounded-full"
          style={{ background: "var(--evoury-accent2)" }}
        />
      </div>
    </div>
  );
}
