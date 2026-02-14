use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};
use leptos::prelude::*;

/// Render a Markdown string to Leptos components
pub fn render_markdown<'a, T: IntoView + 'static + Clone>(
    md: &'a str,
    examples: &'a HashMap<String, T>,
) -> impl IntoView + use<T> {
    let arena = Arena::new();
    let mut option = Options::default();
    option.extension.table = true;
    let root = parse_document(&arena, md, &option);

    view! { {render_nodes(root.children().collect::<Vec<_>>().as_slice(), &arena, examples)} }
        .into_any()
}

fn render_nodes<'a, T: IntoView + 'static + Clone>(
    nodes: &[&'a AstNode<'a>],
    arena: &'a Arena<AstNode<'a>>,
    examples: &'a HashMap<String, T>,
) -> Vec<impl IntoView + use<T>> {
    nodes
        .iter()
        .map(|&node| render_node(node, arena, examples))
        .collect()
}

fn render_node<'a, T: IntoView + 'static + Clone>(
    node: &'a AstNode<'a>,
    arena: &'a Arena<AstNode<'a>>,
    examples: &'a HashMap<String, T>,
) -> impl IntoView + use<T> {
    match &node.data.borrow().value {
        NodeValue::Heading(heading) => {
            let level = heading.level;
            let content_text = extract_text(node);
            let id = generate_id(&content_text);

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
            .into_any()
        }

        NodeValue::Paragraph => view! {
            <p class="my-4 text-base-content/80 leading-relaxed">
                {render_inline_nodes(node.children().collect::<Vec<_>>().as_slice(), arena)}
            </p>
        }
        .into_any(),

        NodeValue::CodeBlock(code_block) => {
            log::debug!("examples {:?}", examples.keys());

            let key = code_block.literal.trim();

            let demo = examples.get(key).cloned();
            log::debug!("is_some(), {}", demo.is_some());

            view! { {demo.map(|x| x)} }.into_any()
        }

        NodeValue::Table(_) => {
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

        NodeValue::Link(link) => {
            let url = link.url.clone();
            view! {
                <a href=url class="link link-hover">
                    {render_inline_nodes(node.children().collect::<Vec<_>>().as_slice(), arena)}
                </a>
            }
            .into_any()
        }

        _ => view! { <div>{"unimplemented type"}</div> }.into_any(),
    }
}

fn render_inline_nodes<'a, 'b>(
    nodes: &'b [&'a AstNode<'a>],
    _arena: &'a Arena<AstNode<'a>>,
) -> Vec<AnyView> {
    nodes
        .iter()
        .map(|&node| render_inline_node(node, _arena))
        .collect()
}

fn render_inline_node<'a>(node: &'a AstNode<'a>, _arena: &'a Arena<AstNode<'a>>) -> AnyView {
    match &node.data.borrow().value {
        NodeValue::Text(text) => view! { <span>{text.clone()}</span> }.into_any(),
        NodeValue::Code(code) => {
            let code_str = code.literal.clone();
            view! { <code class="bg-base-300 px-1 py-0.5 rounded text-sm font-mono">{code_str}</code> }
            .into_any()
        }
        NodeValue::SoftBreak => view! { <span>" "</span> }.into_any(),
        NodeValue::LineBreak => view! { <br /> }.into_any(),
        NodeValue::Strong => {
            let content = node.children().collect::<Vec<_>>();
            view! {
                <strong class="font-semibold">
                    {content.iter().map(|n| render_inline_node(n, _arena)).collect_view()}
                </strong>
            }
            .into_any()
        }
        NodeValue::Emph => {
            let content = node.children().collect::<Vec<_>>();
            view! {
                <em class="italic">
                    {content.iter().map(|n| render_inline_node(n, _arena)).collect_view()}
                </em>
            }
            .into_any()
        }
        NodeValue::Link(link) => {
            let url = link.url.clone();
            let content = node.children().collect::<Vec<_>>();
            view! {
                <a href=url class="link link-hover">
                    {content.iter().map(|n| render_inline_node(n, _arena)).collect_view()}
                </a>
            }
            .into_any()
        }
        _ => view! { <span></span> }.into_any(),
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

use std::collections::HashMap;

#[component]
pub fn MarkdownRenderer<T: IntoView + 'static + Clone>(
    content: String,
    examples: HashMap<String, T>,
) -> impl IntoView {
    // Convert to owned values to avoid lifetime issues
    let content_owned = content;
    let examples_owned = examples;
    render_markdown(&content_owned, &examples_owned)
}
