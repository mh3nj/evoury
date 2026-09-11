export type NotificationLevel = "Info" | "Success" | "Warning" | "Error";
export type NotificationCategory =
  | "Scan" | "Import" | "Export" | "BatchOp" | "Macro" | "Health"
  | "Maintenance" | "Pipeline" | "System" | "Undo" | "Metadata" | "Update";

export interface AppNotification {
  id: string;
  title: string;
  message: string;
  level: NotificationLevel;
  category: NotificationCategory;
  action_label: string | null;
  action_id: string | null;
  read: boolean;
  dismissed: boolean;
  created_at: string;
}

export type MaintenanceTask =
  | "VacuumOrphanedMetadata" | "RebuildMissingPreviews" | "ReindexAll"
  | { CleanupOldVersions: { keep: number } }
  | { PruneActivityLog: { older_than_days: number } }
  | "CompactCache" | "VerifyFileIntegrity" | "RepairBrokenReferences";

export interface MaintenanceReport {
  task: MaintenanceTask;
  started_at: string;
  completed_at: string;
  items_processed: number;
  items_fixed: number;
  errors: string[];
  success: boolean;
}

export interface MaintenanceSchedule {
  auto_run: boolean;
  interval_hours: number;
  tasks: MaintenanceTask[];
}

export type RenamePattern =
  | { Counter: { prefix: string; start: number; digits: number; suffix: string } }
  | { Date: { format: string; suffix: string } }
  | { Regex: { pattern: string; replacement: string } }
  | { Metadata: { field: string; template: string } }
  | { Custom: { template: string } };

export interface RenamePreview {
  asset_id: string;
  original_name: string;
  new_name: string;
  extension: string;
}

export interface PipelineStage {
  id: number;
  name: string;
  action: StageAction;
  enabled: boolean;
  config: string;
}

export type StageAction =
  | "DetectDuplicates" | "Deduplicate" | "Classify"
  | { ApplyTemplate: { template_id: string | null } }
  | { AddTags: { tags: string[] } }
  | { AddToCollection: { collection_id: string | null } }
  | "GeneratePreviews"
  | { AddToBasket: { basket_id: string | null } }
  | { Export: { format: string; destination: string } }
  | "SkipExisting" | "ValidateSources" | "RemoveOriginalsAfterImport"
  | { Custom: { name: string; config: string } };

export interface Pipeline {
  id: string;
  name: string;
  description: string;
  stages: PipelineStage[];
  status: "Idle" | "Running" | "Completed" | { Failed: string };
  created_at: string;
}

export interface MacroRecording {
  id: string;
  name: string;
  description: string;
  steps: MacroStep[];
  shortcut: string | null;
  created_at: string;
  updated_at: string;
}

export interface MacroStep {
  command_id: string;
  args: string;
  timestamp: string;
}
