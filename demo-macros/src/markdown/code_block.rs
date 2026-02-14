use comrak::nodes::{AstNode, NodeCodeBlock};
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(
    node: &'a AstNode<'a>,
    code_block_node: &NodeCodeBlock,
) -> TokenStream {
    quote! {
            let key = code_block.literal.trim();

            let demo = examples.get(key).cloned();
            view! { {demo.map(|x| x)} }.into_any()
    }
}
