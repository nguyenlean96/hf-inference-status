use polars::prelude::*;
use std::collections::HashMap;
use tauri::State;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::modules::inference_models::prelude::*;
use crate::states::prelude::InferenceModelState;
use crate::types::prelude::{FilterColumn, SortOrder, TableColumn};

#[tauri::command]
pub async fn get_data(
    state: State<'_, InferenceModelState>,
    filtered_by: Vec<FilterColumn>,
    sorted_by: HashMap<TableColumn, SortOrder>,
) -> Result<Vec<HFModelInferenceStatusRowData>, String> {
    let mut state_lock = state.lock().await;
    state_lock.update().await;

    if let Some(df) = &state_lock.data {
        let mut lf = df.clone().lazy();

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
            lf = lf.filter(
                col(TableColumn::ProviderName.as_str()).is_in(lit(targets).implode(), true),
            );
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

        let response: InferenceModelStatusCollection =
            InferenceModelStatusCollection::from(&result_df);
        Ok(response.data)
    } else {
        Ok(vec![])
    }
}
