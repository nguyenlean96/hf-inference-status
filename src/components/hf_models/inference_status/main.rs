use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;

use super::list::ModelInferenceStatusList;
use super::tool_bar::toggle_show_favorite_only::ToggleShowFavoriteOnly;
use super::tool_bar::{InferenceStatusToolBar, ToolBarAction};
use crate::states::prelude::*;

#[component]
pub fn ModelInferenceStatus() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();
    let favorite_inference_service_state = expect_context::<Store<FavoriteInferenceServiceState>>();

    let is_showing_favorite_only =
        Memo::new(move |_| favorite_inference_service_state.show_favorite_only().get());

    let is_loading = Memo::new(move |_| model_inference_state.is_loading());

    let request_fetch_data = move |e: MouseEvent| {
        e.prevent_default();

        spawn_local(async move {
            model_inference_state
                .get_data(is_showing_favorite_only.get())
                .await;
        });
    };

    let handle_open_options_panel = move |e: MouseEvent| {
        e.prevent_default();
        if is_loading.get() {
            return;
        }

        app_state.options_panel_enabled().update(|v| {
            *v = !*v;
        });
    };

    view! {
        <div class="flex flex-col max-h-full min-h-full">
            <InferenceStatusToolBar>
                <ToolBarAction slot:tool_bar_actions_slot>
                    <ToggleShowFavoriteOnly />
                </ToolBarAction>
                <ToolBarAction slot:tool_bar_actions_slot>
                    <button
                        class="p-1.5 flex items-center gap-0.5 \
                        text-[#aaa] hover:text-white 
                        hover:bg-gray-900 \
                        border border-gray-800 rounded-lg"
                        on:click=request_fetch_data
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24" fill="none"
                            stroke="currentColor" stroke-width="2"
                            stroke-linecap="round" stroke-linejoin="round"
                            class="block size-4.5"
                            class:animate-spin={move || is_loading.get()}
                            >
                            <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
                            <path d="M3 3v5h5"/>
                            <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/>
                            <path d="M16 16h5v5"/>
                        </svg>
                    </button>
                </ToolBarAction>
                <ToolBarAction slot:tool_bar_actions_slot>
                    <button
                        class="p-1.5 flex items-center gap-0.5 \
                        hover:text-white 
                        hover:bg-gray-900 \
                        border rounded-lg"
                        class:text-gray-500=move || is_loading.get()
                        class:text-gray-100=move || !is_loading.get()
                        class:border-gray-900=move || is_loading.get()
                        class:border-gray-800=move || !is_loading.get()
                        on:click=handle_open_options_panel
                        disabled=move || is_loading.get()
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24" fill="none"
                            stroke="currentColor" stroke-width="2"
                            stroke-linecap="round" stroke-linejoin="round"
                            class="size-4.5">
                            <path d="M14 17H5"/>
                            <path d="M19 7h-9"/>
                            <circle cx="17" cy="17" r="3"/>
                            <circle cx="7" cy="7" r="3"/>
                        </svg>
                    </button>
                </ToolBarAction>
            </InferenceStatusToolBar>
            <div class="relative flex-1 \
                max-h-full min-h-full max-w-full \
                overflow-auto">
                <ModelInferenceStatusList />
            </div>
        </div>
    }
}
