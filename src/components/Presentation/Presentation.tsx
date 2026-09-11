import { useState, useEffect, useCallback, useMemo } from "react";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { invoke } from "@tauri-apps/api/tauri";
import type { Asset } from "../../types";

interface Props {
  assets: Asset[];
  onClose: () => void;
}

export default function Presentation({ assets, onClose }: Props) {
  // Resume from saved position
  const [position, setPosition] = useState(() => {
    const saved = localStorage.getItem("evoury_presentation_pos");
    return saved ? parseInt(saved, 10) : 0;
  });

  const previewAssets = useMemo(() => assets.filter((a) => a.preview?.path || a.archive_path), [assets]);
  const current = previewAssets[position];

  // Save position on change
  useEffect(() => {
    localStorage.setItem("evoury_presentation_pos", String(position));
  }, [position]);

  const next = useCallback(() => {
    setPosition((p) => Math.min(p + 1, previewAssets.length - 1));
  }, [previewAssets.length]);

  const prev = useCallback(() => {
    setPosition((p) => Math.max(p - 1, 0));
  }, []);

  useEffect(() => {
    function handler(e: KeyboardEvent) {
      if (e.key === "Escape") onClose();
      if (e.key === "ArrowRight") next();
      if (e.key === "ArrowLeft") prev();
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose, next, prev]);

  function openArchive(archivePath: string) {
    invoke("open_external_file", { path: archivePath }).catch(() => {});
  }

  if (previewAssets.length === 0) return null;

  return (
    <div className="fixed inset-0 z-50 flex flex-col"
      style={{ background: "var(--evoury-bg)" }}>
      <div className="flex items-center justify-between px-5 py-3 animate-slide-up"
        style={{ background: "rgba(10, 10, 15, 0.8)", backdropFilter: "blur(20px)", borderBottom: "1px solid var(--evoury-border)" }}>
        <div className="flex items-center gap-3">
          <div className="w-6 h-6 rounded-lg flex items-center justify-center text-[10px] font-bold"
            style={{ background: "linear-gradient(135deg, var(--evoury-accent), var(--evoury-accent2))", color: "white" }}>
            E
          </div>
          <span className="text-xs" style={{ color: "var(--evoury-text-dim)" }}>
            Presentation
          </span>
          <span className="text-xs font-mono" style={{ color: "var(--evoury-border-light)" }}>
            {position + 1} of {previewAssets.length}
          </span>
          {current && (
            <span className="text-[10px] truncate max-w-[200px]" style={{ color: "var(--evoury-text)" }}>
              {current.name}
            </span>
          )}
        </div>
        <button onClick={onClose}
          className="text-xs transition-all px-3 py-1.5 rounded-lg flex items-center gap-1.5"
          style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}>
          <i className="fas fa-times mr-1 text-[10px]" />
          Exit (Esc)
        </button>
      </div>

      <div className="flex-1 flex items-center justify-center p-4 sm:p-8 overflow-hidden">
        {current && (
          <div
            onClick={() => current.archive_path && openArchive(current.archive_path)}
            className="flex items-center justify-center w-full h-full cursor-pointer"
          >
            {current.preview?.path ? (
              <img
                src={convertFileSrc(current.preview.path)}
                alt={current.name}
                className="max-w-full max-h-full object-contain rounded-xl transition-all duration-300 hover:scale-[1.02]"
                style={{ boxShadow: "0 8px 40px rgba(0,0,0,0.4)", maxWidth: "95%", maxHeight: "95%" }}
              />
            ) : (
              <div className="text-center">
                <div className="w-16 h-16 rounded-2xl flex items-center justify-center mx-auto mb-3"
                  style={{ background: "var(--evoury-elevated)" }}>
                  <i className="fas fa-image text-2xl" style={{ color: "var(--evoury-text-dim)" }} />
                </div>
                <span className="text-sm" style={{ color: "var(--evoury-text-dim)" }}>No Preview</span>
                <p className="text-[10px] mt-1" style={{ color: "var(--evoury-border-light)" }}>
                  Click to open archive
                </p>
              </div>
            )}
          </div>
        )}
      </div>

      <div className="flex items-center justify-center gap-4 px-4 py-4 animate-slide-up"
        style={{ background: "rgba(10, 10, 15, 0.8)", backdropFilter: "blur(20px)", borderTop: "1px solid var(--evoury-border)" }}>
        <button onClick={prev}
          className="px-5 py-2 text-sm rounded-lg transition-all flex items-center gap-2"
          style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}>
          <i className="fas fa-chevron-left text-[10px]" /> Previous
        </button>
        <button onClick={next}
          className="px-5 py-2 text-sm rounded-lg transition-all flex items-center gap-2"
          style={{ background: "var(--evoury-elevated)", color: "var(--evoury-text-dim)", border: "1px solid var(--evoury-border)" }}>
          Next <i className="fas fa-chevron-right text-[10px]" />
        </button>
      </div>
    </div>
  );
}
