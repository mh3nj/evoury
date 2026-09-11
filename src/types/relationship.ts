export type RelationshipKind =
  | "Reference" | "DerivedFrom" | "Version" | "Parent" | "Child"
  | "Dependency" | "Font" | "Image" | "Link";

export interface Relationship {
  id: string;
  source_id: string;
  target_id: string;
  kind: RelationshipKind;
  label: string;
  metadata: string;
  created_at: string;
}
