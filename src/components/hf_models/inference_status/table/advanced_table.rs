use leptos::prelude::*;

use super::custom_header::CustomHeader;
use crate::types::prelude::TableColumn;

#[slot]
pub struct CustomHeaderSlot {
    #[prop(default = 1)]
    pub col_span: u32,
    #[prop(optional)]
    pub table_column: Option<TableColumn>,
    #[prop(optional, into)]
    pub alt_name: TextProp,
}

#[slot]
pub struct Tbody {
    pub children: ChildrenFn,
}

#[component]
pub fn AdvancedTable(
    #[prop(default=vec![])] custom_headers_slot: Vec<CustomHeaderSlot>,
    tbody: Tbody,
) -> impl IntoView {
    view! {
        <div>
            <table class="">
                <thead>
                    <tr class="sticky top-0 z-2 bg-gray-950">
                        {
                            custom_headers_slot
                            .into_iter()
                            .map(|ch| {
                                view! {
                                    <th class="text-xs bg-gray-900"
                                        colspan={ch.col_span}>
                                        <CustomHeader
                                            alt_name=ch.alt_name
                                            table_column=ch.table_column
                                        />
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
