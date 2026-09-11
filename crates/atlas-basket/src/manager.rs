use std::collections::HashMap;
use uuid::Uuid;
use crate::basket::Basket;

pub struct BasketManager {
    baskets: HashMap<Uuid, Basket>,
}

impl BasketManager {
    pub fn new() -> Self {
        Self {
            baskets: HashMap::new(),
        }
    }

    pub fn create(&mut self, basket: Basket) {
        self.baskets.insert(basket.id, basket);
    }

    pub fn get(&self, id: &Uuid) -> Option<&Basket> {
        self.baskets.get(id)
    }

    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Basket> {
        self.baskets.get_mut(id)
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.baskets.remove(id);
    }

    pub fn all(&self) -> Vec<Basket> {
        let mut baskets: Vec<Basket> = self.baskets.values().cloned().collect();
        baskets.sort_by(|a, b| b.created.cmp(&a.created));
        baskets
    }

    pub fn add_asset(&mut self, basket_id: &Uuid, asset_id: Uuid) -> bool {
        if let Some(basket) = self.baskets.get_mut(basket_id) {
            if !basket.assets.contains(&asset_id) {
                basket.assets.push(asset_id);
                return true;
            }
        }
        false
    }

    pub fn remove_asset(&mut self, basket_id: &Uuid, asset_id: &Uuid) -> bool {
        if let Some(basket) = self.baskets.get_mut(basket_id) {
            let len = basket.assets.len();
            basket.assets.retain(|a| a != asset_id);
            return basket.assets.len() < len;
        }
        false
    }

    pub fn clear(&mut self, basket_id: &Uuid) {
        if let Some(basket) = self.baskets.get_mut(basket_id) {
            basket.assets.clear();
        }
    }

    pub fn contains(&self, basket_id: &Uuid, asset_id: &Uuid) -> bool {
        self.baskets
            .get(basket_id)
            .map(|b| b.assets.contains(asset_id))
            .unwrap_or(false)
    }
}
