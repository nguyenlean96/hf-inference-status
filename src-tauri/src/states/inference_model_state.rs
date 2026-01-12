use std::sync::Mutex;
use chrono::NaiveDateTime;
use polars::prelude::*;

pub struct InferenceModelStateInner {
    pub last_data: DataFrame,
    pub updated_at: NaiveDateTime,
}

pub type InferenceModelState = Mutex<InferenceModelStateInner>;