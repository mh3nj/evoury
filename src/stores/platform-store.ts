import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

export interface MigrationInfo {
  version: number;
  name: string;
}

export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  enabled: boolean;
}

export interface CrashReport {
  id: string;
  timestamp: string;
  app_version: string;
  os: string;
  error_message: string;
  error_kind: string;
  stack_trace: string | null;
}

export interface SystemInfo {
  os: string;
  os_version: string;
  cpu_count: number;
  total_memory_mb: number;
  free_memory_mb: number;
  app_data_path: string;
  disk_free_mb: number;
}

export interface BackupContents {
  workspace: boolean;
  collections: boolean;
  metadata: boolean;
  baskets: boolean;
  preferences: boolean;
}

interface PlatformState {
  migrations: MigrationInfo[];
  pendingMigrations: number[];
  plugins: PluginManifest[];
  crashReports: CrashReport[];
  systemInfo: SystemInfo | null;
  backups: string[];
  loading: boolean;

  loadMigrations: () => Promise<void>;
  loadPlugins: () => Promise<void>;
  unregisterPlugin: (id: string) => Promise<void>;
  loadCrashReports: () => Promise<void>;
  loadSystemInfo: () => Promise<void>;
  loadBackups: () => Promise<void>;
  createBackup: (name: string, contents: BackupContents) => Promise<string>;
  restoreBackup: (name: string) => Promise<void>;
  exportDiagnostics: () => Promise<string>;
  reportCrash: (message: string, kind: string) => Promise<string>;
}

export const usePlatformStore = create<PlatformState>((set) => ({
  migrations: [],
  pendingMigrations: [],
  plugins: [],
  crashReports: [],
  systemInfo: null,
  backups: [],
  loading: false,

  loadMigrations: async () => {
    try {
      const [migrations, pending] = await Promise.all([
        invoke<MigrationInfo[]>("get_db_migrations"),
        invoke<number[]>("get_db_pending_migrations"),
      ]);
      set({ migrations, pendingMigrations: pending });
    } catch { /* ignore */ }
  },

  loadPlugins: async () => {
    try {
      const plugins = await invoke<PluginManifest[]>("get_plugins");
      set({ plugins });
    } catch { /* ignore */ }
  },

  unregisterPlugin: async (id) => {
    await invoke("unregister_plugin", { pluginId: id });
    set((s) => ({ plugins: s.plugins.filter((p) => p.id !== id) }));
  },

  loadCrashReports: async () => {
    try {
      const crashReports = await invoke<CrashReport[]>("get_crash_reports");
      set({ crashReports });
    } catch { /* ignore */ }
  },

  loadSystemInfo: async () => {
    try {
      const systemInfo = await invoke<SystemInfo>("get_system_info");
      set({ systemInfo });
    } catch { /* ignore */ }
  },

  loadBackups: async () => {
    try {
      const backups = await invoke<string[]>("list_backups");
      set({ backups });
    } catch { /* ignore */ }
  },

  createBackup: async (name, contents) => {
    const path = await invoke<string>("create_backup", {
      name,
      workspace: contents.workspace,
      collections: contents.collections,
      metadata: contents.metadata,
      baskets: contents.baskets,
      preferences: contents.preferences,
    });
    set((s) => ({ backups: [name, ...s.backups] }));
    return path;
  },

  restoreBackup: async (name) => {
    await invoke("restore_backup", { backupName: name });
  },

  exportDiagnostics: async () => {
    return invoke<string>("export_diagnostic_bundle");
  },

  reportCrash: async (message, kind) => {
    return invoke<string>("report_crash", { errorMessage: message, errorKind: kind });
  },
}));
