use chrono::{Local, NaiveDateTime, TimeDelta};
use polars::prelude::*;
use tokio::sync::Mutex;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::modules::inference_models::prelude::{
    InferenceModelStatusCollection, html_table_to_df, select_html_data_table,
    site_fetcher as fetch_data,
};
use crate::types::prelude::{FilterColumn, SortOrder, TableColumn};

#[derive(Debug, Default)]
pub struct InferenceModelStateInner {
    pub is_loading: bool,
    pub data: Option<DataFrame>,
    pub updated_at: Option<NaiveDateTime>,
}

impl InferenceModelStateInner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_model_inference_service_by_id(
        &self,
        id: &str,
    ) -> Option<HFModelInferenceStatusRowData> {
        let df = self.data.as_ref()?;
        // filter the df id column using the given id
        let filtered_df = df
            .clone()
            .lazy()
            .filter(col("id").eq(lit(id)))
            .collect()
            .ok()?;
        // convert the row to HFModelInferenceStatusRowData
        InferenceModelStatusCollection::from(&filtered_df)
            .data
            .first()
            .cloned()
    }

    fn is_stale(&self) -> bool {
        if let Some(updated_at) = &self.updated_at {
            let now_naive = Local::now().naive_local();
            let diff = now_naive - *updated_at;
            diff.abs() <= TimeDelta::seconds(30)
        } else {
            true
        }
    }

    pub async fn update(&mut self) {
        if !self.is_stale() {
            return;
        }

        self.is_loading = true;

        if let Ok(response) = fetch_data().await
            && let Some(data_table) = select_html_data_table(response)
            && let Ok(df) = html_table_to_df(data_table)
        {
            self.data = Some(df);
            self.updated_at = Some(Local::now().naive_local());
        }

        self.is_loading = false;
    }
}

pub type InferenceModelState = Mutex<InferenceModelStateInner>;

pub trait Queryable {
    fn query(
        &self,
        filters: &[FilterColumn],
        sorts: &[(TableColumn, SortOrder)],
    ) -> Result<Vec<HFModelInferenceStatusRowData>, String>;
    // fn get_by_id(&self, id: &str) -> Result<Vec<HFModelInferenceStatusRowData>, String>;
}

impl Queryable for InferenceModelStateInner {
    fn query(
        &self,
        filters: &[FilterColumn],
        sorts: &[(TableColumn, SortOrder)],
    ) -> Result<Vec<HFModelInferenceStatusRowData>, String> {
        let results = Vec::new();

        if let Some(df) = &self.data {
            let mut lf = df.clone().lazy();

            let mut filter_ids: Vec<String> = Vec::new();
            let mut filter_mfs: Vec<String> = Vec::new();
            let mut filter_providers: Vec<String> = Vec::new();

            // Phase 1: Apply simple equality filters immediately, collect values for "is_in" filters
            //
            // Note: We cannot apply `is_in` predicates immediately within the match arms because
            // each filter() call creates an AND condition. If we had filters [Id("a"), Id("b")]
            // and applied them as we encounter them, we'd get:
            //   lf.filter(col("id").is_in(["a"])).filter(col("id").is_in(["b"]))
            // which means "id == 'a' AND id == 'b'" - a condition that can never be true.
            //
            // Instead, we collect all values for each column type and apply a single `is_in`
            // predicate after the loop, creating: "id == 'a' OR id == 'b'".
            for filter_col in filters {
                let predicate = match filter_col {
                    FilterColumn::Id(id) => {
                        filter_ids.push(id.clone());
                        None
                    }
                    FilterColumn::ToolsSupport(enabled) => {
                        Some(col(TableColumn::ToolsSupport.as_str()).eq(lit(*enabled)))
                    }
                    FilterColumn::StructuredOutputSupport(enabled) => {
                        Some(col(TableColumn::StructuredOutputSupport.as_str()).eq(lit(*enabled)))
                    }
                    FilterColumn::ModelFamily(mf) => {
                        filter_mfs.push(mf.clone());
                        None
                    }
                    FilterColumn::ProviderName(provider) => {
                        filter_providers.push(provider.clone());
                        None
                    }
                };

                if let Some(p) = predicate {
                    lf = lf.filter(p);
                }
            }

            // Phase 2: Apply collected "is_in" filters
            //
            // These filters use OR logic within each column (match any of the values).
            // Different columns are still combined with AND logic.
            if !filter_ids.is_empty() {
                let targets = Series::new("targets".into(), filter_ids);
                lf = lf.filter(col(TableColumn::Id.as_str()).is_in(lit(targets).implode(), true));
            }

            if !filter_mfs.is_empty() {
                let targets = Series::new("targets".into(), filter_mfs);
                lf = lf.filter(
                    col(TableColumn::ModelFamily.as_str()).is_in(lit(targets).implode(), true),
                );
            }

            if !filter_providers.is_empty() {
                let targets = Series::new("targets".into(), filter_providers);
                lf = lf.filter(
                    col(TableColumn::ProviderName.as_str()).is_in(lit(targets).implode(), true),
                );
            }

            // 2. Apply Sorting
            lf = lf.sort(
                if !sorts.is_empty() {
                    sorts.iter().map(|(col, _)| col.as_str()).collect()
                } else {
                    vec![TableColumn::OriginalOrder.as_str()]
                },
                SortMultipleOptions::new()
                    .with_order_descending_multi(if !sorts.is_empty() {
                        sorts
                            .iter()
                            .map(|(_, v)| matches!(v, SortOrder::Descending))
                            .collect()
                    } else {
                        vec![false]
                    })
                    .with_nulls_last(true), // .with_maintain_order(true)
            );

            let result_df = lf.collect().map_err(|e| e.to_string())?;

            let row_data = InferenceModelStatusCollection::from(&result_df);
            return Ok(row_data.data);
        }

        Ok(results)
    }
}
