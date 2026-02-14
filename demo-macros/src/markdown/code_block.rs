use comrak::nodes::{AstNode, NodeCodeBlock};
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(
    _node: &'a AstNode<'a>,
    code_block_node: &NodeCodeBlock,
) -> TokenStream {
    let _info = code_block_node.info.clone();
    let code = code_block_node.literal.clone();

    let code_raw_stream = code.parse::<TokenStream>().expect("Failed to covert stream from string");

    quote! {
        <div>
            #code_raw_stream
        </div>
    }
}
