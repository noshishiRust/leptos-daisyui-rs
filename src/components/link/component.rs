use super::style::LinkColor;
use crate::merge_classes;
use leptos::{html::A, prelude::*};

/// # Link Component
///
/// A reactive Leptos wrapper for daisyUI's link component that provides styled
/// anchor elements with customizable colors and hover effects.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("link link-neutral link-primary link-secondary link-accent link-success link-info link-warning link-error link-hover");
/// ```
///
/// ## Node References
/// - `node_ref` - References the anchor element ([HTMLAnchorElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement))
#[component]
pub fn Link(
    /// Color scheme of the link
    #[prop(optional, into)]
    color: Signal<LinkColor>,

    /// Whether to show hover effect
    #[prop(optional, into)]
    hover: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the anchor element
    #[prop(optional)]
    node_ref: NodeRef<A>,

    /// URL that the link points to
    #[prop(optional)]
    href: Option<&'static str>,

    /// Text content of the link
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
