use leptos::ev::MouseEvent;
use leptos::prelude::*;
use reactive_stores::Store;

use crate::components::table_options_panel::custom_button::CustomButton;
use crate::states::prelude::*;
use crate::types::prelude::{SortOrder, TableColumn};

#[component]
pub fn TableOptionsPanel() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();

    let sort_by_col = Memo::new(move |_| model_inference_state.sort_by_cols().get());
    let handle_toggle_options_panel = move |e: MouseEvent| {
        e.prevent_default();
        app_state.options_panel_enabled().update(|val| {
            *val = !*val;
        });
    };

    let handle_toggle_sort = move |col: TableColumn| {
        model_inference_state.toggle_col_sort_order(col);
    };

    let handle_clear_sorted_cols = move || {
        model_inference_state.sort_by_cols().update(|v| {
            v.clear();
        });
    };

    let handle_clear_all = move |e: MouseEvent| {
        e.prevent_default();
        handle_clear_sorted_cols();
    };

    view! {
        <Show when=move || app_state.options_panel_enabled().get()>
            <div class="p-1 fixed right-0 bottom-0 top-0 z-5">
                <div class="p-1 flex flex-col justify-between \
                    border border-gray-800 \
                    bg-gray-950 rounded-lg h-full"
                >
                    <div class="flex flex-col gap-1 p-1">
                        <div class="border-b border-[#888] p-1">
                            <h3 class="text-xs text-[#aaa] font-semibold">"Sort by column(s)"</h3>
                        </div>
                        <ul class="gap-1">
                            <li>
                                <CustomButton
                                    col=TableColumn::InputPricePer1m
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::InputPricePer1m).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::OutputPricePer1m
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::OutputPricePer1m).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::ContextWindowSize
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::ContextWindowSize).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::Latency
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::Latency).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::ThroughputTokenPerSec
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::ThroughputTokenPerSec).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::ToolsSupport
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::ToolsSupport).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::StructuredOutputSupport
                                    on_click=handle_toggle_sort
                                    sort_order=move || sort_by_col.get().get(&TableColumn::StructuredOutputSupport).copied().unwrap_or(SortOrder::NotSet)
                                />
                            </li>
                        </ul>
                        <div class="border-b border-[#888] p-1">
                            <h3 class="text-xs text-[#aaa] font-semibold">
                                "Group by column"
                            </h3>
                        </div>
                        <ul>
                            <li></li>
                        </ul>
                    </div>

                    <div class="flex flex-col gap-1 p-1">
                        <button
                            class="p-1.5 gap-0.5 \
                            text-[#aaa] hover:text-white \
                            flex items-center justify-center \
                            bg-red-900 hover:bg-red-800 \
                            border border-red-800 rounded-lg"
                            on:click=handle_clear_all
                        >
                            <span class="block text-xs">"Clear all"</span>
                        </button>
                        <button
                            class="p-1.5 flex items-center gap-0.5 \
                            text-[#aaa] hover:text-white \
                            justify-center \
                            border border-gray-800 rounded-lg"
                            on:click=handle_toggle_options_panel
                        >
                            <span class="block text-xs">"Close"</span>
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
