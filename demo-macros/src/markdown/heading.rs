use comrak::nodes::{AstNode, NodeHeading, NodeValue};
use proc_macro2::TokenStream;
use quote::quote;

pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>, heading_node: &NodeHeading) -> TokenStream {
    let level = heading_node.level;
    let content_text = extract_text(node);
    let id = generate_id(&content_text);

    quote! {

            view! {
                <div class="heading-wrapper mt-8 mb-4 scroll-mt-24">
                    {match level {
                        1 => {
                            view! {
                                <h1 id=id class="text-4xl font-bold flex items-center gap-2">
                                    <span>{content_text.clone()}</span>
                                    <a
                                        href=format!("#{id}")
                                        class="text-base-content/30 text-xl hover:text-primary transition-colors ml-2"
                                    >
                                        "#"
                                    </a>
                                </h1>
                            }
                                .into_any()
                        }
                        2 => {
                            view! {
                                <h2 id=id class="text-3xl font-bold flex items-center gap-2">
                                    <span>{content_text.clone()}</span>
                                    <a
                                        href=format!("#{id}")
                                        class="text-base-content/30 text-xl hover:text-primary transition-colors ml-2"
                                    >
                                        "#"
                                    </a>
                                </h2>
                            }
                                .into_any()
                        }
                        3 => {
                            view! {
                                <h3 id=id class="text-2xl font-bold flex items-center gap-2">
                                    <span>{content_text.clone()}</span>
                                    <a
                                        href=format!("#{id}")
                                        class="text-base-content/30 text-xl hover:text-primary transition-colors ml-2"
                                    >
                                        "#"
                                    </a>
                                </h3>
                            }
                                .into_any()
                        }
                        _ => {
                            view! {
                                <h4 id=id class="text-xl font-bold flex items-center gap-2">
                                    <span>{content_text.clone()}</span>
                                    <a
                                        href=format!("#{id}")
                                        class="text-base-content/30 text-xl hover:text-primary transition-colors ml-2"
                                    >
                                        "#"
                                    </a>
                                </h4>
                            }
                                .into_any()
                        }
                    }}
                </div>
            }
    }
}

fn extract_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    extract_text_recursive(node, &mut text);
    text
}

fn extract_text_recursive<'a>(node: &'a AstNode<'a>, text: &mut String) {
    match &node.data.borrow().value {
        NodeValue::Text(t) => text.push_str(t),
        NodeValue::Code(c) => text.push_str(&c.literal),
        NodeValue::SoftBreak => text.push(' '),
        NodeValue::LineBreak => text.push(' '),
        _ => {
            let children: Vec<&AstNode<'_>> = node.children().collect();
            for child in children {
                extract_text_recursive(child, text);
            }
        }
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
