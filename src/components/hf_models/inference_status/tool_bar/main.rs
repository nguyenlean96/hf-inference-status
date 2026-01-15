use leptos::prelude::*;

#[slot]
pub struct ToolBarAction {
    pub children: ChildrenFn,
}

#[component]
pub fn InferenceStatusToolBar(
    #[prop(default=vec![])] tool_bar_actions_slot: Vec<ToolBarAction>,
) -> impl IntoView {
    view! {
        <div class="px-2 p-1 flex items-center justify-between">
            <h1 class="text-xl text-[#ccc]">
                "Inference models status"
            </h1>

            <div class="flex items-center gap-1">
                {
                    tool_bar_actions_slot
                    .into_iter()
                    .map(|action| {
                        let children = action.children.clone();
                        view! {
                            {children()}
                        }
                    })
                    .collect_view()
                }
            </div>
        </div>
    }
}
