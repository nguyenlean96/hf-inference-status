use chrono::{Local, NaiveDateTime};
use polars::prelude::*;
use tokio::sync::Mutex;

use crate::modules::inference_models::{
    fetch::fetch_data, parser::html_table_to_df, utils::select_html_data_table,
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

    pub async fn update(&mut self) {
        self.is_loading = true;

        if let Ok(response) = fetch_data().await {
            if let Some(data_table) = select_html_data_table(response) {
                if let Ok(df) = html_table_to_df(data_table) {
                    self.data = Some(df);
                    self.updated_at = Some(Local::now().naive_local());
                }
            }
        }
        self.is_loading = false;
    }
}

pub type InferenceModelState = Mutex<InferenceModelStateInner>;
