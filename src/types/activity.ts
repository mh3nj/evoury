export type ActivityEventType =
  | "View" | "Open" | "Export" | "Edit"
  | "TagAdd" | "TagRemove" | "RatingChange" | "FavoriteToggle" | "NoteUpdate" | "MetadataChange"
  | "RelationshipAdd" | "RelationshipRemove"
  | "ProjectAdd" | "ProjectRemove"
  | "CollectionAdd" | "CollectionRemove"
  | "VersionCreate" | "VersionRestore";

export interface ActivityEvent {
  id: string;
  asset_id: string;
  event_type: ActivityEventType;
  details: string;
  timestamp: string;
}
