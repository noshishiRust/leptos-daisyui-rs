//! # Table Node Handler
//!
//! This module handles markdown table nodes, converting them into daisyUI Table components.
//!
//! ## Purpose
//!
//! Tables are commonly used in documentation for:
//! - Props reference tables (name, type, default, description)
//! - Enum/variant listings
//! - Configuration options
//! - Event/callback signatures
//!
//! ## Component Structure
//!
//! Tables are rendered using daisyUI components:
//! - `<Table>` with `zebra=true` for alternating row colors
//! - `<TableHead>` containing the header row
//! - `<TableBody>` containing data rows
//! - `<TableRow>` for each table row
//! - `<TableHeader>` for header cells
//! - `<TableCell>` for data cells
//!
//! ## Cell Content
//!
//! Table cells support inline formatting:
//! - **Plain text**: Rendered as `<span>text</span>`
//! - **Inline code** (backticks): Rendered as `<code class="bg-base-300 px-1 py-0.5 rounded text-sm font-mono">code</code>`
//!
//! ## Limitations
//!
//! - **TODO**: Only text and inline code in cells are supported
//! - **TODO**: No support for links, emphasis, or other inline elements in cells
//! - **TODO**: No support for column alignment or table attributes
//! - **TODO**: No support for multiline cell content
//! - **TODO**: Empty cells render with empty tags (could use `&nbsp;` or placeholder text)

use comrak::nodes::{AstNode, NodeValue};
use proc_macro2::TokenStream;
use quote::quote;

