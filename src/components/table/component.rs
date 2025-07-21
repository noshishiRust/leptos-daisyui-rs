/// # Table Component
///
/// A reactive Leptos wrapper for daisyUI's table component that provides
/// styled table elements for displaying tabular data.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("table table-zebra table-pin-rows table-pin-cols table-xs table-sm table-md table-lg table-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the table element ([HTMLTableElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement))

use super::style::TableSize;
use crate::merge_classes;
use leptos::{
    html::{Table as HtmlTable, Tbody, Td, Tfoot, Th, Thead, Tr},
    prelude::*,
};
#[component]
pub fn Table(
    /// Size variant for the table
    #[prop(optional, into)]
    size: Signal<TableSize>,
    /// Enable zebra striping for alternating rows
    #[prop(optional, into)]
    zebra: Signal<bool>,
    /// Pin header and footer rows when scrolling
    #[prop(optional, into)]
    pin_rows: Signal<bool>,
    /// Pin first column when scrolling horizontally
    #[prop(optional, into)]
    pin_cols: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the table element
    #[prop(optional)]
    node_ref: NodeRef<HtmlTable>,
    /// Table content
    children: Children,
) -> impl IntoView {
    view! {
        <table
            node_ref=node_ref
            class=move || {
                merge_classes!("table",
                size.get().as_str(),
                class)
            }
            class:table-zebra=zebra
            class:table-pin-rows=pin_rows
            class:table-pin-cols=pin_cols
        >
            {children()}
        </table>
    }
}

/// # Table Head Component
///
/// A reactive Leptos wrapper for table header sections.
///
/// ## Node References
/// - `node_ref` - References the thead element ([HTMLTableSectionElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement))
#[component]
pub fn TableHead(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the thead element
    #[prop(optional)]
    node_ref: NodeRef<Thead>,
    /// Header content
    children: Children,
) -> impl IntoView {
    view! {
        <thead node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </thead>
    }
}

/// # Table Body Component
///
/// A reactive Leptos wrapper for table body sections.
///
/// ## Node References
/// - `node_ref` - References the tbody element ([HTMLTableSectionElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement))
#[component]
pub fn TableBody(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the tbody element
    #[prop(optional)]
    node_ref: NodeRef<Tbody>,
    /// Body content
    children: Children,
) -> impl IntoView {
    view! {
        <tbody node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </tbody>
    }
}

/// # Table Foot Component
///
/// A reactive Leptos wrapper for table footer sections.
///
/// ## Node References
/// - `node_ref` - References the tfoot element ([HTMLTableSectionElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement))
#[component]
pub fn TableFoot(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the tfoot element
    #[prop(optional)]
    node_ref: NodeRef<Tfoot>,
    /// Footer content
    children: Children,
) -> impl IntoView {
    view! {
        <tfoot node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </tfoot>
    }
}

/// # Table Row Component
///
/// A reactive Leptos wrapper for table rows.
///
/// ## Node References
/// - `node_ref` - References the tr element ([HTMLTableRowElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement))
#[component]
pub fn TableRow(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the tr element
    #[prop(optional)]
    node_ref: NodeRef<Tr>,
    /// Row content
    children: Children,
) -> impl IntoView {
    view! {
        <tr node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </tr>
    }
}

/// # Table Header Component
///
/// A reactive Leptos wrapper for table header cells.
///
/// ## Node References
/// - `node_ref` - References the th element ([HTMLTableCellElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement))
#[component]
pub fn TableHeader(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the th element
    #[prop(optional)]
    node_ref: NodeRef<Th>,
    /// Header content
    children: Children,
) -> impl IntoView {
    view! {
        <th node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </th>
    }
}

/// # Table Cell Component
///
/// A reactive Leptos wrapper for table data cells.
///
/// ## Node References
/// - `node_ref` - References the td element ([HTMLTableCellElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement))
#[component]
pub fn TableCell(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the td element
    #[prop(optional)]
    node_ref: NodeRef<Td>,
    /// Cell content
    children: Children,
) -> impl IntoView {
    view! {
        <td node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </td>
    }
}
