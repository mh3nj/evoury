/** A library folder (several can be added). */
export interface LibraryRoot {
  id: string;
  path: string;
  name: string;
  added_at: string;
  last_scanned_at: string | null;
  asset_count: number;
  /** False while the folder can't be reached (unplugged drive, share down). */
  available: boolean;
}

export interface ScanSummary {
  added: number;
  updated: number;
  removed: number;
  total: number;
  read_folders: number;
  reused_folders: number;
  duration_ms: number;
  errors: string[];
}

export interface AnalyzeProgress {
  analyzed: number;
  failed: number;
  needs_tool: number;
  remaining: number;
}

export interface ExactDuplicateGroup {
  hash: string;
  size: number;
  assets: string[];
}

export interface SharedFile {
  size: number;
  crc: number;
  names: string[];
  assets: string[];
}

export interface SimilarPair {
  a: string;
  b: string;
  shared_files: number;
  shared_bytes: number;
  ratio: number;
  identical: boolean;
}

export interface DuplicateReport {
  exact: ExactDuplicateGroup[];
  shared_files: SharedFile[];
  similar: SimilarPair[];
  analyzed: number;
  total: number;
}
