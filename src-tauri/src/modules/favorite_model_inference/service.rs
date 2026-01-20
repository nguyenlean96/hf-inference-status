use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;

const STORE_PATH: &str = "favorite_models.json";

/// Service to handle favorite model logic and persistence.
/// It acts as an abstraction layer over the tauri-plugin-store.
pub struct FavoriteModelService;

impl FavoriteModelService {
    /// Save a model as a favorite.
    /// This creates a snapshot of the current state.
    pub fn add_favorite(
        app: &AppHandle,
        model: HFModelInferenceStatusRowData,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let store = app.store(STORE_PATH)?;

        store.set(model.id.clone(), serde_json::to_value(model.clone())?);
        store.save()?;

        Ok(model.id.clone())
    }

    /// Remove a model from favorites by its ID.
    pub fn remove_favorite(app: &AppHandle, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let store = app.store(STORE_PATH)?;
        store.delete(id);
        store.save()?;
        Ok(())
    }

    /// Get all favorite models with their computed hashes.
    pub fn get_favorites(
        app: &AppHandle,
    ) -> Result<Vec<HFModelInferenceStatusRowData>, Box<dyn std::error::Error>> {
        let store = app.store(STORE_PATH)?;

        let mut favorites = Vec::new();

        for key in store.keys() {
            if let Some(value) = store.get(&key)
                && let Ok(snapshot) = serde_json::from_value::<HFModelInferenceStatusRowData>(value)
            {
                favorites.push(snapshot);
            }
        }

        Ok(favorites)
    }
}
