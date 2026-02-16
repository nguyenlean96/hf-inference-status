use leptos::ev::MouseEvent;
use leptos::prelude::*;
use reactive_stores::Store;

use crate::components::table_options_panel::custom_button::{
    CustomButton, SortButton, ToggleButton, ToggleProvider,
};
use crate::states::prelude::*;
use crate::types::prelude::{FilterColumn, SortOrder, TableColumn};

#[component]
pub fn TableOptionsPanel() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();

    let is_loading = Memo::new(move |_| model_inference_state.is_loading());

    let sort_by_col = Memo::new(move |_| model_inference_state.sort_by_cols().get());
    let filter_cols = Memo::new(move |_| model_inference_state.filter_by_cols().get());

    let handle_toggle_options_panel = move |e: MouseEvent| {
        e.prevent_default();
        app_state.options_panel_enabled().update(|val| {
            *val = !*val;
        });
    };

    let handle_toggle_sort = move |col: TableColumn| {
        if is_loading.get() {
            return;
        }
        model_inference_state.toggle_col_sort_order(col);
    };

    let handle_toggle_provider = move |col: TableColumn, provider_name: String| {
        if is_loading.get() {
            return;
        }
        model_inference_state
            .toggle_col_filter_value(col, Some(FilterColumn::ProviderName(provider_name)));
    };

    let handle_toggle_bool = move |col: TableColumn| {
        if is_loading.get() {
            return;
        }
        model_inference_state.toggle_col_filter_value(col, None);
    };

    let clear_bool_filter = move || {
        model_inference_state.clear_filter_by_col(TableColumn::ToolsSupport);
        model_inference_state.clear_filter_by_col(TableColumn::StructuredOutputSupport);
    };

    let clear_provider_filter = move || {
        model_inference_state.clear_filter_by_col(TableColumn::ProviderName);
    };

    let handle_clear_provider_filter = move |e: MouseEvent| {
        e.prevent_default();
        if is_loading.get() {
            return;
        }
        clear_provider_filter();
    };

    let clear_sorted_cols = move || {
        model_inference_state.sort_by_cols().update(|v| {
            v.clear();
        });
    };

    let handle_clear_all = move |e: MouseEvent| {
        e.prevent_default();
        if is_loading.get() {
            return;
        }

        clear_bool_filter();
        clear_sorted_cols();
        clear_provider_filter();
    };

    view! {
        <Show when=move || app_state.options_panel_enabled().get()>
            <div class="p-1 fixed right-0 bottom-0 top-0 z-5">
                <div class="p-1 flex flex-col justify-between \
                    border border-gray-800 bg-gray-950 \
                    rounded-lg h-full max-h-full overflow-y-hidden"
                >
                    <div class="flex flex-col gap-1 p-1 pr-4 pb-8 \
                        max-h-full overflow-y-auto"
                    >
                        <div class="border-b border-[#888] p-1">
                            <h3 class="text-xs text-[#aaa] font-semibold">
                                "Sort by column(s)"
                            </h3>
                        </div>
                        <ul class="space-y-1">
                            <li>
                                <CustomButton
                                    col=TableColumn::InputPricePer1m
                                    on_click=handle_toggle_sort
                                >
                                    <SortButton
                                        col=TableColumn::InputPricePer1m
                                        sort_order=move || sort_by_col.get().get(&TableColumn::InputPricePer1m).copied().unwrap_or(SortOrder::NotSet)
                                    />
                                </CustomButton>
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::OutputPricePer1m
                                    on_click=handle_toggle_sort
                                >
                                    <SortButton
                                        col=TableColumn::OutputPricePer1m
                                        sort_order=move || sort_by_col.get().get(&TableColumn::OutputPricePer1m).copied().unwrap_or(SortOrder::NotSet)
                                    />
                                </CustomButton>
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::ContextWindowSize
                                    on_click=handle_toggle_sort
                                >
                                    <SortButton
                                        col=TableColumn::ContextWindowSize
                                        sort_order=move || sort_by_col.get().get(&TableColumn::ContextWindowSize).copied().unwrap_or(SortOrder::NotSet)
                                    />
                                </CustomButton>
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::Latency
                                    on_click=handle_toggle_sort
                                >
                                    <SortButton
                                        col=TableColumn::Latency
                                        sort_order=move || sort_by_col.get().get(&TableColumn::Latency).copied().unwrap_or(SortOrder::NotSet)
                                    />
                                </CustomButton>
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::ThroughputTokenPerSec
                                    on_click=handle_toggle_sort
                                >
                                    <SortButton
                                        col=TableColumn::ThroughputTokenPerSec
                                        sort_order=move || sort_by_col.get().get(&TableColumn::ThroughputTokenPerSec).copied().unwrap_or(SortOrder::NotSet)
                                    />
                                </CustomButton>
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::ToolsSupport
                                    on_click=handle_toggle_bool
                                >
                                    <ToggleButton
                                        col=TableColumn::ToolsSupport
                                        active=move || {
                                            let has_enabled = filter_cols.get().into_iter().find(|f| matches!(f, FilterColumn::ToolsSupport(_)));
                                            if let Some(some_enabled) = has_enabled {
                                                if let FilterColumn::ToolsSupport(true) = some_enabled {
                                                    Some(true)
                                                } else {
                                                    Some(false)
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                    />
                                </CustomButton>
                            </li>
                            <li>
                                <CustomButton
                                    col=TableColumn::StructuredOutputSupport
                                    on_click=handle_toggle_bool
                                >
                                    <ToggleButton
                                        col=TableColumn::StructuredOutputSupport
                                        active=move || {
                                            let has_enabled = filter_cols.get().into_iter().find(|f| matches!(f, FilterColumn::StructuredOutputSupport(_)));
                                            if let Some(some_enabled) = has_enabled {
                                                if let FilterColumn::StructuredOutputSupport(true) = some_enabled {
                                                    Some(true)
                                                } else {
                                                    Some(false)
                                                }
                                            } else {
                                                None
                                            }
                                        }
                                    />
                                </CustomButton>
                            </li>
                        </ul>

                        <div class="border-b border-[#888] p-1">
                            <h3 class="text-xs text-[#aaa] font-semibold">
                                "Select Providers"
                            </h3>
                        </div>
                        <ul class="space-y-1">
                            <For
                                each=move || model_inference_state.providers().get()
                                key=|i| i.clone()
                                let(p)
                            >
                                {
                                    let provider = StoredValue::new(p);
                                    view! {
                                        <li>
                                            <ToggleProvider
                                                name=provider.get_value()
                                                active=move || filter_cols.get().iter().any(|f| *f == FilterColumn::ProviderName(provider.get_value()))
                                                on_click=handle_toggle_provider
                                            />
                                        </li>
                                    }
                                }
                            </For>
                        </ul>
                        <Show when=move || filter_cols.get().iter().any(|f| matches!(f, FilterColumn::ProviderName(_)))>
                            <button
                                class="p-1.5 gap-0.5 \
                                text-gray-200 hover:text-gray-50 \
                                flex items-center justify-center \
                                bg-gray-900 hover:bg-yellow-700 \
                                rounded-lg border \
                                border-gray-800 hover:border-red-800"
                                on:click=handle_clear_provider_filter
                            >
                                <span class="block text-xs">"Clear provider filter"</span>
                            </button>
                        </Show>

                        <div class="border-b border-[#888] p-1">
                            <h3 class="text-xs text-[#aaa] font-semibold">
                                "Group by column"
                            </h3>
                        </div>
                        <ul class="space-y-1">
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
