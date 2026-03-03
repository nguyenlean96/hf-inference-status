use std::collections::{HashMap, HashSet};

use leptos::{logging, prelude::*};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use crate::types::prelude::{FilterColumn, InferenceModelStatusRowData, SortOrder, TableColumn};
use crate::utils::tauri_invoke::tauri_invoke;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDataProps {
    filtered_by: Vec<FilterColumn>,
    sorted_by: Vec<(TableColumn, SortOrder)>,
}

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
    pub filter_by_cols: Vec<FilterColumn>,
    pub sort_by_cols: HashMap<TableColumn, SortOrder>,
    pub group_by_col: Option<TableColumn>,
    pub model_families: Vec<String>,
    pub providers: Vec<String>,
    pub data: Vec<InferenceModelStatusRowData>,
}

impl ModelInferenceServiceState {
    pub fn new() -> Self {
        Self {
            initialized: InitStatus::NotInitialized,
            filter_by_cols: Vec::new(),
            sort_by_cols: HashMap::new(),
            group_by_col: None,
            model_families: Vec::new(),
            providers: Vec::new(),
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
    fn clear_filter_by_col(&self, col: TableColumn);
    fn toggle_col_filter_value(&self, col: TableColumn, value: Option<FilterColumn>);
    fn toggle_col_sort_order(&self, col: TableColumn);
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

    fn toggle_col_sort_order(&self, col: TableColumn) {
        self.sort_by_cols().update(|curr| {
            let next_order = match curr.get(&col) {
                Some(SortOrder::Ascending) => SortOrder::Descending,
                Some(SortOrder::Descending) => SortOrder::NotSet,
                _ => SortOrder::Ascending,
            };

            if next_order == SortOrder::NotSet {
                curr.remove(&col);
            } else {
                curr.insert(col, next_order);
            }
        });
    }

    fn clear_filter_by_col(&self, col: TableColumn) {
        self.filter_by_cols().update(|curr| match col {
            TableColumn::StructuredOutputSupport => {
                curr.retain(|f| !matches!(f, FilterColumn::StructuredOutputSupport(_)));
            }
            TableColumn::ToolsSupport => {
                curr.retain(|f| !matches!(f, FilterColumn::ToolsSupport(_)));
            }
            TableColumn::ModelFamily => {
                curr.retain(|f| !matches!(f, FilterColumn::ModelFamily(_)));
            }
            TableColumn::ProviderName => {
                curr.retain(|f| !matches!(f, FilterColumn::ProviderName(_)));
            }
            _ => {}
        });
    }

    fn toggle_col_filter_value(&self, col: TableColumn, value: Option<FilterColumn>) {
        self.filter_by_cols().update(|curr| {
            match (col, value) {
                (TableColumn::ToolsSupport, None) => {
                    if let Some(idx) = curr
                        .iter()
                        .position(|f| matches!(f, FilterColumn::ToolsSupport(_)))
                    {
                        if let FilterColumn::ToolsSupport(true) = curr[idx] {
                            curr[idx] = FilterColumn::ToolsSupport(false);
                        } else {
                            curr.remove(idx);
                        }
                    } else {
                        curr.push(FilterColumn::ToolsSupport(true));
                    }
                }
                (TableColumn::StructuredOutputSupport, None) => {
                    if let Some(idx) = curr
                        .iter()
                        .position(|f| matches!(f, FilterColumn::StructuredOutputSupport(_)))
                    {
                        if let FilterColumn::StructuredOutputSupport(true) = curr[idx] {
                            curr[idx] = FilterColumn::StructuredOutputSupport(false);
                        } else {
                            curr.remove(idx);
                        }
                    } else {
                        curr.push(FilterColumn::StructuredOutputSupport(true));
                    }
                }
                (TableColumn::ProviderName, Some(val)) | (TableColumn::ModelFamily, Some(val)) => {
                    logging::debug_log!("toggle {:#?}", val);
                    if let Some(idx) = curr.iter().position(|f| f == &val) {
                        curr.remove(idx);
                    } else {
                        curr.push(val);
                    }
                }
                _ => {}
            };
        });
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
            GetDataProps {
                filtered_by: self.filter_by_cols().get(),
                sorted_by: self.sort_by_cols().get().into_iter().collect::<Vec<_>>(),
            },
        )
        .await
        {
            Ok(data) => {
                let is_model_fams_empty = self.model_families().get().is_empty();
                let is_providers_empty = self.providers().get().is_empty();
                if is_model_fams_empty || is_providers_empty {
                    let mut model_families = HashSet::<String>::new();
                    let mut providers = HashSet::<String>::new();

                    for model in &data {
                        if let Some(model_fam) = model.model_family.clone()
                            && !model_fam.is_empty()
                        {
                            model_families.insert(model_fam);
                        }
                        providers.insert(model.provider_name.clone());
                    }

                    if is_providers_empty {
                        let mut sorted_providers = providers.into_iter().collect::<Vec<String>>();
                        sorted_providers.sort();
                        self.providers().set(sorted_providers);
                    }

                    if is_model_fams_empty {
                        let mut sorted_model_fams =
                            model_families.into_iter().collect::<Vec<String>>();
                        sorted_model_fams.sort();
                        self.model_families().set(sorted_model_fams);
                    }
                }

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
