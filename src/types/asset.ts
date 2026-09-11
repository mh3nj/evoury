export interface Asset {
  id: string;
  name: string;
  mime_type?: string;
  archive_type: "Zip" | "Rar" | "SevenZip" | "Tar" | "Unknown";
  archive_path: string;
  preview: { path: string } | null;
  file_size: number;
  modified_at: string;
  collection_id: string | null;
  category_id: string | null;
  state: "Discovered" | "Paired" | "Validated" | "Indexed" | "Ready" | "Broken";
}

export interface AssetSummary {
  id: string;
  name: string;
  preview: string | null;
  archive_type: string;
  health: "ok" | "missing_preview" | "missing_source";
}

export interface ScanProgress {
  scan_id: string;
  mode: string;
  total_files: number;
  scanned_files: number;
  archives_found: number;
  previews_found: number;
  assets_created: number;
  errors: string[];
  current_file: string | null;
}
