use super::style::StackPlacement;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Stack Component
///
/// Visually stacks elements on top of each other with layered positioning.
/// Use Tailwind CSS classes to control dimensions and positioning.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("stack stack-top stack-bottom stack-start stack-end");
/// ```
///
/// ## Node References
/// - `node_ref` - References the stack `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Stack(
    /// Stack placement
    #[prop(optional, into)]
    placement: Signal<StackPlacement>,

    /// Additional CSS classes to apply to the stack container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the stack `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Elements to be stacked on top of each other
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("stack", placement.get().as_str(), class)
        >
            {children()}
        </div>
    }
}
