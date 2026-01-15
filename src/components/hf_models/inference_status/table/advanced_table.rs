use leptos::prelude::*;

use super::custom_header::CustomHeader;
use super::types::SortOrder;

#[slot]
pub struct CustomHeader {
    #[prop(default=SortOrder::Ascending)]
    pub sort_order: SortOrder,
    pub children: ChildrenFn,
}

#[slot]
pub struct Tbody {
    pub children: ChildrenFn,
}

#[component]
pub fn AdvancedTable(
    #[prop(default=vec![])] custom_headers_slot: Vec<CustomHeader>,
    tbody: Tbody,
) -> impl IntoView {
    view! {
        <div>
            <table class="text-xs">
                <thead>
                    <tr class="sticky top-0 z-2 bg-gray-950">
                        {
                            custom_headers_slot
                            .into_iter()
                            .map(|ch| {
                                let children = ch.children.clone();
                                view! {
                                    <th class="text-xs bg-gray-900">
                                        <CustomHeader sort_order=ch.sort_order>
                                            {children()}
                                        </CustomHeader>
                                    </th>
                                }
                            })
                            .collect_view()
                        }
                    </tr>
                </thead>
                <tbody>
                    {(tbody.children)().into_any()}
                </tbody>
            </table>
        </div>
    }
}
