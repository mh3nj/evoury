export interface VersionRecord {
  id: string;
  asset_id: string;
  label: string;
  description: string;
  previous_version_id: string | null;
  file_path: string;
  file_size: number;
  created_at: string;
}
