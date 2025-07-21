/// # Mockup Code Component
///
/// A reactive Leptos wrapper for daisyUI's mockup code component that provides
/// a styled terminal or code editor frame for displaying code content.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("mockup-code");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))

use crate::merge_classes;
use leptos::{
    html::{Div, Pre},
    prelude::*,
};

#[component]
pub fn MockupCode(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the div element
    #[prop(optional)] node_ref: NodeRef<Div>,
    /// Code lines displayed within the mockup
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("mockup-code", class)>
            {children()}
        </div>
    }
}

/// # Mockup Code Line Component
///
/// A reactive Leptos wrapper for individual code lines within a MockupCode component
/// that provides styled terminal or code lines with customizable prefixes.
///
/// ## Node References
/// - `node_ref` - References the pre element ([HTMLPreElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement))

#[component]
pub fn MockupCodeLine(
    /// Prefix character for the code line (defaults to "$")
    #[prop(optional)] prefix: Option<&'static str>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the pre element
    #[prop(optional)] node_ref: NodeRef<Pre>,
    /// Code content of the line
    children: Children,
) -> impl IntoView {
    view! {
        <pre node_ref=node_ref class=class data-prefix=prefix.unwrap_or("$")>
            {children()}
        </pre>
    }
}
