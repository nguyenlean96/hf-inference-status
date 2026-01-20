use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;

use crate::components::hf_models::inference_status::table::prelude::*;
use crate::states::prelude::*;
use crate::types::inference_model_status::InferenceModelStatusRowData;

#[component]
pub fn ModelInferenceStatusList() -> impl IntoView {
    let model_inference_state = expect_context::<Store<ModelInferenceServiceState>>();
    let favorite_inference_service_state = expect_context::<Store<FavoriteInferenceServiceState>>();
    let is_loading = Memo::new(move |_| model_inference_state.is_loading());

    favorite_inference_service_state.get_favorite_model_inference_ids();

    Effect::new(move |_| {
        if favorite_inference_service_state.show_favorite_only().get() {
            spawn_local(async move {
                model_inference_state.get_data(true).await;
            });
        } else {
            spawn_local(async move {
                model_inference_state.get_data(false).await;
            });
        }
    });

    view! {
        <div class="pb-4">
            <Show when=move || { is_loading.get() }>
                <div class="flex flex-col items-center justify-center mt-10 gap-4">
                    <svg xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round"
                        class="size-8 animate-spin text-[#555]"
                    >
                        <circle cx="12" cy="12" r="10" stroke-opacity="0.25"/>
                        <path d="M22 12a10 10 0 0 1-10 10" stroke-opacity="0.75"/>
                    </svg>
                    <span class="text-[#777]">"Loading data..."</span>
                </div>
            </Show>
            <Show when=move || {!model_inference_state.data().get().is_empty() && model_inference_state.is_initialized()}>
                <AdvancedTable>
                    <CustomHeader slot:custom_headers_slot>"Model"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Provider"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Input"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Output"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Context"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Latency"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Throughput"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Tools"</CustomHeader>
                    <CustomHeader slot:custom_headers_slot>"Structured"</CustomHeader>
                    <Tbody slot>
                        <For
                            each=move || model_inference_state.data().get()
                            key=|item: &InferenceModelStatusRowData| item.id.clone()
                            let(item)
                        >
                            <TableRow item=item />
                        </For>
                    </Tbody>
                </AdvancedTable>
            </Show>
        </div>
    }
}