/// Converts a markdown table AST node into Leptos view tokens.
///
/// Tables are rendered as daisyUI Table components with zebra striping and
/// proper semantic structure (head/body).
///
/// # Arguments
///
/// * `node` - The AST node containing the table and its rows
///
/// # Returns
///
/// A `TokenStream` containing a complete daisyUI Table component with:
/// - `zebra=true` attribute for alternating row colors
/// - Optional table head (if header row exists)
/// - Table body with all data rows
///
/// # Example
///
/// Given markdown:
/// ```markdown
/// | Prop  | Type  | Default |
/// | ------ | ------ | ------- |
/// | `name` | `str` | `""`     |
/// ```
///
/// Generates:
/// ```ignore
/// quote! {
///     <Table zebra=true>
///         <TableHead>
///             <TableRow>
///                 <TableHeader><span>Prop</span></TableHeader>
///                 <TableHeader><span>Type</span></TableHeader>
///                 <TableHeader><span>Default</span></TableHeader>
///             </TableRow>
///         </TableHead>
///         <TableBody>
///             <TableRow>
///                 <TableCell><span>name</span></TableCell>
///                 <TableCell><span>str</span></TableCell>
///                 <TableCell><span>""</span></TableCell>
///             </TableRow>
///         </TableBody>
///     </Table>
/// }
/// ```
///
/// # Panics
///
/// - Panics if table structure is invalid (missing rows/cells)
/// - **TODO**: Should provide better error messages for malformed tables
///
/// # Empty Tables
///
/// **TODO**: Handle edge cases:
/// - Tables with no rows
/// - Tables with only a header row
/// - Tables with missing cells in rows
pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>) -> TokenStream {
    // Collect all child rows ( TableRow nodes)
    let children = node.children().collect::<Vec<_>>();

    // First row is treated as header (GitHub/CommonMark convention)
    let header_row = children.first();

    // Remaining rows are body/content
    let body_rows = children.get(1..).unwrap_or(&[]);

    // Generate table head from first row (if it exists)
    let header_stream = match header_row {
        Some(header) => {
            let mut cells_stream = TokenStream::new();

            let header_data = header.data.borrow();

            if matches!(&header_data.value, NodeValue::TableRow(_)) {
                // Extract all cells from header row
                let cells = header.children().collect::<Vec<_>>();

                // Convert each cell to TableHeader component
                cells.into_iter().for_each(|cell| {
                    let cell_data = cell.data.borrow();

                    if matches!(cell_data.value, NodeValue::TableCell) {
                        // Get first child (should be text or code)
                        let cell_text_node = cell
                            .children()
                            .next()
                            .expect("Not Found Column Header name");
                        let cell_inline = inline_stream(&cell_text_node);
                        cells_stream.extend(quote! { <TableHeader>#cell_inline</TableHeader> });
                    } else {
                        // **TODO**: Handle non-table-cell nodes
                        // Currently renders empty TableHeader
                        cells_stream.extend(quote! { <TableHeader></TableHeader> });
                    }
                });
            } else {
                // **TODO**: First row is not a TableRow - should this be an error?
                unimplemented!("Not Found Table Header")
            }

            quote! {
                <TableHead>
                    <TableRow>
                        #cells_stream
                    </TableRow>
                </TableHead>
            }
        }
        None => quote! {}, // No header row, render empty
    };

    // Generate table body from remaining rows
    let mut rows_stream = TokenStream::new();
    body_rows.into_iter().for_each(|row| {
        let mut row_stream = TokenStream::new();

        let row_data = row.data.borrow();
        if matches!(&row_data.value, NodeValue::TableRow(_)) {
            // Extract all cells from this row
            let cells = row.children().collect::<Vec<_>>();

            // Convert each cell to TableCell component
            cells.into_iter().for_each(|cell| {
                let cell_data = cell.data.borrow();

                if matches!(cell_data.value, NodeValue::TableCell) {
                    // Get first child (should be text or code)
                    let cell_text_node = cell.children().next().expect("Not Found Column value");
                    let cell_inline = inline_stream(&cell_text_node);
                    row_stream.extend(quote! { <TableCell>#cell_inline</TableCell> });
                } else {
                    // **TODO**: Handle non-table-cell nodes
                    row_stream.extend(quote! { <TableCell></TableCell> });
                }
            });
        }

        rows_stream.extend(quote! { <TableRow>#row_stream</TableRow> });
    });

    // **TODO**: Handle empty body rows:
    // if body_rows.is_empty() {
    //     rows_stream.extend(quote! { <TableRow><TableCell>"No data"</TableCell></TableRow> });
    // }

    // Wrap body rows in TableBody component
    let body_stream = quote! {
        <TableBody>
            #rows_stream
        </TableBody>
    };

    // **TODO**: Consider adding configurable table options:
    // - zebra: bool (currently hardcoded to true)
    // - compact: bool for smaller padding
    // - border: bool for outer borders
    quote! {
        <Table zebra=true>
            #header_stream
            #body_stream
        </Table>
    }
}

/// Converts inline content within a table cell into view tokens.
///
/// Table cells can contain plain text or inline code elements. This function
/// handles both cases, applying appropriate styling.
///
/// # Arguments
///
/// * `node` - An AST node within a table cell (usually Text or Code)
///
/// # Returns
///
/// A `TokenStream` containing either:
/// - `<span>text</span>` for plain text
/// - `<code class="bg-base-300 px-1 py-0.5 rounded text-sm font-mono">code</code>` for inline code
/// - Empty `<span></span>` for unsupported node types
///
/// # Supported Inline Elements
///
/// - `NodeValue::Text`: Rendered as plain span
/// - `NodeValue::Code`: Rendered as styled code element
///
/// # Unsupported Inline Elements
///
/// **TODO**: The following are not supported but could be added:
///
/// ```ignore
/// NodeValue::Emph     // *italic* text
/// NodeValue::Strong    // **bold** text
/// NodeValue::Link      // [link text](url)
/// NodeValue::SoftBreak // Line breaks within cells
/// NodeValue::HardBreak // Hard breaks
/// ```
///
/// To add support, would need to recursively handle child nodes:
/// ```ignore
/// fn inline_stream_recursive(node: &AstNode) -> TokenStream {
///     match &node.data.borrow().value {
///         NodeValue::Text(text) => quote! { <span>#text</span> },
///         NodeValue::Code(code) => quote! { <code class="...">#code</code> },
///         NodeValue::Emph => {
///             let children = node.children().map(inline_stream_recursive).collect();
///             quote! { <em>#children</em> }
///         },
///         // ... other inline types
///     }
/// }
/// ```
fn inline_stream<'a>(node: &'a AstNode<'a>) -> TokenStream {
    match &node.data.borrow().value {
        // Plain text in table cell
        NodeValue::Text(text) => quote! { <span>#text</span> },

        // Inline code in table cell (e.g., `prop_name`)
        NodeValue::Code(code) => {
            let code_str = code.literal.clone();
            quote! { <code class="bg-base-300 px-1 py-0.5 rounded text-sm font-mono">#code_str</code> }
        }

        // **TODO**: Handle more inline elements in table cells
        // NodeValue::Emph(text) => quote! { <em>#text</em> },
        // NodeValue::Strong(text) => quote! { <strong>#text</strong> },
        // NodeValue::Link(link) => {
        //     let url = link.url.clone();
        //     let text = extract_text(link);
        //     quote! { <a href=#url>#text</a> }
        // },

        // Fallback for unsupported types (renders empty)
        _ => quote! { <span></span> },
    }
}
