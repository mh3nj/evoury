import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { Pipeline, PipelineStage } from "../types";

interface PipelineState {
  pipelines: Pipeline[];
  loading: boolean;
  load: () => Promise<void>;
  create: (name: string, description: string) => Promise<Pipeline>;
  remove: (id: string) => Promise<void>;
  addStage: (pipelineId: string, stage: PipelineStage) => Promise<void>;
}

export const usePipelineStore = create<PipelineState>((set) => ({
  pipelines: [],
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const pipelines = await invoke<Pipeline[]>("get_pipelines");
      set({ pipelines, loading: false });
    } catch { set({ loading: false }); }
  },
  create: async (name, description) => {
    const pipeline = await invoke<Pipeline>("create_pipeline", { name, description });
    set((s) => ({ pipelines: [...s.pipelines, pipeline] }));
    return pipeline;
  },
  remove: async (id) => {
    await invoke("remove_pipeline", { pipelineId: id });
    set((s) => ({ pipelines: s.pipelines.filter((p) => p.id !== id) }));
  },
  addStage: async (pipelineId, stage) => {
    await invoke("add_pipeline_stage", { pipelineId, stage });
  },
}));
