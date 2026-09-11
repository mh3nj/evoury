import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";
import type { Relationship } from "../types";

interface RelationshipsState {
  relationships: Relationship[];
  loading: boolean;
  addRelationship: (rel: Relationship) => Promise<void>;
  removeRelationship: (id: string) => Promise<void>;
  getForAsset: (assetId: string) => Promise<Relationship[]>;
  getRelatedAssets: (assetId: string) => Promise<string[]>;
  getDependencyChain: (assetId: string) => Promise<string[]>;
  getDependentChain: (assetId: string) => Promise<string[]>;
}

export const useRelationshipsStore = create<RelationshipsState>(() => ({
  relationships: [],
  loading: false,
  addRelationship: async (rel) => {
    await invoke("add_relationship", { relationship: rel });
  },
  removeRelationship: async (id) => {
    await invoke("remove_relationship", { relationshipId: id });
  },
  getForAsset: async (assetId) => {
    return invoke<Relationship[]>("get_relationships_for_asset", { assetId });
  },
  getRelatedAssets: async (assetId) => {
    return invoke<string[]>("get_related_assets", { assetId });
  },
  getDependencyChain: async (assetId) => {
    return invoke<string[]>("get_dependency_chain", { assetId });
  },
  getDependentChain: async (assetId) => {
    return invoke<string[]>("get_dependent_chain", { assetId });
  },
}));
