use comrak::nodes::{AstNode, NodeValue};
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>) -> TokenStream {
    let text_node = node.children().next().expect("Not Found Text in paragraph");
    let value = &text_node.data.borrow().value;
    let text = match value {
        NodeValue::Text(t) => t.clone(),
        _ => unimplemented!("Not Found Text"),
    };

    quote! {
        <p class="text-base-content/70">
            #text
        </p>
    }
}
