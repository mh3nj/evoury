import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { AutoClassifyRule } from "../types";

interface ClassifyState {
  rules: AutoClassifyRule[];
  loading: boolean;
  loadRules: () => Promise<void>;
  addRule: (rule: AutoClassifyRule) => Promise<void>;
  removeRule: (id: string) => Promise<void>;
  classifyAsset: (name: string, mimeType?: string, extension?: string) => Promise<AutoClassifyRule[]>;
}

export const useClassifyStore = create<ClassifyState>((set) => ({
  rules: [],
  loading: false,
  loadRules: async () => {
    set({ loading: true });
    try {
      const rules = await invoke<AutoClassifyRule[]>("get_classify_rules");
      set({ rules, loading: false });
    } catch { set({ loading: false }); }
  },
  addRule: async (rule) => {
    await invoke("add_classify_rule", { rule });
    const rules = await invoke<AutoClassifyRule[]>("get_classify_rules");
    set({ rules });
  },
  removeRule: async (id) => {
    await invoke("remove_classify_rule", { ruleId: id });
    set((s) => ({ rules: s.rules.filter((r) => r.id !== id) }));
  },
  classifyAsset: async (name, mimeType, extension) => {
    return invoke<AutoClassifyRule[]>("classify_asset", { name, mimeType, extension });
  },
}));
