use leptos::{logging, prelude::*};
use reactive_stores::Store;

use crate::types::inference_model_status::InferenceModelStatusRowData;
use crate::utils::tauri_invoke::tauri_invoke;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitStatus {
    NotInitialized,
    Loading,
    Initialized,
    Error(String),
}

#[derive(Clone, Store)]
pub struct ModelInferenceServiceState {
    pub initialized: InitStatus,
    pub data: Vec<InferenceModelStatusRowData>,
}

impl ModelInferenceServiceState {
    pub fn new() -> Self {
        Self {
            initialized: InitStatus::NotInitialized,
            data: Vec::new(),
        }
    }
}

impl Default for ModelInferenceServiceState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ModelInferenceServiceStateExt {
    fn is_loading(&self) -> bool;
    fn is_initialized(&self) -> bool;
    fn has_error(&self) -> Option<String>;
    async fn get_data(&self, favorites_only: bool);
}

impl ModelInferenceServiceStateExt for Store<ModelInferenceServiceState> {
    fn is_loading(&self) -> bool {
        matches!(self.initialized().get(), InitStatus::Loading)
    }

    fn is_initialized(&self) -> bool {
        matches!(self.initialized().get(), InitStatus::Initialized)
    }

    fn has_error(&self) -> Option<String> {
        match self.initialized().get() {
            InitStatus::Error(err) => Some(err),
            _ => None,
        }
    }

    async fn get_data(&self, favorites_only: bool) {
        if self.initialized().get() == InitStatus::Loading {
            return;
        }

        self.initialized().set(InitStatus::Loading);
        match tauri_invoke::<Vec<InferenceModelStatusRowData>>(
            if favorites_only {
                "get_favorite_model_inference_data"
            } else {
                "get_data"
            },
            (),
        )
        .await
        {
            Ok(data) => {
                self.data().set(data);
                self.initialized().set(InitStatus::Initialized);
            }
            Err(err) => {
                logging::error!("Failed to fetch model status data: {:?}", err);
                self.initialized()
                    .set(InitStatus::Error(format!("{:?}", err)));
            }
        }
    }
}
