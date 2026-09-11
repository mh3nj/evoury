import { useColorStore, type DisplayColorSpace } from "../../stores/color-store";

const spaces: { value: DisplayColorSpace; label: string; icon: string }[] = [
  { value: "native", label: "Native", icon: "fa-display" },
  { value: "srgb", label: "sRGB", icon: "fa-palette" },
  { value: "display-p3", label: "Display P3", icon: "fa-palette" },
  { value: "hdr10", label: "HDR10", icon: "fa-sun" },
];

export default function ColorIndicator() {
  const { displaySpace, enableIcc, enableWideGamut, setDisplaySpace, setEnableIcc, setEnableWideGamut } = useColorStore();

  return (
    <div className="flex items-center gap-1 rounded-lg p-0.5 text-[10px]"
      style={{ background: "var(--evoury-elevated)", border: "1px solid var(--evoury-border)" }}>
      {spaces.map((s) => (
        <button key={s.value} onClick={() => setDisplaySpace(s.value)}
          className={`flex items-center gap-1 px-1.5 py-1 rounded-md transition-all ${displaySpace === s.value ? "text-white" : ""}`}
          style={displaySpace === s.value ? { background: "var(--evoury-accent)" } : { color: "var(--evoury-text-dim)" }}
          title={s.label}>
          <i className={`fas ${s.icon} text-[8px]`} />
          <span>{s.label === "Display P3" ? "P3" : s.label === "Native" ? "Nat" : s.label}</span>
        </button>
      ))}
      <div className="w-px h-3 mx-1" style={{ background: "var(--evoury-border)" }} />
      <button onClick={() => setEnableIcc(!enableIcc)}
        className="flex items-center gap-1 px-1.5 py-1 rounded-md transition-all"
        style={{ color: enableIcc ? "var(--evoury-accent)" : "var(--evoury-text-dim)" }}
        title="ICC Profile">
        <i className="fas fa-file-contract text-[8px]" />
      </button>
      <button onClick={() => setEnableWideGamut(!enableWideGamut)}
        className="flex items-center gap-1 px-1.5 py-1 rounded-md transition-all"
        style={{ color: enableWideGamut ? "var(--evoury-accent)" : "var(--evoury-text-dim)" }}
        title="Wide Gamut">
        <i className="fas fa-expand-arrows-alt text-[8px]" />
      </button>
    </div>
  );
}
