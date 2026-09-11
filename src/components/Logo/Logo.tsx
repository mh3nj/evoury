interface Props {
  size?: number;
  showText?: boolean;
}

export default function Logo({ size = 28, showText = false }: Props) {
  return (
    <div className="flex items-center gap-2.5">
      <img src="/logo/logo.png" alt="Evoury" width={size} height={size}
        style={{ borderRadius: 6, objectFit: "contain" }} />
      {showText && (
        <div>
          <div className="text-sm font-semibold tracking-tight" style={{ color: "var(--evoury-text)" }}>Evoury</div>
          <div className="text-[9px] leading-none mt-0.5 tracking-wider uppercase" style={{ color: "var(--evoury-text-dim)" }}>Enter the Flow</div>
        </div>
      )}
    </div>
  );
}
