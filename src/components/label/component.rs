/// # Label Component
///
/// A reactive Leptos wrapper for daisyUI's label component that provides accessible
/// form labeling with consistent styling and layout.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("label label-text label-text-alt");
/// ```
///
/// ## Node References
/// - `node_ref` - References the label element ([HTMLLabelElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement))

use crate::merge_classes;
use leptos::{
    html::{Label as HtmlLabel, Span},
    prelude::*,
};

#[component]
pub fn Label(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the label element
    #[prop(optional)] node_ref: NodeRef<HtmlLabel>,
    /// Child elements of the label
    children: Children,
) -> impl IntoView {
    view! {
        <label node_ref=node_ref class=move || merge_classes!("label", class)>
            {children()}
        </label>
    }
}

/// # Label Text Component
///
/// A reactive Leptos wrapper for daisyUI's label text component that provides
/// primary text content within label containers.
///
/// ## Node References
/// - `node_ref` - References the span element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))

#[component]
pub fn LabelText(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the span element
    #[prop(optional)] node_ref: NodeRef<Span>,
    /// Text content of the label
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("label-text", class)>
            {children()}
        </span>
    }
}

/// # Label Text Alt Component
///
/// A reactive Leptos wrapper for daisyUI's label text alt component that provides
/// secondary or alternative text content within label containers.
///
/// ## Node References
/// - `node_ref` - References the span element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))

#[component]
pub fn LabelTextAlt(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the span element
    #[prop(optional)] node_ref: NodeRef<Span>,
    /// Alternative text content of the label
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("label-text-alt", class)>
            {children()}
        </span>
    }
}
