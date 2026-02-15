use leptos::prelude::*;
use reactive_stores::Store;

use crate::components::hf_models::inference_status::main::ModelInferenceStatus;
use crate::components::table_options_panel::main::TableOptionsPanel;
use crate::states::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(AppState::new()));
    provide_context(Store::new(ModelInferenceServiceState::new()));
    provide_context(Store::new(FavoriteInferenceServiceState::new()));

    view! {
        <main class="h-screen">
            <ModelInferenceStatus />
            <TableOptionsPanel />
        </main>
    }
}
