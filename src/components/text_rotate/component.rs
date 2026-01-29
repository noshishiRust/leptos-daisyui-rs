use crate::merge_classes;
use leptos::{html::Span, prelude::*};

/// # Text Rotate Component
///
/// A reactive Leptos wrapper for daisyUI's text-rotate component that displays up to 6 lines
/// of text, one at a time, with an infinite loop animation. Animation pauses on hover.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("text-rotate");
/// ```
///
/// ## Usage
/// The component requires a three-level nesting structure:
/// - Outer span: The TextRotate component itself
/// - Middle span: TextRotateContainer for the rotating items
/// - Inner spans: TextRotateItem components (up to 6)
///
/// ## Node References
/// - `node_ref` - References the outer `<span>` element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn TextRotate(
    /// Additional CSS classes for the outer span
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the outer span element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Container and rotating items (use TextRotateContainer with TextRotateItem children)
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("text-rotate", class)>
            {children()}
        </span>
    }
}

/// # Text Rotate Container Component
///
/// The middle container that holds the rotating text items. Place TextRotateItem components
/// as children of this container.
///
/// ## Node References
/// - `node_ref` - References the container `<span>` element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn TextRotateContainer(
    /// Additional CSS classes for the container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container span element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Text items to rotate through (use TextRotateItem components)
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=class>
            {children()}
        </span>
    }
}

/// # Text Rotate Item Component
///
/// An individual text item in the rotation sequence. Up to 6 items are supported.
///
/// ## Node References
/// - `node_ref` - References the item `<span>` element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn TextRotateItem(
    /// Additional CSS classes for the item
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the item span element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Text content or other elements
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("text-rotate-item", class)>
            {children()}
        </span>
    }
}
