use leptos::prelude::*;
use reactive_stores::Store;

use crate::components::hf_models::inference_status::main::ModelInferenceStatus;
use crate::components::nav_bar::main::NavigationBar;
use crate::states::prelude::{FavoriteInferenceServiceState, ModelInferenceServiceState};

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(ModelInferenceServiceState::new()));
    provide_context(Store::new(FavoriteInferenceServiceState::new()));

    view! {
        <main class="pl-13 h-screen">
            <NavigationBar />
            <ModelInferenceStatus />
        </main>
    }
}
