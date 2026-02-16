use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::types::prelude::{SortOrder, TableColumn};

#[component]
pub fn ToggleProvider<F>(
    #[prop(optional, into)] class_name: TextProp,
    #[prop(into)] name: TextProp,
    #[prop(optional, into, default = Signal::from(false))] active: Signal<bool>,
    on_click: F,
) -> impl IntoView
where
    F: Fn(TableColumn, String) + Clone + Send + 'static,
{
    let provider_name = StoredValue::new(name.get());
    let handle_on_click = move |ev: MouseEvent| {
        ev.prevent_default();

        on_click(
            TableColumn::ProviderName,
            provider_name.get_value().to_string(),
        )
    };
    view! {
        <button class=move || format!("flex items-center gap-1 {}", class_name.get())
            on:click=handle_on_click
        >
            <div class="min-w-4.5 aspect-square \
                flex items-center justify-center"
            >
                <div class="relative min-w-4 aspect-square \
                    border border-gray-500 rounded"
                >
                    <Show when=move || active.get()>
                        <span class="absolute top-1/2 left-1/2 -translate-1/2 \
                            text-lime-400 text-sm"
                        >"✔"</span>
                    </Show>
                </div>
            </div>
            <span class="text-xs"
                class:text-gray-100=move || active.get()
                class:text-gray-800=move || !active.get()
            >{provider_name.get_value()}</span>
        </button>
    }
}

#[component]
pub fn ToggleButton(
    #[prop(optional, into)] class_name: TextProp,
    col: TableColumn,
    #[prop(optional, into, default=Signal::derive(move || None))] active: Signal<Option<bool>>,
) -> impl IntoView {
    let has_filter = Memo::new(move |_| active.get().is_some());

    view! {
        <div
            class=move || format!(
                "flex items-center gap-1 {}",
                class_name.get()
            )
        >
            <div class="min-w-4.5 w-4.5 aspect-square flex items-center justify-center">
                <div class="relative flex items-center justify-center \
                    min-w-4 w-4 aspect-square rounded \
                    border border-gray-500"
                >
                    <Show when=move || has_filter.get()>
                        <span class="absolute top-1/2 left-1/2 -translate-1/2 text-sm"
                            class:text-lime-400=move || active.get().unwrap_or_default()
                            class:text-red-600=move || !active.get().unwrap_or_default()
                        >
                            {move || if active.get().unwrap_or_default() {"✔"} else {"✗"}}
                        </span>
                    </Show>
                </div>
            </div>
            <span class="block text-xs"
                class:text-gray-50=move || has_filter.get()
                class:text-gray-500=move || !has_filter.get()
            >{col.name()}</span>
        </div>
    }
}

#[component]
pub fn SortButton(
    #[prop(optional, into)] class_name: TextProp,
    col: TableColumn,
    #[prop(optional, into, default=Signal::derive(move || SortOrder::NotSet))] sort_order: Signal<
        SortOrder,
    >,
) -> impl IntoView {
    view! {
        <div class=move || format!(
                "flex items-center gap-1 {}",
                class_name.get()
            )
        >
            <div class="min-w-4.5 aspect-square">
                <div class="relative flex items-center justify-center \
                    min-w-4 w-4 aspect-square rounded \
                    border border-gray-500"
                >
                    <Show when=move || !matches!(col, TableColumn::ToolsSupport) && !matches!(col, TableColumn::StructuredOutputSupport)>
                        <div class="absolute top-1/2 left-1/2 -translate-1/2">
                            <Show when=move || matches!(sort_order.get(), SortOrder::Ascending)>
                                <svg xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24" fill="none" stroke-width="2"
                                    stroke-linecap="round" stroke-linejoin="round"
                                    class="size-4 stroke-gray-100">
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
                                    class="size-4 stroke-gray-100">
                                    <path d="m3 8 4-4 4 4"/>
                                    <path d="M7 4v16"/>
                                    <path d="M11 12h10"/>
                                    <path d="M11 16h7"/>
                                    <path d="M11 20h4"/>
                                </svg>
                            </Show>
                        </div>
                    </Show>
                </div>
            </div>
            <span class="block text-xs"
                class:text-gray-50=move || !matches!(sort_order.get(), SortOrder::NotSet)
                class:text-gray-500=move || matches!(sort_order.get(), SortOrder::NotSet)
            >{move || col.name()}</span>
        </div>
    }
}

#[component]
pub fn CustomButton<F>(
    #[prop(optional, into)] class_name: TextProp,
    col: TableColumn,
    on_click: F,
    children: ChildrenFn,
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
            "cursor-pointer flex items-center {}",
            class_name.get()
        )
            on:click=handle_on_click
        >
            {(children)()}
        </button>
    }
}
