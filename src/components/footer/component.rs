/// # Footer Component
///
/// A reactive Leptos wrapper for daisyUI's footer component that provides structured
/// page footers with customizable placement and layout direction.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("footer footer-center footer-horizontal footer-vertical footer-title");
/// ```
///
/// ## Node References
/// - `node_ref` - References the footer element ([HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement))

use super::style::{FooterDirection, FooterPlacement};
use crate::merge_classes;
use leptos::{
    html::{Div, Footer as HtmlFooter},
    prelude::*,
};

#[component]
pub fn Footer(
    /// Placement alignment of footer content
    #[prop(optional, into)] placement: Signal<FooterPlacement>,
    /// Layout direction of footer items
    #[prop(optional, into)] direction: Signal<FooterDirection>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the footer element
    #[prop(optional)] node_ref: NodeRef<HtmlFooter>,
    /// Child elements of the footer
    children: Children,
) -> impl IntoView {
    view! {
        <footer
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "footer",
                placement.get().as_str(),
                direction.get().as_str(),
                class
                )
            }
        >
            {children()}
        </footer>
    }
}

/// # Footer Title Component
///
/// A reactive Leptos wrapper for daisyUI's footer title component that provides
/// styled title sections within footer containers.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))

#[component]
pub fn FooterTitle(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the div element
    #[prop(optional)] node_ref: NodeRef<Div>,
    /// Title content of the footer section
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("footer-title", class)>
            {children()}
        </div>
    }
}
