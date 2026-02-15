use leptos::ev::MouseEvent;
use leptos::prelude::*;
use reactive_stores::Store;

use crate::states::prelude::*;
use crate::types::{inference_table::SortOrder, prelude::TableColumn};

#[component]
pub fn CustomHeader(
    table_column: Option<TableColumn>,
    #[prop(optional, into)] alt_name: TextProp,
) -> impl IntoView {
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();

    let sorting_order = Memo::new(move |_| {
        if let Some(col) = table_column {
            Some(
                model_inference_state
                    .sort_by_cols()
                    .get()
                    .get(&col)
                    .copied()
                    .unwrap_or(SortOrder::NotSet),
            )
        } else {
            None
        }
    });
    let toggle_sorting_order = move |e: MouseEvent| {
        e.prevent_default();
        if let Some(col) = table_column {
            model_inference_state.toggle_col_sort_order(col);
        }
    };
    view! {
        <button class="flex items-center justify-between w-full gap-1"
            on:click=toggle_sorting_order
        >
            <div>{
                if !alt_name.get().is_empty() {
                    alt_name.get().to_string()
                } else if let Some(col_name) = table_column {
                    col_name.name().to_string()
                } else {
                    "".to_string()
                }
            }</div>
            <Show when=move || table_column.is_some()>
                <div>
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="size-4">
                        <path d="m7 9 5-5 5 5"
                            stroke=move || {
                                if let Some(SortOrder::Descending) = sorting_order.get() {
                                    "#FFD21E"
                                } else {
                                    "gray"
                                }
                            }
                        />
                        <path d="m7 15 5 5 5-5"
                            stroke=move || {
                                if let Some(SortOrder::Ascending) = sorting_order.get() {
                                    "#FFD21E"
                                } else {
                                    "gray"
                                }
                            }
                        />
                    </svg>
                </div>
            </Show>
        </button>
    }
}
