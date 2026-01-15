use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;

use super::list::ModelInferenceStatusList;
use super::tool_bar::{InferenceStatusToolBar, ToolBarAction};
use crate::states::model_inference_state::{
    ModelInferenceServiceState, ModelInferenceServiceStateExt,
};

#[component]
pub fn ModelInferenceStatus() -> impl IntoView {
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();

    let is_loading = Memo::new(move |_| model_inference_state.is_loading());
    let request_fetch_data = move |e: MouseEvent| {
        e.prevent_default();

        spawn_local(async move {
            model_inference_state.get_data().await;
        });
    };
    view! {
        <div class="flex flex-col max-h-full min-h-full">
            <InferenceStatusToolBar>
                <ToolBarAction slot:tool_bar_actions_slot>
                    <button
                        class="p-1.5 flex items-center gap-0.5 \
                        text-yellow-600 \
                        bg-yellow-500/20 hover:bg-yellow-500/40 \
                        border border-yellow-800 rounded-lg"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="size-4">
                            <path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>
                        </svg>
                        <span class="block leading-none text-sm">"Favorite"</span>
                    </button>
                </ToolBarAction>
                <ToolBarAction slot:tool_bar_actions_slot>
                    <button
                        class="p-1.5 flex items-center gap-0.5 \
                        text-[#aaa] hover:text-white 
                        hover:bg-gray-900 \
                        border border-gray-800 rounded-lg"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24" fill="none"
                            stroke="currentColor" stroke-width="2"
                            stroke-linecap="round" stroke-linejoin="round"
                            class="size-4.5">
                            <path d="M10 20a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341L21.74 4.67A1 1 0 0 0 21 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14z"/>
                        </svg>
                    </button>
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
            </InferenceStatusToolBar>
            <div class="relative flex-1 \
                max-h-full min-h-full max-w-full \
                overflow-auto">
                <ModelInferenceStatusList />
            </div>
        </div>
    }
}
