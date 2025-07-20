use super::style::LinkColor;
use crate::merge_classes;
use leptos::{html::A, prelude::*};

#[component]
pub fn Link(
    #[prop(optional, into)] color: Signal<LinkColor>,
    #[prop(optional, into)] hover: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<A>,
    #[prop(optional)] href: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            node_ref=node_ref
            href=href.unwrap_or("#")
            class=move || {
                merge_classes!("link",
                color.get().as_str(),
                class)
            }
            class:link-hover=hover
        >
            {children()}
        </a>
    }
}
