use leptos::{html::Div, prelude::*};
use crate::components::data_table::body::DataTableBody;
use crate::components::data_table::controls::DataTableControls;
use crate::components::data_table::header::DataTableHeader;
use crate::components::data_table::types::{
    Column, DataTableClasses, DataTableTexts, SortOrder, TableRow,
};
use crate::components::table::{Table, TableSize};
use crate::merge_classes;

/// # DataTable Component
///
/// A production-ready data table with sorting, pagination, loading states,
/// and efficient handling of large datasets (10,000+ rows).
///
/// ## Features
/// - Column-based sorting (click headers to toggle Asc/Desc)
/// - Pagination with customizable page size
/// - Loading and empty states
/// - Fully themed with daisyUI
/// - Efficient index-based operations for large datasets
///
/// ## Example
/// ```rust,no_run
/// use std::collections::HashMap;
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::*;
///
/// #[component]
/// fn MyComponent() -> impl IntoView {
///     let columns = vec![
///         Column::new("name", "Name"),
///         Column::new("email", "Email"),
///         Column::new_non_sortable("status", "Status"),
///     ];
///
///     let data = vec![
///         HashMap::from([
///             ("name", "Alice".to_string()),
///             ("email", "alice@example.com".to_string()),
///             ("status", "Active".to_string()),
///         ]),
///     ];
///
///     view! {
///         <DataTable
///             columns=Signal::derive(move || columns.clone())
///             data=Signal::derive(move || data.clone())
///             page_size=10
///         />
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("table table-zebra table-pin-rows table-pin-cols table-xs table-sm table-md table-lg");
/// @source inline("btn btn-sm animate-pulse");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DataTable(
    /// Table data (HashMap with column IDs as keys)
    #[prop(into)]
    data: Signal<Vec<TableRow>>,

    /// Column definitions
    #[prop(into)]
    columns: Signal<Vec<Column>>,

    /// Number of rows per page (default: 10)
    #[prop(optional, into)]
    page_size: Signal<usize>,

    /// Loading state
    #[prop(optional, into)]
    loading: Signal<bool>,

    /// Enable pagination (default: true)
    #[prop(optional, into)]
    paginate: Signal<bool>,

    /// Custom CSS classes
    #[prop(optional)]
    classes: DataTableClasses,

    /// Custom text strings
    #[prop(optional)]
    texts: DataTableTexts,

    /// Additional CSS classes for container
    #[prop(optional, into)]
    class: &'static str,

    /// Table size variant
    #[prop(optional, into)]
    table_size: Signal<TableSize>,

    /// Enable zebra striping
    #[prop(optional, into)]
    zebra: Signal<bool>,

    /// Pin header and footer rows
    #[prop(optional, into)]
    pin_rows: Signal<bool>,

    /// Pin first column
    #[prop(optional, into)]
    pin_cols: Signal<bool>,

    /// Node reference to container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    // Default page size to 10 if not set
    let page_size = Signal::derive(move || {
        let size = page_size.get();
        if size == 0 {
            10
        } else {
            size
        }
    });

    // Default paginate to true
    let paginate = Signal::derive(move || {
        let p = paginate.get();
        if p {
            p
        } else {
            true
        }
    });

    // Pagination state
    let (current_page, set_current_page) = signal(0_usize);

    // Sorting state
    let (sort_column, set_sort_column) = signal(Option::<&'static str>::None);
    let (sort_order, set_sort_order) = signal(SortOrder::default());

    // Reset to page 1 when data changes
    Effect::new(move |_| {
        let _ = data.get();
        set_current_page.set(0);
    });

    // Filtered indices (all rows for now - placeholder for future filtering)
    let filtered_indices = Memo::new(move |_| (0..data.get().len()).collect::<Vec<usize>>());

    // Sorted indices
    let sorted_indices = Memo::new(move |_| {
        let mut indices = filtered_indices.get();
        if let Some(col_id) = sort_column.get() {
            let data_vec = data.get();
            indices.sort_by(|&a, &b| {
                let a_val = data_vec
                    .get(a)
                    .and_then(|row| row.get(col_id))
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let b_val = data_vec
                    .get(b)
                    .and_then(|row| row.get(col_id))
                    .map(|s| s.as_str())
                    .unwrap_or("");
                match sort_order.get() {
                    SortOrder::Asc => a_val.cmp(b_val),
                    SortOrder::Desc => b_val.cmp(a_val),
                }
            });
        }
        indices
    });

    // Total pages calculation with safety guards
    let total_pages = Memo::new(move |_| {
        let safe_page_size = page_size.get().max(1); // Prevent division by zero
        let total_items = sorted_indices.get().len();
        if total_items == 0 {
            1 // Always show at least "Page 1 of 1"
        } else {
            ((total_items as f64 / safe_page_size as f64).ceil() as usize).max(1)
        }
    });

    // Current page rows with safety guards
    let current_page_rows = Memo::new(move |_| {
        if !paginate.get() {
            // No pagination: return all rows
            return sorted_indices
                .get()
                .iter()
                .filter_map(|&idx| data.get().get(idx).cloned())
                .collect::<Vec<_>>();
        }

        let safe_page = current_page.get().min(total_pages.get().saturating_sub(1));
        let start = safe_page * page_size.get();
        let end = ((safe_page + 1) * page_size.get()).min(sorted_indices.get().len());

        sorted_indices.get()[start..end]
            .iter()
            .filter_map(|&idx| data.get().get(idx).cloned())
            .collect::<Vec<_>>()
    });

    // Sort callback
    let on_sort = Callback::new(move |col_id: &'static str| {
        if sort_column.get() == Some(col_id) {
            // Same column: toggle order
            set_sort_order.set(sort_order.get().toggle());
        } else {
            // New column: set to Asc
            set_sort_column.set(Some(col_id));
            set_sort_order.set(SortOrder::Asc);
        }
    });

    let container_class = merge_classes!(classes.container, class);
    let texts_for_body = texts.clone();
    let texts_for_controls = texts.clone();

    view! {
        <div class=container_class node_ref=node_ref>
            <Table
                size=table_size
                zebra=zebra
                pin_rows=pin_rows
                pin_cols=pin_cols
            >
                <DataTableHeader
                    columns=columns
                    sort_column=Signal::derive(move || sort_column.get())
                    sort_order=Signal::derive(move || sort_order.get())
                    on_sort=on_sort
                    header_cell_class=classes.header_cell
                />
                <DataTableBody
                    columns=columns
                    rows=Signal::derive(move || current_page_rows.get())
                    loading=loading
                    texts=texts_for_body
                    body_cell_class=classes.body_cell
                    row_class=classes.row
                    loading_row_class=classes.loading_row
                    empty_row_class=classes.empty_row
                />
            </Table>

            {move || {
                if paginate.get() && !loading.get() && !data.get().is_empty() {
                    Some(view! {
                        <DataTableControls
                            current_page=Signal::derive(move || current_page.get())
                            set_current_page=set_current_page
                            total_pages=Signal::derive(move || total_pages.get())
                            texts=texts_for_controls.clone()
                            pagination_class=classes.pagination
                            button_class=classes.pagination_button
                            indicator_class=classes.page_indicator
                        />
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
