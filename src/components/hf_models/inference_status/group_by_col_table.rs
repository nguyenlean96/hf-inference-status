use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::hf_models::inference_status::table::prelude::*;
use crate::types::prelude::InferenceModelStatusRowData;
use crate::types::prelude::TableColumn;

#[component]
pub fn GroupByColTable(
    #[prop(into)] group_by_data: Signal<HashMap<String, Vec<InferenceModelStatusRowData>>>,
) -> impl IntoView {
    view! {
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
                <For
                    each=move || {
                        let mut model_names = group_by_data.get().keys().cloned().collect::<Vec<String>>();
                        model_names.sort();
                        model_names
                    }
                    key=|model_name| model_name.clone()
                    let(model_name)
                >
                    <tr>
                        <td colspan="10">
                            <div class="rounded p-1 flex items-center gap-1 border border-gray-800">
                                // Get the first item's avatar url
                                <img
                                    src={group_by_data.get().get(&model_name.clone()).unwrap()[0].avatar_url.clone()}
                                    alt="Model Avatar"
                                    class="size-4 rounded-md"
                                />
                                <span class="block text-sm text-nowrap leading-none">
                                    {model_name.clone()}
                                </span>
                            </div>
                        </td>
                    </tr>
                    <For
                        each=move || { group_by_data.get().get(&model_name.clone()).unwrap().clone() }
                        key=|item: &InferenceModelStatusRowData| item.id.clone()
                        let(item)
                    >
                        <TableRow item=item />
                    </For>
                </For>
            </Tbody>
        </AdvancedTable>
    }
}
