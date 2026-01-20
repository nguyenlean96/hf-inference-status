use polars::prelude::*;
use tauri::State;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::modules::inference_models::prelude::*;
use crate::states::prelude::InferenceModelState;

#[tauri::command]
pub async fn get_data(
    state: State<'_, InferenceModelState>,
) -> Result<Vec<HFModelInferenceStatusRowData>, String> {
    let state_lock = state.lock().await;
    if let Some(df) = &state_lock.data {
        // Sort the following columns:
        // - model_family (asc)
        // - short_name (asc)
        // - provider_name (asc)
        // by default
        let sorted_df = df
            .sort(
                ["model_family", "short_name", "provider_name"],
                SortMultipleOptions::new().with_order_descending_multi([false, false, false]),
            )
            .map_err(|e| e.to_string())?;

        let response: InferenceModelStatusCollection =
            InferenceModelStatusCollection::from(&sorted_df);
        Ok(response.data)
    } else {
        Ok(vec![])
    }
}
