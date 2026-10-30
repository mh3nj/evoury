import { useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import type { Asset, AnalyzeProgress, DuplicateReport } from "../../types";
import { formatBytes } from "../../utils/archive-tree";
import { analysisFinished } from "../../utils/duplicates";
import { Modal } from "./Modal";

interface Props {
  assets: Asset[];
  onClose: () => void;
}

type Phase = "idle" | "reading" | "comparing" | "done" | "error";

const dim = { color: "var(--evoury-text-dim)" } as const;
const chip = {
  background: "var(--evoury-elevated)",
  border: "1px solid var(--evoury-border)",
} as const;

/**
 * Duplicates, three ways:
 *  - identical archives (same bytes)
 *  - near-identical packages (the same pack saved twice, in two formats or two places)
 *  - files that sit inside several packages (the same font bundled into twelve mockups)
 *
 * Nothing is deleted here; it only shows where the copies are. The last two need the
 * contents of each archive, which Kaelovun's manifest provides instantly (otherwise
 * 7-Zip reads them once and Evoury remembers).
 */
export default function DuplicatesDialog({ assets, onClose }: Props) {
  const [phase, setPhase] = useState<Phase>("idle");
  const [read, setRead] = useState({ done: 0, total: 0 });
  const [needsTool, setNeedsTool] = useState(0);
  const [report, setReport] = useState<DuplicateReport | null>(null);
  const [error, setError] = useState<string | null>(null);
  const cancelled = useRef(false);

  const byId = useMemo(() => new Map(assets.map((a) => [a.id, a])), [assets]);

  const start = async () => {
    cancelled.current = false;
    setError(null);
    setReport(null);
    try {
      setPhase("reading");
      let total = 0;
      let done = 0;
      let tool = 0;
      for (;;) {
        const p = await invoke<AnalyzeProgress>("analyze_package_contents", {
          limit: 40,
        });
        if (total === 0) total = p.analyzed + p.failed + p.remaining;
        done += p.analyzed + p.failed;
        tool = p.needs_tool;
        setRead({ done, total });
        setNeedsTool(tool);
        if (cancelled.current) throw new Error("Cancelled.");
        if (analysisFinished(p)) break;
      }
      setPhase("comparing");
      const r = await invoke<DuplicateReport>("find_duplicates");
      setReport(r);
      setPhase("done");
    } catch (e) {
      const message = String(e).replace(/^Error:\s*/, "");
      if (message === "Cancelled.") {
        setPhase("idle");
      } else {
        setError(message);
        setPhase("error");
      }
    }
  };

  const stop = () => {
    cancelled.current = true;
    void invoke("cancel_catalog_task");
  };

  const name = (id: string) => byId.get(id)?.name ?? "(removed)";
  const reveal = (id: string) => {
    const a = byId.get(id);
    if (a) void invoke("open_file_location", { path: a.archive_path });
  };

  const working = phase === "reading" || phase === "comparing";
  const nothing =
    report &&
    report.exact.length === 0 &&
    report.similar.length === 0 &&
    report.shared_files.length === 0;

  const AssetLine = ({ id }: { id: string }) => (
    <button
      className="text-left text-xs truncate hover:underline"
      title={byId.get(id)?.archive_path}
      onClick={() => reveal(id)}
    >
      <i className="fas fa-folder-open mr-1.5 text-[10px]" style={dim} />
      {name(id)}
      <span className="ml-2" style={dim}>
        {
          byId
            .get(id)
            ?.archive_path.replace(/\\/g, "/")
            .split("/")
            .slice(-2, -1)[0]
        }
      </span>
    </button>
  );

  return (
    <Modal title="Duplicates" icon="fa-clone" onClose={onClose} width={760}>
      <p className="text-xs mb-4" style={dim}>
        Finds identical archives, packs saved twice, and files that sit inside
        several packages. Nothing is deleted here. Click a name to show it in
        the file manager.
      </p>

      <div className="flex items-center gap-3 mb-4">
        <button
          onClick={start}
          disabled={working}
          className="premium-btn px-4 py-1.5 text-xs disabled:opacity-50"
        >
          <i
            className={`fas ${working ? "fa-spinner fa-pulse" : "fa-magnifying-glass"} mr-1.5 text-[10px]`}
          />
          {working ? "Working..." : report ? "Search again" : "Find duplicates"}
        </button>
        {working && (
          <button
            onClick={stop}
            className="px-3 py-1.5 text-xs rounded-lg"
            style={chip}
          >
            Stop
          </button>
        )}
        <span role="status" className="text-xs" style={dim}>
          {phase === "reading" &&
            `Reading package contents... ${read.done.toLocaleString()} of ${read.total.toLocaleString()}`}
          {phase === "comparing" && "Comparing archives..."}
          {phase === "done" &&
            report &&
            `Looked inside ${report.analyzed.toLocaleString()} of ${report.total.toLocaleString()} archives`}
        </span>
      </div>

      {needsTool > 0 && (
        <div className="rounded-lg p-3 text-xs mb-4" style={chip} role="note">
          <i className="fas fa-circle-info mr-2" />
          {needsTool.toLocaleString()} archive(s) can't be looked inside because
          7-Zip isn't installed and they have no Kaelovun manifest. Identical
          archives are still found.
        </div>
      )}
      {error && (
        <div className="rounded-lg p-3 text-xs mb-4" style={chip} role="alert">
          {error}
        </div>
      )}
      {nothing && (
        <div className="text-sm py-6 text-center" style={dim}>
          No duplicates found.
        </div>
      )}

      {report && report.exact.length > 0 && (
        <section className="mb-6">
          <h3 className="text-xs font-medium mb-2">
            Identical archives ({report.exact.length}){" "}
            <span style={dim}>
              ·{" "}
              {formatBytes(
                report.exact.reduce(
                  (n, g) => n + g.size * (g.assets.length - 1),
                  0,
                ),
              )}{" "}
              could be freed
            </span>
          </h3>
          <div className="flex flex-col gap-2">
            {report.exact.map((g) => (
              <div
                key={g.hash}
                className="rounded-lg p-3 flex flex-col gap-1"
                style={chip}
              >
                <div className="text-[11px]" style={dim}>
                  {formatBytes(g.size)} each · {g.assets.length} copies
                </div>
                {g.assets.map((id) => (
                  <AssetLine key={id} id={id} />
                ))}
              </div>
            ))}
          </div>
        </section>
      )}

      {report && report.similar.length > 0 && (
        <section className="mb-6">
          <h3 className="text-xs font-medium mb-2">
            Packages with the same files ({report.similar.length})
          </h3>
          <div className="flex flex-col gap-2">
            {report.similar.slice(0, 200).map((p) => (
              <div
                key={`${p.a}-${p.b}`}
                className="rounded-lg p-3 flex flex-col gap-1"
                style={chip}
              >
                <div className="text-[11px]" style={dim}>
                  {p.identical
                    ? "Same contents"
                    : `${Math.round(p.ratio * 100)}% in common`}{" "}
                  · {p.shared_files} files · {formatBytes(p.shared_bytes)}
                </div>
                <AssetLine id={p.a} />
                <AssetLine id={p.b} />
              </div>
            ))}
          </div>
        </section>
      )}

      {report && report.shared_files.length > 0 && (
        <section>
          <h3 className="text-xs font-medium mb-2">
            Files inside several packages ({report.shared_files.length})
          </h3>
          <div className="flex flex-col gap-2">
            {report.shared_files.slice(0, 200).map((f) => (
              <div
                key={`${f.size}-${f.crc}`}
                className="rounded-lg p-3 flex flex-col gap-1"
                style={chip}
              >
                <div className="text-xs truncate" title={f.names.join(", ")}>
                  {f.names[0]}
                  {f.names.length > 1 && (
                    <span style={dim}>
                      {" "}
                      (also {f.names.slice(1).join(", ")})
                    </span>
                  )}
                </div>
                <div className="text-[11px]" style={dim}>
                  {formatBytes(f.size)} · in {f.assets.length} packages ·{" "}
                  {formatBytes(f.size * (f.assets.length - 1))} repeated
                </div>
                <div className="flex flex-wrap gap-x-4 gap-y-0.5">
                  {f.assets.slice(0, 6).map((id) => (
                    <AssetLine key={id} id={id} />
                  ))}
                  {f.assets.length > 6 && (
                    <span className="text-[11px]" style={dim}>
                      +{f.assets.length - 6} more
                    </span>
                  )}
                </div>
              </div>
            ))}
          </div>
        </section>
      )}
    </Modal>
  );
}
