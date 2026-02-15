use polars::prelude::*;
use std::collections::HashMap;
use tauri::State;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::modules::inference_models::prelude::*;
use crate::states::prelude::InferenceModelState;
use crate::types::prelude::{SortOrder, TableColumn};

#[tauri::command]
pub async fn get_data(
    state: State<'_, InferenceModelState>,
    sorted_by: HashMap<TableColumn, SortOrder>,
) -> Result<Vec<HFModelInferenceStatusRowData>, String> {
    let mut state_lock = state.lock().await;
    state_lock.update().await;

    if let Some(df) = &state_lock.data {
        // Sort the following columns:
        // - model_family (asc)
        // - short_name (asc)
        // - provider_name (asc)
        // by default
        let sorted_df = df
            .sort(
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
            )
            .map_err(|e| e.to_string())?;

        let response: InferenceModelStatusCollection =
            InferenceModelStatusCollection::from(&sorted_df);
        Ok(response.data)
    } else {
        Ok(vec![])
    }
}
