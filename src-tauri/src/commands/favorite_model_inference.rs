use polars::prelude::*;
use std::collections::HashMap;
use tauri::{AppHandle, Manager};

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;

use crate::modules::favorite_model_inference::service::FavoriteModelService;
use crate::modules::inference_models::prelude::*;
use crate::states::inference_model_state::Queryable;
use crate::states::prelude::*;
use crate::types::prelude::{FilterColumn, SortOrder, TableColumn};

// Take model inference (mi) id
#[tauri::command]
pub async fn add_favorite(app: AppHandle, model_inference_id: String) -> Result<(), String> {
    let inference_model_state = app.state::<InferenceModelState>();
    let user_favorite_state = app.state::<UserFavoriteState>();
    let inference_model_state_lock = inference_model_state.lock().await;
    let mut user_favorite_state_lock = user_favorite_state.lock().await;

    let Some(model) =
        inference_model_state_lock.get_model_inference_service_by_id(&model_inference_id)
    else {
        return Err("Model not found".to_string());
    };
    user_favorite_state_lock.add_favorite(&model_inference_id);
    let _ = FavoriteModelService::add_favorite(&app, model).map_err(|e| e.to_string());

    Ok(())
}

#[tauri::command]
pub async fn remove_favorite(app: AppHandle, model_inference_id: String) -> Result<(), String> {
    let state = app.state::<UserFavoriteState>();
    let mut state_lock = state.lock().await;

    println!("Removing favorite for id: {}", model_inference_id);
    state_lock.remove_favorite(&model_inference_id);
    let _ =
        FavoriteModelService::remove_favorite(&app, model_inference_id).map_err(|e| e.to_string());

    Ok(())
}

#[tauri::command]
pub async fn is_favorite(app: AppHandle, id: String) -> Result<bool, String> {
    let state = app.state::<UserFavoriteState>();
    let state_lock = state.lock().await;
    Ok(state_lock.fav_ids.contains(&id))
}

#[tauri::command]
pub async fn get_favorite_ids(app: AppHandle) -> Result<Vec<String>, String> {
    let state = app.state::<UserFavoriteState>();
    let state_lock = state.lock().await;
    Ok(state_lock.fav_ids.clone())
}

#[tauri::command]
pub async fn get_favorite_model_inference_data(
    app: AppHandle,
    mut filtered_by: Vec<FilterColumn>,
    sorted_by: Vec<(TableColumn, SortOrder)>,
) -> Result<Vec<HFModelInferenceStatusRowData>, String> {
    let inference_model_state = app.state::<InferenceModelState>();
    let user_favorite_state = app.state::<UserFavoriteState>();
    let mut inference_model_state_lock = inference_model_state.lock().await;
    let user_favorite_state_lock = user_favorite_state.lock().await;

    // Check if df is empty
    // NOTE:
    // - DataFrame `is_empty()` method is deprecated. Use `shape()` to
    //   check whether the DF size is (0, 0):
    //   (0 height (rows), 0 width (columns)) instead
    if inference_model_state_lock
        .data
        .as_ref()
        .is_none_or(|d| d.shape() == (0, 0))
    {
        inference_model_state_lock.update().await;
    }

    for id in user_favorite_state_lock.fav_ids.clone() {
        filtered_by.push(FilterColumn::Id(id));
    }

    inference_model_state_lock.query(&filtered_by, &sorted_by)
}
