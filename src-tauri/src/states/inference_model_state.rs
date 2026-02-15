use chrono::{Local, NaiveDateTime, TimeDelta};
use polars::prelude::*;
use tokio::sync::Mutex;

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::modules::inference_models::prelude::{
    InferenceModelStatusCollection, html_table_to_df, select_html_data_table,
    site_fetcher as fetch_data,
};

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
