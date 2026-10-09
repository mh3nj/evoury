export type OsKind = "windows" | "macos" | "linux" | "unknown";

export function detectOs(): OsKind {
  const ua = (navigator.userAgent || "").toLowerCase();
  if (ua.includes("win")) return "windows";
  if (ua.includes("mac")) return "macos";
  if (ua.includes("linux")) return "linux";
  return "unknown";
}

export function revealLabel(): string {
  switch (detectOs()) {
    case "macos":
      return "Reveal in Finder";
    case "windows":
      return "Show in Explorer";
    case "linux":
      return "Show in Folder";
    default:
      return "Show in Folder";
  }
}
