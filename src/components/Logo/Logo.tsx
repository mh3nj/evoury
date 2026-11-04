import { LOGO_LOCKUP, LOGO_MARK } from "./logo-paths";

interface Props {
  size?: number;
  showText?: boolean;
  className?: string;
}

/**
 * The Evoury brand mark. Both shapes are the real SVGs from `assets/logo`,
 * filled with `--evoury-logo` so the logo follows the active theme colour
 * (editable per theme in the Theme Studio).
 */
export default function Logo({
  size = 28,
  showText = false,
  className,
}: Props) {
  if (showText) {
    // Horizontal lockup: mark + logotype, 1200x300 (4:1).
    const height = Math.round(size * 1.4);
    return (
      <svg
        role="img"
        aria-label="Evoury"
        viewBox="0 0 1200 300"
        width={height * 4}
        height={height}
        className={className}
        style={{ display: "block" }}
      >
        {LOGO_LOCKUP.map((p, i) => (
          <path
            key={i}
            d={p.d}
            fillRule={p.fillRule}
            fill="var(--evoury-logo)"
          />
        ))}
      </svg>
    );
  }

  return (
    <svg
      role="img"
      aria-label="Evoury"
      viewBox="0 0 1200 1200"
      width={size}
      height={size}
      className={className}
      style={{ display: "block" }}
    >
      {LOGO_MARK.map((p, i) => (
        <path key={i} d={p.d} fillRule={p.fillRule} fill="var(--evoury-logo)" />
      ))}
    </svg>
  );
}
