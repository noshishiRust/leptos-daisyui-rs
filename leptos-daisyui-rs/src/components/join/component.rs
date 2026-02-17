use super::style::JoinDirection;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Join Component
///
/// A container for grouping multiple items with connected borders. Items are
/// visually joined together with shared border radius applied to first and last items.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("join join-item join-vertical join-horizontal");
/// ```
///
/// ## Node References
/// - `node_ref` - References the join `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Join(
    /// Direction of the join layout (horizontal by default)
    #[prop(optional, into)]
    direction: Signal<JoinDirection>,

    /// Additional CSS classes to apply to the join container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the join `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to be joined together (buttons, inputs, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "join",
                direction.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}
