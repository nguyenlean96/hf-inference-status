use leptos::prelude::*;

#[component]
pub fn FilterMenu() -> impl IntoView {
    let (is_open, set_is_open) = signal::<bool>(false);

    view! {
        <div class="relative">
            <button
                class="p-1.5 flex items-center gap-0.5 \
                text-[#aaa] hover:text-white 
                hover:bg-gray-900 \
                border border-gray-800 rounded-lg"
                on:click=move |e| {
                    e.prevent_default();
                    set_is_open.update(|v| *v = !*v);
                }
            >
                <svg xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24" fill="none"
                    stroke="currentColor" stroke-width="2"
                    stroke-linecap="round" stroke-linejoin="round"
                    class="size-4.5">
                    <path d="M10 20a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341L21.74 4.67A1 1 0 0 0 21 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14z"/>
                </svg>
            </button>
            <Show when=move || {is_open.get()}>
                <div class="absolute top-[calc(100%+0.25rem)] right-0 z-10 \
                    bg-gray-900 border border-gray-800 \
                    rounded-lg p-0.5">
                    <ul class="space-y-0.5">
                        <li>
                            <button class="text-nowrap space-x-1 p-1 px-1.5 hover:bg-gray-800 hover:text-white rounded-lg w-full text-start">
                                <svg xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="inline-block size-4 stroke-yellow-600">
                                    <path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>
                                </svg>
                                <span class="text-nowrap text-sm">"Show favorite only"</span>
                            </button>
                            </li>
                        <li class="border-b border-gray-800"></li>
                        <li>
                            <button class="text-nowrap space-x-1 p-1 px-1.5 hover:bg-gray-800 hover:text-white rounded-lg w-full text-start">
                            </button>
                        </li>
                        <li class="border-b border-gray-800"></li>
                        <li>
                            <button class="text-nowrap space-x-1 p-1 px-1.5 hover:bg-gray-800 hover:text-white rounded-lg w-full text-start">
                                <span>"Reset"</span>
                            </button>
                        </li>
                    </ul>
                </div>
            </Show>
        </div>
    }
}
