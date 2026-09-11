import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { Project } from "../types";

interface ProjectsState {
  projects: Project[];
  activeProjectId: string | null;
  loading: boolean;
  loadProjects: () => Promise<void>;
  createProject: (name: string) => Promise<Project>;
  getProject: (id: string) => Promise<Project | null>;
  removeProject: (id: string) => Promise<void>;
  addAsset: (projectId: string, assetId: string) => Promise<void>;
  removeAsset: (projectId: string, assetId: string) => Promise<void>;
  getProjectAssets: (projectId: string) => Promise<string[]>;
  setActiveProject: (id: string | null) => void;
}

export const useProjectsStore = create<ProjectsState>((set) => ({
  projects: [],
  activeProjectId: null,
  loading: false,
  loadProjects: async () => {
    set({ loading: true });
    try {
      const projects = await invoke<Project[]>("get_all_projects");
      set({ projects, loading: false });
    } catch { set({ loading: false }); }
  },
  createProject: async (name) => {
    const project = await invoke<Project>("create_project", { name });
    set((s) => ({ projects: [...s.projects, project] }));
    return project;
  },
  getProject: async (id) => {
    return invoke<Project | null>("get_project", { projectId: id });
  },
  removeProject: async (id) => {
    await invoke("remove_project", { projectId: id });
    set((s) => ({ projects: s.projects.filter((p) => p.id !== id) }));
  },
  addAsset: async (projectId, assetId) => {
    await invoke("add_asset_to_project", { projectId, assetId });
  },
  removeAsset: async (projectId, assetId) => {
    await invoke("remove_asset_from_project", { projectId, assetId });
  },
  getProjectAssets: async (projectId) => {
    return invoke<string[]>("get_project_assets", { projectId });
  },
  setActiveProject: (id) => set({ activeProjectId: id }),
}));
