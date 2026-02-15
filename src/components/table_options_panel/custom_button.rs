use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::types::prelude::{SortOrder, TableColumn};

#[component]
pub fn CustomButton<F>(
    #[prop(optional, into)] class_name: TextProp,
    col: TableColumn,
    #[prop(optional, into, default=Signal::derive(move || SortOrder::NotSet))] sort_order: Signal<
        SortOrder,
    >,
    on_click: F,
) -> impl IntoView
where
    F: Fn(TableColumn) + Clone + Send + 'static,
{
    let handle_on_click = move |ev: MouseEvent| {
        ev.prevent_default();
        on_click(col);
    };

    view! {
        <button class=move || format!(
            "cursor-pointer p-1 flex items-center gap-1 {}",
            class_name.get()
        )
            on:click=handle_on_click
        >
            <div class="min-w-4.5 aspect-square">
                <Show when=move || matches!(sort_order.get(), SortOrder::Ascending)>
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24" fill="none" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round"
                        class="size-4.5 stroke-gray-100">
                        <path d="m3 16 4 4 4-4"/>
                        <path d="M7 20V4"/>
                        <path d="M11 4h4"/>
                        <path d="M11 8h7"/>
                        <path d="M11 12h10"/>
                    </svg>
                </Show>
                <Show when=move || matches!(sort_order.get(), SortOrder::Descending)>
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24" fill="none" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round"
                        class="size-4.5 stroke-gray-100">
                        <path d="m3 8 4-4 4 4"/>
                        <path d="M7 4v16"/>
                        <path d="M11 12h10"/>
                        <path d="M11 16h7"/>
                        <path d="M11 20h4"/>
                    </svg>
                </Show>
            </div>
            <span class="block text-xs"
                class:text-gray-50=move || !matches!(sort_order.get(), SortOrder::NotSet)
                class:text-gray-500=move || matches!(sort_order.get(), SortOrder::NotSet)
            >{move || col.name()}</span>
        </button>
    }
}
