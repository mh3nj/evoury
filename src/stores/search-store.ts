import { create } from "zustand";
import type { SearchResult, ParsedFilter } from "../types";

interface SearchState {
  query: string;
  results: SearchResult[];
  active: boolean;
  suggestions: string[];
  filters: ParsedFilter[];
  setQuery: (query: string) => void;
  setResults: (results: SearchResult[]) => void;
  setActive: (active: boolean) => void;
  setSuggestions: (suggestions: string[]) => void;
  setFilters: (filters: ParsedFilter[]) => void;
  clear: () => void;
}

export const useSearchStore = create<SearchState>((set) => ({
  query: "",
  results: [],
  active: false,
  suggestions: [],
  filters: [],
  setQuery: (query) => set({ query }),
  setResults: (results) => set({ results, active: results.length > 0 }),
  setActive: (active) => set({ active }),
  setSuggestions: (suggestions) => set({ suggestions }),
  setFilters: (filters) => set({ filters }),
  clear: () => set({ query: "", results: [], active: false, suggestions: [], filters: [] }),
}));
