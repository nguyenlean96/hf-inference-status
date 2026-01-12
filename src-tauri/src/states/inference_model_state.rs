use chrono::{Local, NaiveDateTime};
use polars::prelude::*;
use std::sync::Mutex;

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

    pub fn update(&mut self, data: DataFrame) {
        self.data = Some(data);
        self.updated_at = Some(Local::now().naive_local());
    }
}

pub type InferenceModelState = Mutex<InferenceModelStateInner>;
