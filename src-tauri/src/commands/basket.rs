use tauri::State;
use atlas_basket::Basket;
use crate::state::AppState;

fn save_baskets(state: &AppState) {
    if let Ok(manager) = state.basket_manager.lock() {
        if let Ok(persistence) = state.persistence.lock() {
            let _ = persistence.save_baskets(&manager.all());
        }
    }
}

#[tauri::command]
pub fn create_basket(state: State<'_, AppState>, name: String) -> Result<Basket, String> {
    let basket = Basket {
        id: uuid::Uuid::new_v4(),
        name,
        assets: Vec::new(),
        created: chrono::Utc::now(),
    };
    let mut manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    let b = basket.clone();
    manager.create(basket);
    drop(manager);
    save_baskets(&state);
    Ok(b)
}

#[tauri::command]
pub fn get_baskets(state: State<'_, AppState>) -> Result<Vec<Basket>, String> {
    let manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.all())
}

#[tauri::command]
pub fn add_to_basket(state: State<'_, AppState>, basket_id: String, asset_id: String) -> Result<bool, String> {
    let bid = uuid::Uuid::parse_str(&basket_id).map_err(|e| e.to_string())?;
    let aid = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let mut manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    let result = manager.add_asset(&bid, aid);
    drop(manager);
    save_baskets(&state);
    Ok(result)
}

#[tauri::command]
pub fn remove_from_basket(state: State<'_, AppState>, basket_id: String, asset_id: String) -> Result<bool, String> {
    let bid = uuid::Uuid::parse_str(&basket_id).map_err(|e| e.to_string())?;
    let aid = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let mut manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    let result = manager.remove_asset(&bid, &aid);
    drop(manager);
    save_baskets(&state);
    Ok(result)
}

#[tauri::command]
pub fn clear_basket(state: State<'_, AppState>, basket_id: String) -> Result<(), String> {
    let bid = uuid::Uuid::parse_str(&basket_id).map_err(|e| e.to_string())?;
    let mut manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    manager.clear(&bid);
    drop(manager);
    save_baskets(&state);
    Ok(())
}

#[tauri::command]
pub fn delete_basket(state: State<'_, AppState>, basket_id: String) -> Result<(), String> {
    let bid = uuid::Uuid::parse_str(&basket_id).map_err(|e| e.to_string())?;
    let mut manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    manager.remove(&bid);
    drop(manager);
    save_baskets(&state);
    Ok(())
}

#[tauri::command]
pub fn is_in_basket(state: State<'_, AppState>, basket_id: String, asset_id: String) -> Result<bool, String> {
    let bid = uuid::Uuid::parse_str(&basket_id).map_err(|e| e.to_string())?;
    let aid = uuid::Uuid::parse_str(&asset_id).map_err(|e| e.to_string())?;
    let manager = state.basket_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.contains(&bid, &aid))
}
