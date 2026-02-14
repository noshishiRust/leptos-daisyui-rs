mod code_block;
mod heading;
mod paragraph;
mod table;

use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};
use proc_macro2::TokenStream;
use quote::quote;

pub fn markdown_to_token_stream(markdown: &str) -> TokenStream {
    let mut view_token_stream = TokenStream::new();

    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;

    let root_node = parse_document(&arena, markdown, &options);

    root_node.children().into_iter().for_each(|node| {
        let token = match &node.data.borrow().value {
            NodeValue::Heading(h) => heading::token_stream_for_view(root_node, h),
            NodeValue::Paragraph => paragraph::token_stream_for_view(root_node),
            NodeValue::CodeBlock(c) => code_block::token_stream_for_view(root_node, c),
            NodeValue::Table(t) => table::token_stream_for_view(root_node),
            _ => {
                quote! {}
            }
        };

        view_token_stream.extend(token);
    });

    view_token_stream
}
