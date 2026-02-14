use comrak::nodes::AstNode;
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>) -> TokenStream {
    quote! {
                <p class="my-4 text-base-content/80 leading-relaxed">
                    {render_inline_nodes(node.children().collect::<Vec<_>>().as_slice(), arena)}
                </p>

        }
}
