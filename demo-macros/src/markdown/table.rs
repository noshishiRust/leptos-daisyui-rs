use comrak::nodes::{AstNode, NodeValue};
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>) -> TokenStream {
    let children = node.children().collect::<Vec<_>>();
    let header_row = children.first();
    let body_rows = children.get(1..).unwrap_or(&[]);

    let header_stream = match header_row {
        Some(header) => {
            let mut cells_stream = TokenStream::new();

            let header_data = header.data.borrow();

            if matches!(&header_data.value, NodeValue::TableRow(_)) {
                let cells = header.children().collect::<Vec<_>>();

                cells.into_iter().for_each(|cell| {
                    let cell_data = cell.data.borrow();

                    if matches!(cell_data.value, NodeValue::TableCell) {
                        let cell_text_node = cell
                            .children()
                            .next()
                            .expect("Not Found Column Header name");
                        let cell_inline = inline_stream(&cell_text_node);
                        cells_stream.extend(quote! { <TableHeader>#cell_inline</TableHeader> });
                    } else {
                        cells_stream.extend(quote! { <TableHeader></TableHeader> });
                    }
                });
            } else {
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
        None => quote! {},
    };

    let mut rows_stream = TokenStream::new();
    body_rows.into_iter().for_each(|row| {
        let mut row_stream = TokenStream::new();

        let row_data = row.data.borrow();
        if matches!(&row_data.value, NodeValue::TableRow(_)) {
            let cells = row.children().collect::<Vec<_>>();

            cells.into_iter().for_each(|cell| {
                let cell_data = cell.data.borrow();

                if matches!(cell_data.value, NodeValue::TableCell) {
                    let cell_text_node = cell.children().next().expect("Not Found Column value");
                    let cell_inline = inline_stream(&cell_text_node);
                    row_stream.extend(quote! { <TableCell>#cell_inline</TableCell> });
                } else {
                    row_stream.extend(quote! { <TableCell></TableCell> });
                }
            });
        }

        rows_stream.extend(quote! { <TableRow>#row_stream</TableRow> });
    });

    let body_stream = quote! {
        <TableBody>
            #rows_stream
        </TableBody>
    };

    quote! {
        <Table zebra=true>
            #header_stream
            #body_stream
        </Table>
    }
}

fn inline_stream<'a>(node: &'a AstNode<'a>) -> TokenStream {
    match &node.data.borrow().value {
        NodeValue::Text(text) => quote! { <span>#text</span> },
        NodeValue::Code(code) => {
            let code_str = code.literal.clone();
            quote! { <code class="bg-base-300 px-1 py-0.5 rounded text-sm font-mono">#code_str</code> }
        }
        _ => quote! { <span></span> },
    }
}
