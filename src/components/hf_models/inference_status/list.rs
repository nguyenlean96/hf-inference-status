use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;
use std::collections::HashMap;

use crate::components::hf_models::inference_status::{
    group_by_col_table::GroupByColTable, table::prelude::*,
};
use crate::states::prelude::*;
use crate::types::prelude::{InferenceModelStatusRowData, TableColumn};

#[component]
pub fn ModelInferenceStatusList() -> impl IntoView {
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();
    let favorite_inference_service_state = expect_context::<Store<FavoriteInferenceServiceState>>();
    let is_loading = Memo::new(move |_| model_inference_state.is_loading());

    favorite_inference_service_state.get_favorite_model_inference_ids();

    let group_by_data: Memo<Option<HashMap<String, Vec<InferenceModelStatusRowData>>>> =
        Memo::new(move |_| {
            if let Some(col) = model_inference_state.group_by_col().get() {
                let group_data = match col {
                    // Supported columns
                    TableColumn::Id
                    | TableColumn::ModelFamily
                    | TableColumn::ShortName
                    | TableColumn::ProviderName => {
                        // self.group_by = Some(col.clone());
                        // self.group_data.clear();

                        let group_data = model_inference_state.data().get().iter().fold(
                            HashMap::new(),
                            |mut acc, item| {
                                let col_value = match col {
                                    TableColumn::Id => Some(item.id.clone()),
                                    TableColumn::ModelFamily => Some(
                                        item.model_family
                                            .clone()
                                            .unwrap_or_else(|| "Unknown".to_string()),
                                    ),
                                    TableColumn::ShortName => Some(item.short_name.clone()),

                                    TableColumn::ProviderName => Some(item.provider_name.clone()),
                                    _ => None,
                                };

                                if col_value.is_none() {
                                    return acc;
                                }
                                acc.entry(col_value.unwrap())
                                    .or_insert_with(Vec::new)
                                    .push(item.clone());
                                acc
                            },
                        );

                        Ok(group_data)
                    }
                    _ => Err("Unsupported column".to_string()),
                };

                Some(group_data.unwrap_or(HashMap::new()))
            } else {
                None
            }
        });

    Effect::new(move |_| {
        let _ = model_inference_state.sort_by_cols().get();
        if favorite_inference_service_state.show_favorite_only().get() {
            spawn_local(async move {
                model_inference_state.get_data(true).await;
            });
        } else {
            spawn_local(async move {
                model_inference_state.get_data(false).await;
            });
        }
    });

    view! {
        <div class="pb-4">
            <Show when=move || { is_loading.get() }>
                <div class="flex flex-col items-center justify-center mt-10 gap-4">
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round"
                        class="size-8 animate-spin text-[#555]"
                    >
                        <circle cx="12" cy="12" r="10" stroke-opacity="0.25"/>
                        <path d="M22 12a10 10 0 0 1-10 10" stroke-opacity="0.75"/>
                    </svg>
                    <span class="text-[#777]">"Loading data..."</span>
                </div>
            </Show>
            <Show when=move || {!model_inference_state.data().get().is_empty() && model_inference_state.is_initialized()}>
                <AdvancedTable>
                    <CustomHeaderSlot col_span=2 slot:custom_headers_slot
                        alt_name="Model"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::ProviderName
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::InputPricePer1m
                        alt_name="Input"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::OutputPricePer1m
                        alt_name="Output"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::ContextWindowSize
                        alt_name="Context"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::Latency
                        alt_name="Latency"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::ThroughputTokenPerSec
                        alt_name="Throughput"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::ToolsSupport
                        alt_name="Tools"
                    />
                    <CustomHeaderSlot slot:custom_headers_slot
                        table_column=TableColumn::StructuredOutputSupport
                        alt_name="Structured"
                    />

                    <Tbody slot>
                        <Show when=move || model_inference_state.group_by_col().get().is_some()
                            fallback=move || view! {
                                <For
                                    each=move || model_inference_state.data().get()
                                    key=|item: &InferenceModelStatusRowData| item.id.clone()
                                    let(item)
                                >
                                    <TableRow item=item />
                                </For>
                            }
                        >
                            <GroupByColTable
                                group_by_data=group_by_data.get().unwrap_or_default()
                            />
                        </Show>
                    </Tbody>
                </AdvancedTable>
            </Show>
        </div>
    }
}
