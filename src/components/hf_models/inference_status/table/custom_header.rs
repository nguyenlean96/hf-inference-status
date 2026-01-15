use leptos::ev::MouseEvent;
use leptos::prelude::*;

use super::types::SortOrder;

#[component]
pub fn CustomHeader(
    children: ChildrenFn,
    #[prop(default=SortOrder::Ascending)] sort_order: SortOrder,
) -> impl IntoView {
    let (sort_order, set_sort_order) = signal::<SortOrder>(sort_order);

    let toggle_sorting_order = move |_: MouseEvent| {
        set_sort_order.update(|order| {
            *order = match *order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            }
        });
    };
    view! {
        <button class="flex items-center justify-between w-full gap-1"
            on:click=toggle_sorting_order
        >
            <div>{children()}</div>
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
                            match sort_order.get() {
                                SortOrder::Ascending => "green",
                                SortOrder::Descending => "gray",
                            }
                        }
                    />
                    <path d="m7 15 5 5 5-5"
                        stroke=move || {
                            match sort_order.get() {
                                SortOrder::Ascending => "gray",
                                SortOrder::Descending => "green",
                            }
                        }
                    />
                </svg>
            </div>
        </button>
    }
}
