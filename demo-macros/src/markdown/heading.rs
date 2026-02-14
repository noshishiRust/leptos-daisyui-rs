use comrak::nodes::{AstNode, NodeHeading, NodeValue};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>, heading_node: &NodeHeading) -> TokenStream {
    let level = heading_node.level;
    let text_node = node.children().next().expect("Not Found Text in heading");
    let value = &text_node.data.borrow().value;
    let text = match value {
        NodeValue::Text(t) => t.clone(),
        _ => unimplemented!("Not Found Text"),
    };

    let h_id = generate_id(&text);
    let h_tag = format_ident!("h{level}");
    let h_class = heading_class(level);

    quote! {
        <#h_tag class=#h_class id=#h_id>
            #text
        </#h_tag>
    }
}

fn generate_id(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() || c == '-' {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
        .chars()
        .take(50)
        .collect()
}

fn heading_class(level: u8) -> String {
    match level {
        1 => "text-3xl font-bold".into(),
        2 => "text-xl font-semibold".into(),
        3 => "text-lg font-medium".into(),
        _ => {
            unimplemented!()
        }
    }
}
