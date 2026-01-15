use leptos::prelude::*;
use reactive_stores::Store;

use crate::components::hf_models::inference_status::main::ModelInferenceStatus;
use crate::components::nav_bar::main::NavigationBar;
use crate::states::model_inference_state::ModelInferenceServiceState;

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(ModelInferenceServiceState::new()));

    view! {
        <main class="pl-13 h-screen">
            <NavigationBar />
            <ModelInferenceStatus />
        </main>
    }
}
