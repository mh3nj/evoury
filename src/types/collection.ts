export interface Category {
  id: string;
  name: string;
  path: string;
}

export interface Collection {
  id: string;
  name: string;
  path: string | null;
  parent_id?: string | null;
  icon_path: string | null;
  accent_color: string | null;
  created_at?: string;
  categories?: Category[];
  settings?: CollectionSettings;
}

export interface CollectionSettings {
  background: string;
  invert_preview: boolean;
  blend_mode: "Normal" | "Multiply" | "Screen" | "Difference" | "ColorDodge";
}

export interface SmartCollection {
  id: string;
  name: string;
  description: string;
  icon: string;
  rules: SmartRule[];
  match_all: boolean;
  auto_update: boolean;
  created_at: string;
}

export interface SmartRule {
  field: "Tag" | "Rating" | "Favorite" | "Type" | "Name" | "Notes" | "DateAdded" | "DateModified" | "FileSize";
  operator: "Equals" | "NotEquals" | "GreaterThan" | "LessThan" | "Contains" | "StartsWith" | "EndsWith" | "InRange";
  value: string;
}
