import { create } from "zustand";
import { invoke } from "@tauri-apps/api/tauri";

export interface BasketAsset {
  assetId: string;
  name: string;
  archivePath: string;
  previewPath: string | null;
  tempName: string;
}

export interface Basket {
  id: string;
  name: string;
  assets: BasketAsset[];
  created: string;
}

interface BasketState {
  baskets: Basket[];
  activeBasketId: string | null;
  loading: boolean;
  loadBaskets: () => Promise<void>;
  createBasket: (name: string) => Promise<Basket>;
  renameBasket: (id: string, name: string) => void;
  setActiveBasket: (id: string | null) => void;
  addAsset: (asset: { id: string; name: string; archive_path: string; preview?: { path: string } | null }) => Promise<void>;
  addToBasket: (basketId: string, asset: { id: string; name: string; archive_path: string; preview?: { path: string } | null }) => void;
  removeAsset: (basketId: string, assetId: string) => void;
  moveAsset: (fromBasketId: string, toBasketId: string, assetId: string) => void;
  setTempName: (basketId: string, assetId: string, tempName: string) => void;
  clearBasket: (basketId: string) => void;
  deleteBasket: (id: string) => void;
  getBasket: (id: string) => Basket | undefined;
}

function persist(baskets: Basket[]) {
  localStorage.setItem("evoury_baskets", JSON.stringify(baskets));
}

function loadPersisted(): Basket[] {
  try {
    const raw = localStorage.getItem("evoury_baskets");
    return raw ? JSON.parse(raw) : [];
  } catch { return []; }
}

export const useBasketStore = create<BasketState>((set, get) => ({
  baskets: loadPersisted(),
  activeBasketId: null,
  loading: false,

  loadBaskets: async () => {
    set({ loading: true });
    const persisted = loadPersisted();
    if (persisted.length > 0) {
      set({ baskets: persisted, loading: false });
      if (!get().activeBasketId) set({ activeBasketId: persisted[0].id });
      return;
    }
    try {
      const baskets = await invoke<Basket[]>("get_baskets");
      set({ baskets, loading: false });
      if (baskets.length > 0 && !get().activeBasketId) {
        set({ activeBasketId: baskets[0].id });
      }
      persist(baskets);
    } catch { set({ loading: false }); }
  },

  createBasket: async (name) => {
    const basket: Basket = { id: crypto.randomUUID(), name, assets: [], created: new Date().toISOString() };
    const baskets = [basket, ...get().baskets];
    set({ baskets, activeBasketId: basket.id });
    persist(baskets);
    try { await invoke("create_basket", { name }); } catch {}
    return basket;
  },

  renameBasket: (id, name) => {
    const baskets = get().baskets.map((b) => b.id === id ? { ...b, name } : b);
    set({ baskets });
    persist(baskets);
  },

  setActiveBasket: (id) => set({ activeBasketId: id }),

  addAsset: async (asset) => {
    const { activeBasketId, baskets } = get();
    if (!activeBasketId) return;
    const ba: BasketAsset = {
      assetId: asset.id,
      name: asset.name,
      archivePath: asset.archive_path,
      previewPath: asset.preview?.path || null,
      tempName: asset.name,
    };
    const updated = baskets.map((b) =>
      b.id === activeBasketId && !b.assets.find((a) => a.assetId === asset.id)
        ? { ...b, assets: [...b.assets, ba] }
        : b
    );
    set({ baskets: updated });
    persist(updated);
    try { await invoke("add_to_basket", { basketId: activeBasketId, assetId: asset.id }); } catch {}
  },

  addToBasket: (basketId, asset) => {
    const { baskets } = get();
    const ba: BasketAsset = {
      assetId: asset.id,
      name: asset.name,
      archivePath: asset.archive_path,
      previewPath: asset.preview?.path || null,
      tempName: asset.name,
    };
    const updated = baskets.map((b) =>
      b.id === basketId && !b.assets.find((a) => a.assetId === asset.id)
        ? { ...b, assets: [...b.assets, ba] }
        : b
    );
    set({ baskets: updated });
    persist(updated);
  },

  removeAsset: (basketId, assetId) => {
    const baskets = get().baskets.map((b) =>
      b.id === basketId ? { ...b, assets: b.assets.filter((a) => a.assetId !== assetId) } : b
    );
    set({ baskets });
    persist(baskets);
    invoke("remove_from_basket", { basketId, assetId }).catch(() => {});
  },

  moveAsset: (fromBasketId, toBasketId, assetId) => {
    const { baskets } = get();
    const fromBasket = baskets.find((b) => b.id === fromBasketId);
    if (!fromBasket) return;
    const asset = fromBasket.assets.find((a) => a.assetId === assetId);
    if (!asset) return;
    const updated = baskets.map((b) => {
      if (b.id === fromBasketId) return { ...b, assets: b.assets.filter((a) => a.assetId !== assetId) };
      if (b.id === toBasketId && !b.assets.find((a) => a.assetId === assetId)) return { ...b, assets: [...b.assets, asset] };
      return b;
    });
    set({ baskets: updated });
    persist(updated);
  },

  setTempName: (basketId, assetId, tempName) => {
    const baskets = get().baskets.map((b) =>
      b.id === basketId
        ? { ...b, assets: b.assets.map((a) => a.assetId === assetId ? { ...a, tempName } : a) }
        : b
    );
    set({ baskets });
    persist(baskets);
  },

  clearBasket: (basketId) => {
    const baskets = get().baskets.map((b) => b.id === basketId ? { ...b, assets: [] } : b);
    set({ baskets });
    persist(baskets);
    invoke("clear_basket", { basketId }).catch(() => {});
  },

  deleteBasket: (id) => {
    const baskets = get().baskets.filter((b) => b.id !== id);
    set({
      baskets,
      activeBasketId: get().activeBasketId === id
        ? (baskets.length > 0 ? baskets[0].id : null)
        : get().activeBasketId,
    });
    persist(baskets);
    invoke("delete_basket", { basketId: id }).catch(() => {});
  },

  getBasket: (id) => get().baskets.find((b) => b.id === id),
}));
