use polars::prelude::*;
use std::collections::HashMap;
use tauri::{AppHandle, Manager};

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;

use crate::modules::favorite_model_inference::service::FavoriteModelService;
use crate::modules::inference_models::prelude::*;
use crate::states::prelude::*;
use crate::types::prelude::{FilterColumn, SortOrder, TableColumn};

// Take model inference (mi) id
#[tauri::command]
pub async fn add_favorite(app: AppHandle, model_inference_id: String) -> Result<(), String> {
    let inference_model_state = app.state::<InferenceModelState>();
    let user_favorite_state = app.state::<UserFavoriteState>();
    let inference_model_state_lock = inference_model_state.lock().await;
    let mut user_favorite_state_lock = user_favorite_state.lock().await;

    println!("Adding favorite for id: {}", model_inference_id);
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
    filtered_by: Vec<FilterColumn>,
    sorted_by: HashMap<TableColumn, SortOrder>,
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

    let df = inference_model_state_lock
        .data
        .as_ref()
        .ok_or("Data not found")?;

    let targets = Series::new("targets".into(), user_favorite_state_lock.fav_ids.clone());

    let mut lf = df.clone().lazy();
    lf = lf.filter(col("id").is_in(lit(targets).implode(), true));

    let mut filter_providers: Vec<String> = Vec::new();
    // 1. Apply filters first (Performance: Reduces dataset size before sorting)
    for filter_col in filtered_by {
        let predicate = match filter_col {
            FilterColumn::ToolsSupport(enabled) => {
                Some(col(TableColumn::ToolsSupport.as_str()).eq(lit(enabled)))
            }
            FilterColumn::StructuredOutputSupport(enabled) => {
                Some(col(TableColumn::StructuredOutputSupport.as_str()).eq(lit(enabled)))
            }
            FilterColumn::ProviderName(provider) => {
                filter_providers.push(provider);
                None
            }
            _ => None,
        };
        if let Some(p) = predicate {
            lf = lf.filter(p);
        }
    }

    if !filter_providers.is_empty() {
        let targets = Series::new("targets".into(), filter_providers);
        lf = lf.filter(col(TableColumn::ProviderName.as_str()).is_in(lit(targets).implode(), true));
    }

    // 2. Apply Sorting
    lf = lf.sort(
        if !sorted_by.is_empty() {
            sorted_by.keys().map(|k| k.as_str()).collect()
        } else {
            vec![
                TableColumn::ModelFamily.as_str(),
                TableColumn::ShortName.as_str(),
                TableColumn::ProviderName.as_str(),
            ]
        },
        SortMultipleOptions::new()
            .with_order_descending_multi(if !sorted_by.is_empty() {
                sorted_by
                    .values()
                    .map(|v| matches!(v, SortOrder::Descending))
                    .collect()
            } else {
                vec![false, false, false]
            })
            .with_nulls_last(true)
            .with_maintain_order(true),
    );
    let result_df = lf.collect().map_err(|e| e.to_string())?;

    let response = InferenceModelStatusCollection::from(&result_df);
    Ok(response.data)
}
