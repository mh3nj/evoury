export interface AssetMetadata {
  asset_id: string;
  tags: string[];
  notes: string;
  favorite: boolean;
  rating: number;
  last_opened: string | null;
  times_opened: number;
  custom_fields: Record<string, string>;
}

export interface HierarchicalTag {
  id: string;
  name: string;
  parent_id: string | null;
  color: string | null;
  description: string;
  namespace: string;
}

export interface CustomFieldDefinition {
  id: string;
  name: string;
  field_type: "Text" | "Number" | "Boolean" | "Date" | "Dropdown" | "Url" | "Color" | "Dimensions";
  required: boolean;
  default_value: string | null;
  description: string;
  category: string;
}

export interface MetadataTemplate {
  id: string;
  name: string;
  description: string;
  tags: string[];
  custom_fields: { name: string; value: string }[];
  notes_template: string;
  rating: number | null;
  favorite: boolean | null;
}

export interface AutoClassifyRule {
  id: string;
  name: string;
  extensions: string[];
  mime_patterns: string[];
  assign_tags: string[];
  assign_collection_id: string | null;
  assign_rating: number | null;
  assign_favorite: boolean | null;
  priority: number;
}
