use super::style::ToastPosition;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Toast Container Component
///
/// A wrapper for stacking notification elements positioned at the corners of the page.
/// Use for displaying temporary messages, alerts, or notifications.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("toast toast-start toast-center toast-end toast-top toast-middle toast-bottom");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Toast(
    /// Position of the toast container on the page
    #[prop(optional, into)]
    position: Signal<ToastPosition>,

    /// Additional CSS classes to apply to the toast container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the top `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Toast notification content (alerts, messages, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "toast",
                    position.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}
