use leptos::ev::MouseEvent;
use leptos::prelude::*;
use reactive_stores::Store;

use crate::states::prelude::*;
use crate::types::inference_model_status::InferenceModelStatusRowData;
use crate::utils::formatter::format_with_commas;

#[component]
pub fn TableRow(item: InferenceModelStatusRowData) -> impl IntoView {
    let favorite_inference_service_state = expect_context::<Store<FavoriteInferenceServiceState>>();
    let item_clone = StoredValue::new(item.clone());
    let is_favorite =
        Memo::new(move |_| favorite_inference_service_state.is_favorite(item.id.clone().as_str()));

    let toggle_favorite = move |e: MouseEvent| {
        e.prevent_default();

        if is_favorite.get() {
            favorite_inference_service_state.remove_favorite(item_clone.get_value().id);
        } else {
            favorite_inference_service_state.add_favorite(item_clone.get_value().id);
        }
    };

    view! {
        <tr class="text-xs">
            <td>
                <div class="flex items-center gap-2">
                    <img
                        src={item.avatar_url.clone()}
                        alt="Model Avatar"
                        class="size-4 rounded-md"
                    />
                    <span class="block text-nowrap leading-none">
                        {
                            item.model_family.clone().map(|family| view! {
                                <span>{family}"/"</span>
                            })
                        }
                        <span>
                            {item.short_name}
                        </span>
                    </span>
                </div>
            </td>
            <td>
                <div class="flex items-center justify-end flex-nowrap gap-1">
                    <a
                        href={item.model_details_url.clone()}
                        target="_blank"
                        class="block bg-[#888] rounded p-0.5"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="size-3">
                            <path d="M7 7h10v10"/>
                            <path d="M7 17 17 7"/>
                        </svg>
                    </a>
                    <a
                        href=item.model_inference_instruction_url.clone()
                        target="_blank"
                        class="block bg-[#888] rounded p-0.5"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="#aaa"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="size-3">
                            <path d="m16 18 6-6-6-6"/>
                            <path d="m8 6-6 6 6 6"/>
                        </svg>
                    </a>
                    <button
                        class="block rounded p-0.5"
                        on:click=toggle_favorite
                        style:background-color=move || {
                            format!("rgba(239, 177, 0, {})", if is_favorite.get() { 1.0 } else { 0.3 })
                        }
                    >
                        <svg xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke=move || {
                                if is_favorite.get() {
                                    "white"
                                } else {
                                    "#aaa"
                                }
                            }
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="size-3">
                            <path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>
                        </svg>
                    </button>
                </div>
            </td>
            <td>
                <span class="text-nowrap">{item.provider_name}</span>
            </td>
            <td>
                <span class="text-nowrap">
                    {
                        match item.input_price_per_1m {
                            Some(price) => format!("$ {:.2}", price),
                            None => "-".to_string(),
                        }
                    }
                </span>
            </td>
            <td>
                <span class="text-nowrap">
                    {
                        match item.output_price_per_1m {
                            Some(price) => format!("$ {:.2}", price),
                            None => "-".to_string(),
                        }
                    }
                </span>
            </td>
            <td>
                <span>
                    {
                        match item.context_window_size {
                            Some(size) => format_with_commas(size as u64),
                            None => "-".to_string(),
                        }
                    }
                </span>
            </td>
            <td>
                <span>
                    {match item.latency {
                        Some(l) => format!("{:.2}", l),
                        None => "-".to_string(),
                    }}
                </span>
            </td>
            <td>
                <span>{item.throughput_token_per_sec}</span>
            </td>
            <td class="text-center"
                style:background-color=if item.tools_support {
                    "rgba(124, 207, 0, 0.1)"
                } else {
                    // "rgba(194, 150, 83, 0.3)"
                    "transparent"
                }
            >
                <Show when=move || { item.tools_support }
                    fallback=|| view! { <span class="text-red-500">"✗"</span> }
                >
                    <span class="text-lime-400">"✔"</span>
                </Show>
            </td>
            <td class="text-center"
                style:background-color=if item.structured_output_support {
                    "rgba(124, 207, 0, 0.1)"
                } else {
                    // "rgba(194, 150, 83, 0.3)"
                    "transparent"
                }
            >
                <Show when=move || { item.structured_output_support }
                    fallback=|| view! { <span class="text-red-500">"✗"</span> }
                >
                    <span class="text-lime-400">"✔"</span>
                </Show>
            </td>
        </tr>
    }
}
