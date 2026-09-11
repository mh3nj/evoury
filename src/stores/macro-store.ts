import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { MacroRecording } from "../types";

interface MacroState {
  macros: MacroRecording[];
  isRecording: boolean;
  loading: boolean;
  load: () => Promise<void>;
  startRecording: (name: string) => Promise<void>;
  stopRecording: () => Promise<MacroRecording>;
  recordStep: (commandId: string, args: string) => Promise<void>;
  removeMacro: (id: string) => Promise<void>;
}

export const useMacroStore = create<MacroState>((set) => ({
  macros: [],
  isRecording: false,
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const [macros, isRecording] = await Promise.all([
        invoke<MacroRecording[]>("get_macros"),
        invoke<boolean>("is_recording"),
      ]);
      set({ macros, isRecording, loading: false });
    } catch { set({ loading: false }); }
  },
  startRecording: async (name) => {
    await invoke("start_macro_recording", { name });
    set({ isRecording: true });
  },
  stopRecording: async () => {
    const recording = await invoke<MacroRecording>("stop_macro_recording");
    set((s) => ({ isRecording: false, macros: [...s.macros, recording] }));
    return recording;
  },
  recordStep: async (commandId, args) => {
    await invoke("record_macro_step", { commandId, args });
  },
  removeMacro: async (id) => {
    await invoke("remove_macro", { macroId: id });
    set((s) => ({ macros: s.macros.filter((m) => m.id !== id) }));
  },
}));
