import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { MetadataTemplate } from "../types";

interface TemplatesState {
  templates: MetadataTemplate[];
  loading: boolean;
  loadTemplates: () => Promise<void>;
  addTemplate: (template: MetadataTemplate) => Promise<void>;
  removeTemplate: (id: string) => Promise<void>;
}

export const useTemplatesStore = create<TemplatesState>((set) => ({
  templates: [],
  loading: false,
  loadTemplates: async () => {
    set({ loading: true });
    try {
      const templates = await invoke<MetadataTemplate[]>("get_templates");
      set({ templates, loading: false });
    } catch { set({ loading: false }); }
  },
  addTemplate: async (template) => {
    await invoke("add_template", { template });
    const templates = await invoke<MetadataTemplate[]>("get_templates");
    set({ templates });
  },
  removeTemplate: async (id) => {
    await invoke("remove_template", { templateId: id });
    set((s) => ({ templates: s.templates.filter((t) => t.id !== id) }));
  },
}));
