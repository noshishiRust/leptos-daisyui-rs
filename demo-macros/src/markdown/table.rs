use comrak::nodes::AstNode;
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>) -> TokenStream {
    quote! {
        let children = node.children().collect::<Vec<_>>();
        let header_row = children.first();
        let body_rows = children.get(1..).unwrap_or(&[]);

        view! {
            <div class="overflow-x-auto my-4">
                <table class="table table-zebra">
                    {if let Some(header) = header_row {
                        let header_data = header.data.borrow();
                        if let NodeValue::TableRow(_) = &header_data.value {
                            let cells = header.children().collect::<Vec<_>>();
                            let cell_views: Vec<AnyView> = cells
                                .iter()
                                .map(|cell| {
                                    let cell_data = cell.data.borrow();
                                    if matches!(cell_data.value, NodeValue::TableCell { .. }) {
                                        let content = render_inline_nodes(
                                            cell.children().collect::<Vec<_>>().as_slice(),
                                            arena,
                                        );
                                        view! { <th class="px-4 py-2">{content}</th> }.into_any()
                                    } else {
                                        view! { <th></th> }.into_any()
                                    }
                                })
                                .collect();
                            Some(
                                view! {
                                    <thead>
                                        <tr>{cell_views}</tr>
                                    </thead>
                                }
                                    .into_any(),
                            )
                        } else {
                            None
                        }
                    } else {
                        None
                    }}
                    <tbody>
                        {body_rows
                            .iter()
                            .map(|row| {
                                let row_data = row.data.borrow();
                                if let NodeValue::TableRow(_) = &row_data.value {
                                    let cells = row.children().collect::<Vec<_>>();
                                    let cell_views: Vec<AnyView> = cells
                                        .iter()
                                        .map(|cell| {
                                            let cell_data = cell.data.borrow();
                                            if matches!(cell_data.value, NodeValue::TableCell { .. }) {
                                                let content = render_inline_nodes(
                                                    cell.children().collect::<Vec<_>>().as_slice(),
                                                    arena,
                                                );
                                                view! { <td class="px-4 py-2">{content}</td> }.into_any()
                                            } else {
                                                view! { <td></td> }.into_any()
                                            }
                                        })
                                        .collect();
                                    view! { <tr>{cell_views}</tr> }.into_any()
                                } else {
                                    view! { <tr></tr> }.into_any()
                                }
                            })
                            .collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        }
        .into_any()
    }
}
