use super::style::TableSize;
use crate::merge_classes;
use leptos::{
    html::{Table as HtmlTable, Tbody, Td, Tfoot, Th, Thead, Tr},
    prelude::*,
};

/// A styled table component for displaying tabular data.
///
/// The `Table` component provides a semantic HTML table with daisyUI styling
/// and various customization options including size variants, zebra striping,
/// and pinned rows/columns for better data visualization.
///
/// # Props
///
/// - `size` - Optional table size variant (default: `TableSize::Default`)
/// - `zebra` - Optional zebra striping for alternating row backgrounds
/// - `pin_rows` - Optional pinning of header and footer rows during scroll
/// - `pin_cols` - Optional pinning of first column during horizontal scroll
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the table element
/// - `children` - Table content (typically TableHead, TableBody, TableFoot)
///
/// # CSS Classes
///
/// - Base: `table`
/// - Size: `table-xs`, `table-sm`, `table-md`, `table-lg` (based on size prop)
/// - Zebra: `table-zebra` (when zebra is true)
/// - Pin rows: `table-pin-rows` (when pin_rows is true)
/// - Pin cols: `table-pin-cols` (when pin_cols is true)
///
/// # Examples
///
/// ## Basic Table
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableHead>
///                 <TableRow>
///                     <TableHeader>"Product"</TableHeader>
///                     <TableHeader>"Price"</TableHeader>
///                 </TableRow>
///             </TableHead>
///             <TableBody>
///                 <TableRow>
///                     <TableCell>"MacBook"</TableCell>
///                     <TableCell>"$1999"</TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     }
/// }
/// ```
///
/// ## With All Features
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let table_size = RwSignal::new(TableSize::Sm);
///     let zebra_enabled = RwSignal::new(true);
///     let pin_rows = RwSignal::new(true);
///     let pin_cols = RwSignal::new(true);
///
///     view! {
///         <div class="overflow-x-auto">
///             <Table
///                 size=table_size
///                 zebra=zebra_enabled
///                 pin_rows=pin_rows
///                 pin_cols=pin_cols
///             >
///                 <TableHead>
///                     <TableRow>
///                         <TableHeader>"Name"</TableHeader>
///                         <TableHeader>"Q1"</TableHeader>
///                         <TableHeader>"Q2"</TableHeader>
///                         <TableHeader>"Q3"</TableHeader>
///                         <TableHeader>"Q4"</TableHeader>
///                     </TableRow>
///                 </TableHead>
///                 <TableBody>
///                     <TableRow>
///                         <TableCell>"Sales"</TableCell>
///                         <TableCell>"$100K"</TableCell>
///                         <TableCell>"$120K"</TableCell>
///                         <TableCell>"$110K"</TableCell>
///                         <TableCell>"$130K"</TableCell>
///                     </TableRow>
///                 </TableBody>
///             </Table>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn Table(
    /// Size variant for the table (affects padding and font size)
    #[prop(optional, into)]
    size: Signal<TableSize>,
    /// Enable zebra striping for alternating row backgrounds
    #[prop(optional, into)]
    zebra: Signal<bool>,
    /// Pin header and footer rows when scrolling
    #[prop(optional, into)]
    pin_rows: Signal<bool>,
    /// Pin first column when scrolling horizontally
    #[prop(optional, into)]
    pin_cols: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the table element
    #[prop(optional)]
    node_ref: NodeRef<HtmlTable>,
    /// Table content (TableHead, TableBody, TableFoot)
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

/// A table header section component (`<thead>`).
///
/// The `TableHead` component wraps the header content of a table, typically containing
/// column headers and labels. It provides semantic structure and enables proper styling
/// and accessibility for table headers.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the thead element
/// - `children` - Header content (typically TableRow with TableHeader cells)
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableHead>
///                 <TableRow>
///                     <TableHeader>"Column 1"</TableHeader>
///                     <TableHeader>"Column 2"</TableHeader>
///                 </TableRow>
///             </TableHead>
///         </Table>
///     }
/// }
/// ```
#[component]
pub fn TableHead(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the thead element
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

/// A table body section component (`<tbody>`).
///
/// The `TableBody` component contains the main data content of a table.
/// It provides semantic structure and proper styling for table data rows.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the tbody element
/// - `children` - Body content (typically TableRow components with data)
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableBody>
///                 <TableRow>
///                     <TableCell>"Data 1"</TableCell>
///                     <TableCell>"Data 2"</TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     }
/// }
/// ```
#[component]
pub fn TableBody(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the tbody element
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

/// A table footer section component (`<tfoot>`).
///
/// The `TableFoot` component contains footer content for a table, typically
/// used for summaries, totals, or additional column information.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the tfoot element
/// - `children` - Footer content (typically TableRow components)
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableBody>
///                 // ... data rows
///             </TableBody>
///             <TableFoot>
///                 <TableRow>
///                     <TableCell>"Total"</TableCell>
///                     <TableCell>"$1,234"</TableCell>
///                 </TableRow>
///             </TableFoot>
///         </Table>
///     }
/// }
/// ```
#[component]
pub fn TableFoot(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the tfoot element
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

/// A table row component (`<tr>`).
///
/// The `TableRow` component represents a single row in a table, containing
/// either header cells (TableHeader) or data cells (TableCell).
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the tr element
/// - `children` - Row content (TableHeader or TableCell components)
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableBody>
///                 <TableRow>
///                     <TableCell>"John Doe"</TableCell>
///                     <TableCell>"Engineer"</TableCell>
///                     <TableCell>"$75,000"</TableCell>
///                 </TableRow>
///                 <TableRow class="bg-base-200">
///                     <TableCell>"Jane Smith"</TableCell>
///                     <TableCell>"Designer"</TableCell>
///                     <TableCell>"$65,000"</TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     }
/// }
/// ```
#[component]
pub fn TableRow(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the tr element
    #[prop(optional)]
    node_ref: NodeRef<Tr>,
    /// Row content (cells)
    children: Children,
) -> impl IntoView {
    view! {
        <tr node_ref=node_ref class=move || merge_classes!("", class)>
            {children()}
        </tr>
    }
}

/// A table header cell component (`<th>`).
///
/// The `TableHeader` component represents a header cell in a table, typically
/// used for column labels and descriptions. It provides semantic meaning and
/// appropriate styling for table headers.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the th element
/// - `children` - Header content (typically text or simple elements)
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableHead>
///                 <TableRow>
///                     <TableHeader>"Employee Name"</TableHeader>
///                     <TableHeader class="text-right">"Salary"</TableHeader>
///                     <TableHeader>"Department"</TableHeader>
///                 </TableRow>
///             </TableHead>
///         </Table>
///     }
/// }
/// ```
#[component]
pub fn TableHeader(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the th element
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

/// A table data cell component (`<td>`).
///
/// The `TableCell` component represents a data cell in a table, containing
/// the actual data content. It provides appropriate styling and structure
/// for table data.
///
/// # Props
///
/// - `class` - Optional additional CSS classes
/// - `node_ref` - Optional node reference for the td element
/// - `children` - Cell content (data, text, or other elements)
///
/// # Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Table>
///             <TableBody>
///                 <TableRow>
///                     <TableCell>"Product A"</TableCell>
///                     <TableCell class="text-right font-mono">"$99.99"</TableCell>
///                     <TableCell>
///                         <Badge color=BadgeColor::Success>"In Stock"</Badge>
///                     </TableCell>
///                 </TableRow>
///             </TableBody>
///         </Table>
///     }
/// }
/// ```
#[component]
pub fn TableCell(
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the td element
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
