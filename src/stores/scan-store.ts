import { create } from "zustand";

interface ScanState {
  scanning: boolean;
  scanId: string | null;
  totalFiles: number;
  scannedFiles: number;
  archivesFound: number;
  previewsFound: number;
  currentFile: string | null;
  errors: string[];

  startScan: (scanId: string) => void;
  updateProgress: (total: number, scanned: number, archives: number, previews: number, file: string | null) => void;
  finishScan: () => void;
  setError: (error: string) => void;
  reset: () => void;
}

export const useScanStore = create<ScanState>((set) => ({
  scanning: false,
  scanId: null,
  totalFiles: 0,
  scannedFiles: 0,
  archivesFound: 0,
  previewsFound: 0,
  currentFile: null,
  errors: [],

  startScan: (scanId) =>
    set({
      scanning: true,
      scanId,
      totalFiles: 0,
      scannedFiles: 0,
      archivesFound: 0,
      previewsFound: 0,
      currentFile: null,
      errors: [],
    }),

  updateProgress: (total, scanned, archives, previews, file) =>
    set({
      totalFiles: total,
      scannedFiles: scanned,
      archivesFound: archives,
      previewsFound: previews,
      currentFile: file,
    }),

  finishScan: () =>
    set({ scanning: false, currentFile: null }),

  setError: (error) =>
    set((s) => ({ errors: [...s.errors, error] })),

  reset: () =>
    set({
      scanning: false,
      scanId: null,
      totalFiles: 0,
      scannedFiles: 0,
      archivesFound: 0,
      previewsFound: 0,
      currentFile: null,
      errors: [],
    }),
}));
