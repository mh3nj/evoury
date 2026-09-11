export interface SearchQuery {
  text?: string[];
  tags?: string[];
  collection?: string;
  category?: string;
  file_type?: string;
  favorite_only?: boolean;
  min_rating?: number;
}

export interface SearchResult {
  asset_id: string;
  score: number;
}

export interface ParsedFilter {
  field: string;
  value: string;
  label: string;
}

export interface SavedSearch {
  id: string;
  name: string;
  query_text: string;
  icon: string;
  color: string | null;
  folder_id: string | null;
  created_at: string;
}

export interface SearchFolder {
  id: string;
  name: string;
  parent_id: string | null;
  icon: string;
  color: string | null;
}
