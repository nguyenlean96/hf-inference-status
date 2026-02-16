use leptos::{ev::MouseEvent, prelude::*};
use reactive_stores::Store;

use crate::states::prelude::*;

#[component]
pub fn ToggleShowFavoriteOnly() -> impl IntoView {
    let favorite_inference_service_state = expect_context::<Store<FavoriteInferenceServiceState>>();
    let is_showing_favorite_only =
        Memo::new(move |_| favorite_inference_service_state.show_favorite_only().get());

    let toggle_show_favorite_only = move |e: MouseEvent| {
        e.prevent_default();
        favorite_inference_service_state
            .show_favorite_only()
            .update(|v| {
                *v = !*v;
            });
    };

    view! {
        <div class="flex items-center gap-1">
            <Show when=move || { is_showing_favorite_only.get() }>
                <div class="flex items-center gap-1 select-none">
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="#efb100"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="size-3.5"
                    >
                        <path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>
                    </svg>
                    <span class="block text-sm leading-0">"Favorites only"</span>
                </div>
            </Show>
            <button
                class="p-1.5 flex items-center gap-1 rounded-lg"
                style:background-color=move || {
                    if is_showing_favorite_only.get() {
                        "rgb(16, 24, 40)"
                    } else {
                        "rgba(239, 177, 0, 0.2)"
                    }
                }
                style:border=move || {
                    if is_showing_favorite_only.get() {
                        "1px solid rgb(30, 41, 57)"
                    } else {
                        "1px solid rgb(208, 135, 0)"
                    }
                }
                class:text-yellow-600=move || { !is_showing_favorite_only.get() }
                on:click=toggle_show_favorite_only
            >

                <Show when=move || { !is_showing_favorite_only.get() }>
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="size-3.5"
                        class:animate-pulse=move || !is_showing_favorite_only.get()
                    >
                        <path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>
                    </svg>
                </Show>
                <span class="md:block leading-0 text-sm hidden">{move ||
                    if is_showing_favorite_only.get() {
                        "Show all"
                    } else {
                        "Show favorites only"
                    }
                }</span>
            </button>
        </div>
    }
}
