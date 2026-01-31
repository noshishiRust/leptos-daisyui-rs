use crate::components::data_table::types::{Column, SortOrder};
use crate::merge_classes;
use leptos::prelude::*;

/// DataTable header component with sortable columns
#[component]
pub fn DataTableHeader(
    /// Column definitions
    #[prop(into)]
    columns: Signal<Vec<Column>>,

    /// Currently sorted column ID
    #[prop(into)]
    sort_column: Signal<Option<&'static str>>,

    /// Current sort order
    #[prop(into)]
    sort_order: Signal<SortOrder>,

    /// Callback when column header is clicked
    on_sort: Callback<&'static str>,

    /// Custom header cell class
    #[prop(optional, into)]
    header_cell_class: &'static str,
) -> impl IntoView {
    view! {
        <thead>
            <tr>
                {move || {
                    columns.get().iter().map(|col| {
                        let col_id = col.id;
                        let is_sorted = sort_column.get() == Some(col_id);
                        let is_sortable = col.sortable;

                        let cell_class = if is_sortable {
                            merge_classes!(header_cell_class, col.class.unwrap_or(""))
                        } else {
                            merge_classes!(header_cell_class, col.class.unwrap_or(""), "opacity-50")
                        };

                        let aria_sort = if is_sorted {
                            Some(sort_order.get().as_aria_str())
                        } else {
                            Some("none")
                        };

                        let min_width_style = col.min_width.map(|w| format!("min-width: {}px", w));

                        view! {
                            <th
                                class=cell_class
                                aria-sort=aria_sort
                                style=min_width_style
                                on:click=move |_| {
                                    if is_sortable {
                                        on_sort.run(col_id);
                                    }
                                }
                            >
                                <span class="flex items-center gap-1">
                                    {col.header}
                                    {move || {
                                        if is_sorted {
                                            Some(view! {
                                                <span class="text-xs">
                                                    {sort_order.get().as_symbol()}
                                                </span>
                                            })
                                        } else {
                                            None
                                        }
                                    }}
                                </span>
                            </th>
                        }
                    }).collect_view()
                }}
            </tr>
        </thead>
    }
}
