import { create } from "zustand";

export interface AnimationPreset {
  id: string;
  name: string;
  durationMs: number;
  easing: string;
  delayMs: number;
  property: string;
}

interface AnimationState {
  enabled: boolean;
  reducedMotion: boolean;
  speedMultiplier: number;
  presets: AnimationPreset[];
  setReducedMotion: (reduced: boolean) => void;
  setEnabled: (enabled: boolean) => void;
  setSpeedMultiplier: (mult: number) => void;
  getDuration: (presetId: string) => number;
  shouldAnimate: () => boolean;
  registerPreset: (preset: AnimationPreset) => void;
}

export const useAnimationStore = create<AnimationState>((set, get) => ({
  enabled: true,
  reducedMotion: false,
  speedMultiplier: 1.0,
  presets: [
    { id: "fade-in", name: "Fade In", durationMs: 200, easing: "cubic-bezier(0.215, 0.61, 0.355, 1)", delayMs: 0, property: "opacity" },
    { id: "slide-up", name: "Slide Up", durationMs: 300, easing: "cubic-bezier(0.25, 0.46, 0.45, 0.94)", delayMs: 0, property: "transform" },
    { id: "scale-in", name: "Scale In", durationMs: 200, easing: "cubic-bezier(0.215, 0.61, 0.355, 1)", delayMs: 0, property: "transform" },
    { id: "quick", name: "Quick", durationMs: 100, easing: "cubic-bezier(0.25, 0.46, 0.45, 0.94)", delayMs: 0, property: "all" },
    { id: "spring", name: "Spring", durationMs: 400, easing: "cubic-bezier(0.34, 1.56, 0.64, 1)", delayMs: 0, property: "transform" },
  ],

  setReducedMotion: (reduced) =>
    set({
      reducedMotion: reduced,
      speedMultiplier: reduced ? 0.3 : 1.0,
    }),

  setEnabled: (enabled) => set({ enabled }),
  setSpeedMultiplier: (speedMultiplier) => set({ speedMultiplier }),

  getDuration: (presetId) => {
    const { presets, speedMultiplier } = get();
    const preset = presets.find((p) => p.id === presetId);
    return (preset?.durationMs ?? 200) * speedMultiplier;
  },

  shouldAnimate: () => {
    const { enabled, reducedMotion } = get();
    return enabled && !reducedMotion;
  },

  registerPreset: (preset) =>
    set((s) => ({ presets: [...s.presets.filter((p) => p.id !== preset.id), preset] })),
}));
