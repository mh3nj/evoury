use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
};
use atlas_basket::Basket;
use atlas_metadata::AssetMetadata;
use atlas_preferences::Preferences;
use uuid::Uuid;

const METADATA_FILE: &str = "metadata.json";
const PREFERENCES_FILE: &str = "preferences.json";
const BASKETS_FILE: &str = "baskets.json";

pub struct Persistence {
    data_dir: PathBuf,
}

impl Persistence {
    pub fn new(app: &tauri::AppHandle) -> Self {
        let data_dir = tauri::api::path::app_data_dir(&app.config())
            .unwrap_or_else(|| PathBuf::from("."));
        fs::create_dir_all(&data_dir).ok();
        Self { data_dir }
    }

    pub fn load_metadata(&self) -> HashMap<Uuid, AssetMetadata> {
        let path = self.data_dir.join(METADATA_FILE);
        let raw: HashMap<String, AssetMetadata> = match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };
        raw.into_iter()
            .filter_map(|(k, v)| Uuid::parse_str(&k).ok().map(|id| (id, v)))
            .collect()
    }

    pub fn save_metadata(&self, metadata: &HashMap<Uuid, AssetMetadata>) -> Result<(), String> {
        let raw: HashMap<String, &AssetMetadata> = metadata
            .iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let content = serde_json::to_string_pretty(&raw).map_err(|e| e.to_string())?;
        fs::write(self.data_dir.join(METADATA_FILE), content).map_err(|e| e.to_string())
    }

    pub fn load_preferences(&self) -> Preferences {
        let path = self.data_dir.join(PREFERENCES_FILE);
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Preferences::default(),
        }
    }

    pub fn save_preferences(&self, prefs: &Preferences) -> Result<(), String> {
        let content = serde_json::to_string_pretty(prefs).map_err(|e| e.to_string())?;
        fs::write(self.data_dir.join(PREFERENCES_FILE), content).map_err(|e| e.to_string())
    }

    pub fn load_baskets(&self) -> Vec<Basket> {
        let path = self.data_dir.join(BASKETS_FILE);
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    }

    pub fn save_baskets(&self, baskets: &[Basket]) -> Result<(), String> {
        let content = serde_json::to_string_pretty(baskets).map_err(|e| e.to_string())?;
        fs::write(self.data_dir.join(BASKETS_FILE), content).map_err(|e| e.to_string())
    }
}
