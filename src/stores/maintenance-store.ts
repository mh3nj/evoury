import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { MaintenanceReport, MaintenanceSchedule } from "../types";

interface MaintenanceState {
  reports: MaintenanceReport[];
  schedule: MaintenanceSchedule | null;
  running: boolean;
  loadReports: () => Promise<void>;
  loadSchedule: () => Promise<void>;
  runTask: (taskName: string) => Promise<MaintenanceReport>;
  runAll: () => Promise<MaintenanceReport[]>;
  setSchedule: (schedule: MaintenanceSchedule) => Promise<void>;
}

export const useMaintenanceStore = create<MaintenanceState>((set) => ({
  reports: [],
  schedule: null,
  running: false,
  loadReports: async () => {
    try {
      const reports = await invoke<MaintenanceReport[]>("get_maintenance_reports");
      set({ reports });
    } catch { /* ignore */ }
  },
  loadSchedule: async () => {
    try {
      const schedule = await invoke<MaintenanceSchedule>("get_maintenance_schedule");
      set({ schedule });
    } catch { /* ignore */ }
  },
  runTask: async (taskName) => {
    set({ running: true });
    const report = await invoke<MaintenanceReport>("run_maintenance", { taskName });
    set((s) => ({ reports: [...s.reports, report], running: false }));
    return report;
  },
  runAll: async () => {
    set({ running: true });
    const reports = await invoke<MaintenanceReport[]>("run_all_maintenance");
    set((s) => ({ reports: [...s.reports, ...reports], running: false }));
    return reports;
  },
  setSchedule: async (schedule) => {
    await invoke("set_maintenance_schedule", { schedule });
    set({ schedule });
  },
}));
