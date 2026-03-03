use tauri::State;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::states::inference_model_state::Queryable;
use crate::states::prelude::InferenceModelState;
use crate::types::prelude::{FilterColumn, SortOrder, TableColumn};

#[tauri::command]
pub async fn get_data(
    state: State<'_, InferenceModelState>,
    filtered_by: Vec<FilterColumn>,
    sorted_by: Vec<(TableColumn, SortOrder)>,
) -> Result<Vec<HFModelInferenceStatusRowData>, String> {
    let mut state_lock = state.lock().await;
    state_lock.update().await;

    state_lock.query(&filtered_by, &sorted_by)
}
