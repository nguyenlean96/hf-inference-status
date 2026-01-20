use std::collections::HashSet;

use leptos::{logging, prelude::*, task::spawn_local};
use reactive_stores::Store;

use crate::utils::tauri_invoke::tauri_invoke;

#[derive(Clone, Store)]
pub struct FavoriteInferenceServiceState {
    pub show_favorite_only: bool,
    pub favorite_model_inference_ids: HashSet<String>,
}

impl FavoriteInferenceServiceState {
    pub fn new() -> Self {
        Self {
            show_favorite_only: false,
            favorite_model_inference_ids: HashSet::new(),
        }
    }
}

impl Default for FavoriteInferenceServiceState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FavoriteInferenceServiceStateExt {
    fn get_favorite_model_inference_ids(&self);
    fn is_favorite(&self, model_inference_id: &str) -> bool;
    fn add_favorite(&self, model_inference_id: String);
    fn remove_favorite(&self, model_inference_id: String);
}

impl FavoriteInferenceServiceStateExt for Store<FavoriteInferenceServiceState> {
    fn get_favorite_model_inference_ids(&self) {
        let store = *self; // Copy the Store (if it's Copy) or use self.clone()
        spawn_local(async move {
            let ids = tauri_invoke::<Vec<String>>("get_favorite_ids", serde_json::json!({}))
                .await
                .unwrap_or_else(|e| {
                    logging::debug_error!("Failed to get favorite ids: {:?}", e);
                    vec![]
                });
            store.favorite_model_inference_ids().update(|state| {
                *state = ids.into_iter().collect();
            });
        });
    }

    fn is_favorite(&self, model_inference_id: &str) -> bool {
        self.favorite_model_inference_ids()
            .get()
            .contains(model_inference_id)
    }

    fn add_favorite(&self, model_inference_id: String) {
        self.favorite_model_inference_ids().update(|state| {
            state.insert(model_inference_id.clone());
        });
        spawn_local(async move {
            tauri_invoke::<()>(
                "add_favorite",
                serde_json::json!({ "modelInferenceId": model_inference_id }),
            )
            .await
            .unwrap_or_else(|e| {
                logging::debug_error!("Failed to add favorite: {:?}", e);
            });
        });
    }

    fn remove_favorite(&self, model_inference_id: String) {
        self.favorite_model_inference_ids().update(|state| {
            state.remove(&model_inference_id);
        });
        spawn_local(async move {
            tauri_invoke::<()>(
                "remove_favorite",
                serde_json::json!({ "modelInferenceId": model_inference_id }),
            )
            .await
            .unwrap_or_else(|e| {
                logging::debug_error!("Failed to remove favorite: {:?}", e);
            });
        });
    }
}
